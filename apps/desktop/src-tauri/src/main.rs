mod commands;
mod menu;
use futures::executor::block_on;
use sdcorelib;
use tauri::api::path;

fn main() {
  let data_dir = path::data_dir().unwrap_or(std::path::PathBuf::from("./"));
  block_on(sdcorelib::configure(data_dir));

  tauri::Builder::default()
    .setup(|_app| Ok(()))
    .on_menu_event(|event| menu::handle_menu_event(event))
    .invoke_handler(tauri::generate_handler![
      commands::scan_dir,
      commands::get_files,
      commands::get_config,
      commands::get_mounts,
      commands::test_scan,
    ])
    .menu(menu::get_menu())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
