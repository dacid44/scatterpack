use packing_list::{thumbnail::Thumbnails, ListItem, PackCollection, PackItem, PackingList};
use tauri::{async_runtime::Mutex, Manager, Wry};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_store::{Store, StoreBuilder};

mod packing_list;
mod platform;

struct AppState {
    packing_lists_store: Mutex<Store<Wry>>,
    unique_items_store: Mutex<Store<Wry>>,
    thumbnails: Thumbnails,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let mut packing_lists_store =
                StoreBuilder::new("packing_lists.bin").build(app.handle().clone());
            if packing_lists_store.load().is_err() {
                packing_lists_store
                    .insert(
                        "example".to_string(),
                        serde_json::to_value(example_packing_list()).map_err(print_error)?,
                    )
                    .map_err(print_error)?;
            }

            let mut unique_items_store =
                StoreBuilder::new("unique_items.bin").build(app.handle().clone());

            let _ = unique_items_store.load();

            app.manage(AppState {
                packing_lists_store: Mutex::new(packing_lists_store),
                unique_items_store: Mutex::new(unique_items_store),
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
            get_thumbnail_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn print_error(e: impl std::error::Error) -> String {
    format!("{e:?}")
}

#[tauri::command]
fn example_packing_list() -> PackingList {
    PackingList::new(
        "example",
        vec![
            ListItem::Item(PackItem::new("USB Charger", "near bed", 2)),
            ListItem::Collection(PackCollection::new(
                "Clothes",
                vec![
                    ListItem::Item(PackItem::new("T-shirt", "packed", 5)),
                    ListItem::Item(PackItem::new("Socks", "laundry", 6)),
                    ListItem::Item(PackItem::new("Hat", "wearing", 1)),
                ],
            )),
            ListItem::Item(PackItem::new("Razor", "in bathroom", 1)),
        ],
    )
}

#[tauri::command]
async fn save_packing_list(packing_list: PackingList, app: tauri::AppHandle) -> Result<(), String> {
    let state = app.state::<AppState>();
    let mut store = state.packing_lists_store.lock().await;
    store
        .insert(
            packing_list.name.clone(),
            serde_json::to_value(packing_list).map_err(print_error)?,
        )
        .map_err(print_error)?;
    store.save().map_err(print_error)
}

#[tauri::command]
async fn load_packing_lists(app: tauri::AppHandle) -> Result<Vec<PackingList>, String> {
    let state = app.state::<AppState>();
    let store = state.packing_lists_store.lock().await;
    store
        .values()
        .map(|val| serde_json::from_value(val.clone()).map_err(print_error))
        .collect()
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
        .copy_from(path, "test")
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
        .copy_from(path, &name)
        .map_err(|err| format!("error copying file: {err:?}"))?;
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
