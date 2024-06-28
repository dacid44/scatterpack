use packing_list::{PackItem, PackingList};
use tauri_plugin_store::StoreBuilder;

mod packing_list;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let mut store = StoreBuilder::new("packing_lists.bin").build(app.handle().clone());
            let _ = store.load();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![packing_list, save_packing_list, load_packing_lists])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn packing_list() -> Vec<PackItem> {
    vec![
        PackItem::new("shirt", "bag", 3),
        PackItem::new("socks", "washing machine", 5),
        PackItem::new("pants", "wearing", 1),
    ]
}

#[tauri::command]
fn save_packing_list(
    packing_list: PackingList,
    app: tauri::AppHandle,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut store = StoreBuilder::new("packing_lists.bin").build(app.clone());
    store.insert(
        packing_list.name.clone(),
        serde_json::to_value(packing_list)?,
    ).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

#[tauri::command]
fn load_packing_lists(
    app: tauri::AppHandle,
) -> Result<Vec<PackingList>, Box<dyn std::error::Error>> {
    let mut store = StoreBuilder::new("packing_lists.bin").build(app.clone());
    store.load()?;
    store
        .values()
        .map(|val| {
            serde_json::from_value(val.clone())
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        })
        .collect()
}
