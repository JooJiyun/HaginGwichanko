use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

use crate::contexted_err;
use crate::routine::method::RoutineMethod;
use crate::system::TerminateThreadEvent;

const LOOP_INTERVAL_UNIT: u64 = 10;

#[derive(Debug, Clone)]
pub struct RoutineRunner {
    pub routine_name: String,
    pub routine_method: RoutineMethod,

    pub time_created_at: String,
    pub time_last_modified: String,

    pub run_at_startup: bool,

    pub loop_interval: u64,
    tmp_last_loop_interval: u64,
    thread_sender: Option<Sender<TerminateThreadEvent>>,
}

impl RoutineRunner {
    pub fn new(routine_method: RoutineMethod) -> Self {
        Self {
            routine_name: String::new(),
            routine_method,

            time_created_at: String::new(),
            time_last_modified: String::new(),

            run_at_startup: false,

            loop_interval: LOOP_INTERVAL_UNIT,
            tmp_last_loop_interval: LOOP_INTERVAL_UNIT,
            thread_sender: None,
        }
    }

    pub fn is_running(&self) -> bool {
        match &self.thread_sender {
            Some(_) => true,
            None => false,
        }
    }

    pub fn run(&mut self) {
        match &self.thread_sender {
            Some(_) => {}
            None => {
                let sender = run_routine(self.clone());
                self.thread_sender = Some(sender);
                println!("success start <{}> thread", self.routine_name);
            }
        }
    }

    pub fn stop(&mut self) {
        match &self.thread_sender {
            Some(sender) => {
                let send_result = sender
                    .send(TerminateThreadEvent)
                    .or_else(|e| contexted_err!("failed send thread terminate event", e));
                match send_result {
                    Ok(_) => {
                        println!("success terminate <{}> thread", self.routine_name);
                    }
                    Err((msg, e)) => {
                        eprintln!("{} {}", e, msg);
                    }
                }
            }
            None => {}
        }
        self.thread_sender = None;
    }
}

fn run_routine(routine_info: RoutineRunner) -> Sender<TerminateThreadEvent> {
    let (sender, receiver) = channel::<TerminateThreadEvent>();

    let thread_builder = thread::Builder::new().name(routine_info.routine_name.clone());
    let thread_handle_result = thread_builder.spawn(move || {
        run_routine_inner(routine_info, receiver);
    });

    if let Err(e) = thread_handle_result {
        eprintln!("failed spawn skip intro thread : {:?}", e);
    }

    return sender;
}

fn run_routine_inner(mut routine_info: RoutineRunner, receiver: Receiver<TerminateThreadEvent>) {
    loop {
        // method 동작
        if let Ok(_) = receiver.try_recv() {
            println!("received terminate {}", routine_info.routine_name);
            return;
        }
        if let Err(e) = routine_info.routine_method.run_method() {
            println!("{} {}", e.1, e.0);
        }

        // unit time 으로 잘라가면서 delay + 종료 event 체크
        loop {
            if let Ok(_) = receiver.try_recv() {
                println!("received terminate {}", routine_info.routine_name);
                return;
            }
            if routine_info.tmp_last_loop_interval <= LOOP_INTERVAL_UNIT {
                let this_sleep_time = routine_info.tmp_last_loop_interval;
                routine_info.tmp_last_loop_interval = LOOP_INTERVAL_UNIT;
                thread::sleep(Duration::from_millis(this_sleep_time));
                break;
            }
            routine_info.tmp_last_loop_interval -= LOOP_INTERVAL_UNIT;
            thread::sleep(Duration::from_millis(LOOP_INTERVAL_UNIT));
        }
    }
}
