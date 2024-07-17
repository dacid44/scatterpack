use anyhow::Context;
use packing_list::{
    thumbnail::Thumbnails, ListItem, PackCollection, PackItem, PackingList, UniqueItem,
};
use store::Store;
use tauri::{async_runtime::Mutex, Manager};
use tauri_plugin_dialog::DialogExt;

mod packing_list;
mod platform;
mod store;

struct AppState {
    packing_lists: Mutex<Vec<PackingList>>,
    unique_items: Mutex<Vec<UniqueItem>>,
    thumbnails: Thumbnails,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            store::init_store(app.handle().clone()).expect("failed to initialize store");

            let packing_lists = Vec::<PackingList>::load(app.handle().clone())
                .unwrap_or_else(|_| vec![example_packing_list()]);

            let unique_items = Vec::<UniqueItem>::load(app.handle().clone())
                .unwrap_or_else(|_| example_unique_items());

            app.manage(AppState {
                packing_lists: Mutex::new(packing_lists),
                unique_items: Mutex::new(unique_items),
                thumbnails: Thumbnails::new(app.handle().clone())
                    .expect("failed to initialize thumbnail store"),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            example_packing_list,
            save_packing_list,
            load_packing_lists,
            pick_file,
            pick_thumbnail,
            save_base64_thumbnail,
            get_thumbnail_path,
            load_unique_items,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn example_packing_list() -> PackingList {
    PackingList::new(
        "example",
        vec![
            ListItem::Item(PackItem::new("USB Charger", "near bed", 2, Vec::new())),
            ListItem::Collection(PackCollection::new(
                "Clothes",
                vec![
                    ListItem::Item(PackItem::new("T-shirt", "packed", 5, Vec::new())),
                    ListItem::Item(PackItem::new("Socks", "laundry", 6, Vec::new())),
                    ListItem::Item(PackItem::new("Hat", "wearing", 1, Vec::new())),
                ],
            )),
            ListItem::Item(PackItem::new("Razor", "in bathroom", 1, Vec::new())),
        ],
    )
}

#[tauri::command]
async fn save_packing_list(packing_list: PackingList, app: tauri::AppHandle) -> tauri::Result<()> {
    let state = app.state::<AppState>();
    let mut packing_lists = state.packing_lists.lock().await;

    if let Some(list) = packing_lists
        .iter_mut()
        .find(|list| list.name == packing_list.name)
    {
        *list = packing_list;
    } else {
        packing_lists.push(packing_list);
    }

    packing_lists.save(app.clone())?;
    Ok(())
}

#[tauri::command]
async fn load_packing_lists(app: tauri::AppHandle) -> Vec<PackingList> {
    let state = app.state::<AppState>();
    let packing_lists = state.packing_lists.lock().await;
    packing_lists.clone()
}

#[tauri::command]
async fn pick_file(app: tauri::AppHandle) -> Result<String, String> {
    let path = app
        .dialog()
        .file()
        .blocking_pick_file()
        .ok_or("no file picked")?
        .path;
    let thumbnails = &app.state::<AppState>().thumbnails;
    let name = thumbnails
        .copy_from_path(path, "test")
        .map_err(|err| format!("error copying file: {err:?}"))?;
    Ok(thumbnails
        .get_full_path(&name)
        .to_string_lossy()
        .into_owned())
}

#[tauri::command]
async fn pick_thumbnail(name: String, app: tauri::AppHandle) -> Result<String, String> {
    let path = app
        .dialog()
        .file()
        .add_filter("image", &["png", "jpg", "jpeg", "gif", "webp"])
        .set_title("Choose a thumbnail image")
        .blocking_pick_file()
        .ok_or("no file picked")?
        .path;
    let thumbnails = &app.state::<AppState>().thumbnails;
    let name = thumbnails
        .copy_from_path(path, &name)
        .map_err(|err| format!("error copying file: {err:?}"))?;
    Ok(name)
}

#[tauri::command]
async fn save_base64_thumbnail(
    data: &str,
    name: String,
    filename: String,
    app: tauri::AppHandle,
) -> tauri::Result<String> {
    let thumbnails = &app.state::<AppState>().thumbnails;
    let name = thumbnails
        .save_from_base64(data, &name, filename)
        .context("Failed to save base64 thumbnail")?;
    Ok(name)
}

#[tauri::command]
fn get_thumbnail_path(name: String, app: tauri::AppHandle) -> String {
    app.state::<AppState>()
        .thumbnails
        .get_full_path(&name)
        .to_string_lossy()
        .into_owned()
}

fn example_unique_items() -> Vec<UniqueItem> {
    vec![
        UniqueItem::new(
            "Choir DC shirt",
            "gray shirt with cherry blossom design",
            "packed",
            None,
        ),
        UniqueItem::new(
            "Venturing pants",
            "green activewear zip-off pants",
            "in dresser",
            None,
        ),
        UniqueItem::new("Black socks", "", "wearing", None),
        UniqueItem::new(
            "Ghibli shirt",
            "gray shirt with Studio Ghibli stuff on it",
            "wearing",
            None,
        ),
        UniqueItem::new(
            "Dark gray shorts",
            "semi-formal dark gray shorts",
            "in dresser",
            None,
        ),
    ]
}

#[tauri::command]
async fn load_unique_items(app: tauri::AppHandle) -> Vec<UniqueItem> {
    let state = app.state::<AppState>();
    let unique_items = state.unique_items.lock().await;
    unique_items.clone()
}
