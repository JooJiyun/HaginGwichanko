use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

use uiautomation::{inputs::Mouse, types::Point};
use uiautomation::{UIAutomation, UIElement};

use crate::{contexted_err, system::TerminateThreadEvent, SResult};

const LOOP_INTERVAL_UNIT: u64 = 10;
const LOOKUP_TIME_LIMIT_MIN: u64 = 1000;
const MOUSE_MOVE_TIME_DEFAULT: u64 = 10;

#[derive(Clone, Debug, Default)]
pub struct RoutineInfo {
    pub name: String,
    pub created_at: String,
    pub last_modified: String,

    pub is_running: bool,
    pub run_at_startup: bool,

    pub routin_method: RoutineMethod,
    pub loop_interval: u64,
    pub last_loop_interval: u64,
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

fn run_routine_inner(mut routine_info: RoutineInfo, receiver: Receiver<TerminateThreadEvent>) {
    loop {
        // action
        if let Ok(_) = receiver.try_recv() {
            println!("received terminate {}", routine_info.name);
            return;
        }
        if let Err(e) = match routine_info.routin_method {
            RoutineMethod::None => Ok(()),
            RoutineMethod::ClickPosition(ref click_position_info) => click_position_info.run(),
            RoutineMethod::ClickButtonIfFind(ref click_button_if_find_info) => {
                click_button_if_find_info.run()
            }
        } {
            println!("{} {}", e.1, e.0);
        }

        // delay
        loop {
            if let Ok(_) = receiver.try_recv() {
                println!("received terminate {}", routine_info.name);
                return;
            }
            if routine_info.last_loop_interval <= LOOP_INTERVAL_UNIT {
                let this_sleep_time = routine_info.last_loop_interval;
                routine_info.last_loop_interval = LOOP_INTERVAL_UNIT;
                thread::sleep(Duration::from_millis(this_sleep_time));
                break;
            }
            routine_info.last_loop_interval -= LOOP_INTERVAL_UNIT;
            thread::sleep(Duration::from_millis(LOOP_INTERVAL_UNIT));
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ClickPositionInfo {
    pub target_position: (i32, i32),
    pub mouse_speed: u64,
}

impl ClickPositionInfo {
    fn run(&self) -> SResult<()> {
        let mouse = Mouse::default().move_time(self.mouse_speed);
        mouse
            .click(Point::new(self.target_position.0, self.target_position.1))
            .or_else(|e| contexted_err!("failed to get mouse handle", e))?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct ClickButtonIfFindInfo {
    pub filter: Vec<FilterInfo>,
    pub mouse_speed: u64,
    pub find_time_limit: u64,
}

impl Default for ClickButtonIfFindInfo {
    fn default() -> Self {
        Self {
            filter: vec![],
            mouse_speed: MOUSE_MOVE_TIME_DEFAULT,
            find_time_limit: LOOKUP_TIME_LIMIT_MIN,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct FilterInfo {
    pub text: String,
    pub depth: u32,
    pub index: Option<usize>,
}

impl ClickButtonIfFindInfo {
    fn run(&self) -> SResult<()> {
        let automation: UIAutomation =
            UIAutomation::new().or_else(|e| contexted_err!("failed to find view", e))?;
        let root = automation
            .get_root_element()
            .or_else(|e| contexted_err!("failed to find target button", e))?;
        self.run_inner_recursive(&root, 0, &automation)
    }

    fn run_inner_recursive(
        &self,
        root: &UIElement,
        filter_index: usize,
        automation: &UIAutomation,
    ) -> SResult<()> {
        let this_filter = &self.filter[filter_index];

        // find target button and click
        if filter_index == self.filter.len() - 1 {
            self.find_button_and_click(root, this_filter, automation)?;
        }
        // filter
        else {
            let filtered_elements = automation
                .create_matcher()
                .from(root.clone())
                .timeout(self.find_time_limit)
                .depth(this_filter.depth)
                .contains_name(this_filter.text.clone())
                .find_all()
                .or_else(|e| contexted_err!("failed to find filtered element", e))?;

            match this_filter.index {
                Some(target_index) => {
                    if target_index >= filtered_elements.len() {
                        return Ok(());
                    }
                    self.run_inner_recursive(
                        &filtered_elements[target_index],
                        filter_index + 1,
                        automation,
                    )?;
                }
                None => {
                    for filtered_element in &filtered_elements {
                        self.run_inner_recursive(filtered_element, filter_index + 1, automation)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn find_button_and_click(
        &self,
        root: &UIElement,
        button_filter: &FilterInfo,
        automation: &UIAutomation,
    ) -> SResult<()> {
        // run target button filter
        let target_buttons = automation
            .create_matcher()
            .from(root.clone())
            .timeout(self.find_time_limit)
            .depth(button_filter.depth)
            .control_type(uiautomation::controls::ControlType::Button)
            .find_all()
            .or_else(|e| contexted_err!("failed to find target buttons", e))?;

        // filter button text, click
        let mut current_index = 0;
        let target_index = button_filter.index.unwrap_or(0);
        for target_button in target_buttons {
            let button_text = target_button
                .get_name()
                .or_else(|e| contexted_err!("failed to find button text", e))?;
            if button_text == button_filter.text {
                if current_index == target_index {
                    self.click_element(&target_button)?;
                }
                current_index += 1;
            }
        }

        Ok(())
    }

    fn click_element(&self, element: &UIElement) -> SResult<()> {
        element.try_focus();
        let button_point = element
            .get_clickable_point()
            .or_else(|e| contexted_err!("failed getting button point", e))?;

        if let Some(point) = button_point {
            let mouse = Mouse::default().move_time(self.mouse_speed);
            mouse
                .click(point)
                .or_else(|e| contexted_err!("failed click mouse", e))?;
        }

        Ok(())
    }
}
