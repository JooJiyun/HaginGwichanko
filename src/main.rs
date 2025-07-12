// 릴리즈 환경에서 콘솔 나오지 않도록 막기
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use routine_it::{
    contexted_err, show_error_with_terminate,
    system::{self, single_instance::is_single_instance},
    SResult,
};

fn main() {
    if let Err((msg, err)) = run_main() {
        show_error_with_terminate(&msg, &err);
    }
}

fn run_main() -> SResult<()> {
    if !is_single_instance()? {
        return Ok(());
    }

    let event_loop = winit::event_loop::EventLoop::<system::SystemEvent>::with_user_event()
        .build()
        .or_else(|e| contexted_err!("failed create windows event loop", e))?;

    let proxy = event_loop.create_proxy();
    tray_icon::menu::MenuEvent::set_event_handler(Some(move |event| {
        if let Err(e) = proxy.send_event(system::SystemEvent::SystemTrayEvent(event)) {
            eprintln!("failed proxy send event : {:?}", e.to_string());
        }
    }));

    let mut app = system::System::default();
    event_loop
        .run_app(&mut app)
        .or_else(|e| contexted_err!("failed start app", e))
}
