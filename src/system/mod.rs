use crate::routine::{method::RoutineMethod, runner::RoutineRunner};

pub mod core;
pub mod data;
pub mod outview;
pub mod single_instance;
pub mod tray;
pub mod ui;

pub enum AppEvent {
    SystemTrayMenuEvent(tray_icon::menu::MenuEvent),
    SystemTrayIconEvent(tray_icon::TrayIconEvent),
}

pub struct TerminateThreadEvent;

#[derive(Debug, Clone)]
pub enum UIEvent {
    OpenWidgetScene(WidgetScene),

    CreateNewRoutine(RoutineMethod),
    CancelCreateRoutine,

    ChangeRoutineRunState(usize, bool),
    ChangeRoutineRunAtStartUpState(usize, bool),
    DeleteRoutine(usize),
    UpdateRoutine(usize, RoutineRunner),
    ModifyTempRoutine(RoutineRunner),

    UpdateOutView,
}

#[derive(Debug, Clone, Default)]
pub enum WidgetScene {
    #[default]
    RoutineList,
    RoutineDetail(usize),       // routine index
    RoutineModify(usize, bool), // routine index, is new routine
}

impl Into<UIEvent> for WidgetScene {
    fn into(self) -> UIEvent {
        UIEvent::OpenWidgetScene(self)
    }
}
