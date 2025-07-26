// 릴리즈 환경에서 콘솔 나오지 않도록 막기
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hagin_gwichanko::system::core::App;
use hagin_gwichanko::system::single_instance::is_single_instance;
use hagin_gwichanko::system::AppEvent;
use hagin_gwichanko::{contexted_err, show_error_with_terminate, SResult};

fn main() {
    if let Err((msg, err)) = run_main() {
        show_error_with_terminate(&msg, &err);
    }
}

fn run_main() -> SResult<()> {
    if !is_single_instance()? {
        return Ok(());
    }

    let event_loop = winit::event_loop::EventLoop::<AppEvent>::with_user_event()
        .build()
        .or_else(|e| contexted_err!("failed create windows event loop", e))?;

    let proxy = event_loop.create_proxy();
    tray_icon::menu::MenuEvent::set_event_handler(Some(move |event| {
        if let Err(e) = proxy.send_event(AppEvent::SystemTrayMenuEvent(event)) {
            eprintln!("failed send tray menu proxy event : {:?}", e.to_string());
        }
    }));

    let proxy = event_loop.create_proxy();
    tray_icon::TrayIconEvent::set_event_handler(Some(move |event| {
        if let Err(e) = proxy.send_event(AppEvent::SystemTrayIconEvent(event)) {
            eprintln!("failed send tray icon proxy event : {:?}", e.to_string());
        }
    }));

    let mut app = App::default();
    event_loop
        .run_app(&mut app)
        .or_else(|e| contexted_err!("failed start app", e))
}
