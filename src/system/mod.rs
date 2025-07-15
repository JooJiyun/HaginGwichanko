pub mod core;
pub mod data;
pub mod single_instance;
mod system_tray;
mod ui_context;
pub mod view_func;

pub enum AppEvent {
    SystemTrayEvent(tray_icon::menu::MenuEvent),
}

#[derive(Debug, Clone)]
pub enum UIEvent {
    ButtonPressed,
}

pub struct TerminateThreadEvent;
