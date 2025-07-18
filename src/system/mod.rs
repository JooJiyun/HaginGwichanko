pub mod core;
pub mod data;
pub mod routine;
pub mod single_instance;
pub mod tray;
pub mod ui;
pub mod view;

pub enum AppEvent {
    SystemTrayEvent(tray_icon::menu::MenuEvent),
}

#[derive(Debug, Clone)]
pub enum UIEvent {
    OpenWidgetScene(WidgetScene),

    RoutineChanged(RoutineChangeEvent, usize),
    UpdateViewState,
}

#[derive(Debug, Clone, Default)]
pub enum WidgetScene {
    #[default]
    Loading,
    RoutineList,
    RoutineDetail(usize),
    RoutineNew,
}

impl Into<UIEvent> for WidgetScene {
    fn into(self) -> UIEvent {
        UIEvent::OpenWidgetScene(self)
    }
}

#[derive(Debug, Clone)]
pub enum RoutineChangeEvent {
    Rename(String),
    Run,
    Stop,
    Delete,
    SetRunWithStartup(bool),
}

impl RoutineChangeEvent {
    pub fn with_into(&self, index: usize) -> UIEvent {
        UIEvent::RoutineChanged(self.clone(), index)
    }
}

pub struct TerminateThreadEvent;
