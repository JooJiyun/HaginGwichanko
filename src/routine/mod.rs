use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use crate::system::TerminateThread;

pub struct RoutineInfo {
    routine_name: String,
    blocks: Vec<RoutineBlock>,
}

pub enum RoutineBlock {
    Click,
}

pub fn run_routine(routine_info: RoutineInfo) -> Sender<TerminateThread> {
    let (sender, receiver) = channel::<TerminateThread>();

    let thread_builder = thread::Builder::new().name(routine_info.routine_name.clone());
    let thread_handle_result = thread_builder.spawn(move || {
        run_routine_inner(routine_info, receiver);
    });

    if let Err(e) = thread_handle_result {
        eprintln!("failed spawn skip intro thread : {:?}", e);
    }

    return sender;
}

fn run_routine_inner(routine_info: RoutineInfo, receiver: Receiver<TerminateThread>) {
    loop {
        // 오직 종료만 receiver로 받음
        if let Ok(_) = receiver.try_recv() {
            println!("received terminate {}", routine_info.routine_name);
            return;
        }

        for routin_block in &routine_info.blocks {
            println!("");
        }
    }
}
