use packing_list::{ListItem, PackCollection, PackItem, PackingList};
use tauri::{async_runtime::Mutex, Manager, Wry};
use tauri_plugin_store::{Store, StoreBuilder};

mod packing_list;

struct AppState {
    packing_lists_store: Mutex<Store<Wry>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let mut store = StoreBuilder::new("packing_lists.bin").build(app.handle().clone());
            if store.load().is_err() {
                store
                    .insert(
                        "example".to_string(),
                        serde_json::to_value(example_packing_list()).map_err(print_error)?,
                    )
                    .map_err(print_error)?;
            }
            app.manage(AppState {
                packing_lists_store: Mutex::new(store),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            example_packing_list,
            save_packing_list,
            load_packing_lists
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn print_error(e: impl std::error::Error) -> String {
    format!("{e:?}")
}

#[tauri::command]
fn example_packing_list() -> PackingList {
    PackingList::new("example", vec![
        ListItem::Item(PackItem::new("USB Charger", "near bed", 2)),
        ListItem::Collection(PackCollection::new("Clothes", vec![
            ListItem::Item(PackItem::new("T-shirt", "packed", 5)),
            ListItem::Item(PackItem::new("Socks", "laundry", 6)),
            ListItem::Item(PackItem::new("Hat", "wearing", 1)),
        ])),
        ListItem::Item(PackItem::new("Razor", "in bathroom", 1)),
    ])
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
    let mut store = state.packing_lists_store.lock().await;
    store
        .values()
        .map(|val| serde_json::from_value(dbg!(val).clone()).map_err(print_error))
        .collect()
}
