use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use crate::system::TerminateThreadEvent;

#[derive(Clone, Debug, Default)]
pub struct RoutineInfo {
    pub name: String,
    pub created_at: String,
    pub last_modified: String,

    pub is_running: bool,
    pub run_at_startup: bool,

    pub routin_method: RoutineMethod,
}

#[derive(Clone, Debug, Default)]
pub enum RoutineMethod {
    #[default]
    None,
    ClickPosition(ClickPositionInfo),
    ClickButtonIfFind(ClickButtonIfFindInfo),
}

impl RoutineMethod {
    pub const fn method_name(&self) -> &str {
        match self {
            RoutineMethod::None => "None",
            RoutineMethod::ClickPosition(_) => "Click Position",
            RoutineMethod::ClickButtonIfFind(_) => "Click Button If Find",
        }
    }
}

pub fn run_routine(routine_info: RoutineInfo) -> Sender<TerminateThreadEvent> {
    let (sender, receiver) = channel::<TerminateThreadEvent>();

    let thread_builder = thread::Builder::new().name(routine_info.name.clone());
    let thread_handle_result = thread_builder.spawn(move || {
        run_routine_inner(routine_info, receiver);
    });

    if let Err(e) = thread_handle_result {
        eprintln!("failed spawn skip intro thread : {:?}", e);
    }

    return sender;
}

fn run_routine_inner(routine_info: RoutineInfo, receiver: Receiver<TerminateThreadEvent>) {
    loop {
        // 오직 종료만 receiver로 받음
        if let Ok(_) = receiver.try_recv() {
            println!("received terminate {}", routine_info.name);
            return;
        }
        match routine_info.routin_method {
            RoutineMethod::None => todo!(),
            RoutineMethod::ClickPosition(ref click_position_info) => click_position_info.run(),
            RoutineMethod::ClickButtonIfFind(ref click_button_if_find_info) => {
                click_button_if_find_info.run()
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClickPositionInfo {
    target_position: (f32, f32),
    interval: f32,
    mouse_speed: f32,
}
impl ClickPositionInfo {
    fn run(&self) {}
}

#[derive(Clone, Debug, Default)]
pub struct ClickButtonIfFindInfo {
    target_button: f32,
    interval: f32,
    mouse_speed: f32,
}
impl ClickButtonIfFindInfo {
    fn run(&self) {}
}
