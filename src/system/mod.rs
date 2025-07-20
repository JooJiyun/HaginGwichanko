use crate::system::routine::RoutineMethod;

pub mod core;
pub mod data;
pub mod routine;
pub mod single_instance;
pub mod tray;
pub mod ui;
pub mod outview;

pub enum AppEvent {
    SystemTrayEvent(tray_icon::menu::MenuEvent),
}

#[derive(Debug, Clone)]
pub enum UIEvent {
    OpenWidgetScene(WidgetScene),

    RoutineChanged(RoutineChangeEvent, usize),
    CreateNewRoutine(RoutineMethod),

    UpdateViewState,
}

#[derive(Debug, Clone, Default)]
pub enum WidgetScene {
    #[default]
    Loading,
    RoutineList,
    RoutineDetail(usize),       // routine index
    RoutineModify(usize, bool), // routine index, is new routine
}

impl Into<UIEvent> for WidgetScene {
    fn into(self) -> UIEvent {
        UIEvent::OpenWidgetScene(self)
    }
}

#[derive(Debug, Clone)]
pub enum RoutineChangeEvent {
    ChangeRunState(bool),
    Delete,
    SetRunAtStartup(bool),
}

impl RoutineChangeEvent {
    pub fn with_into(&self, index: usize) -> UIEvent {
        UIEvent::RoutineChanged(self.clone(), index)
    }
}

pub struct TerminateThreadEvent;
