use std::sync::{Arc, Mutex};

use iced_wgpu::Renderer;
use iced_winit::core::{Element, Theme};
use iced_winit::runtime::{Program, Task};

use crate::routine::runner::RoutineRunner;
use crate::system::data::AppData;
use crate::system::outview::get_processes_root_node;
use crate::system::{UIEvent, WidgetScene};
use crate::ui::root::view;
use crate::{contexted_err, SResult};

pub struct AppUI {
    pub messages: Vec<UIEvent>,
    pub system_data: Arc<Mutex<AppData>>,
}

impl AppUI {
    pub fn new(data: Arc<Mutex<AppData>>) -> AppUI {
        AppUI {
            messages: vec![],
            system_data: data,
        }
    }
}

impl Program for AppUI {
    type Theme = Theme;
    type Message = UIEvent;
    type Renderer = Renderer;

    fn update(&mut self, message: UIEvent) -> Task<UIEvent> {
        let update_result = self.update_inner(message);
        match update_result {
            Ok(_) => {}
            Err((msg, err)) => {
                println!("{}, {}", err, msg);
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<UIEvent, Theme, Renderer> {
        let data_value = self.system_data.lock().expect("failed get arc mutex");
        let system_data = data_value.clone();
        view(&system_data).into()
    }
}

impl AppUI {
    fn update_inner(&mut self, message: UIEvent) -> SResult<()> {
        let mut data_value = self
            .system_data
            .lock()
            .or_else(|e| contexted_err!("failed get arc mutex", e))?;

        // valid event check : routine index
        match message {
            UIEvent::ChangeRoutineRunState(routine_index, _)
            | UIEvent::DeleteRoutine(routine_index)
            | UIEvent::ChangeRoutineRunAtStartUpState(routine_index, _)
            | UIEvent::UpdateRoutine(routine_index, _)
            | UIEvent::OpenWidgetScene(WidgetScene::RoutineDetail(routine_index))
            | UIEvent::OpenWidgetScene(WidgetScene::RoutineModify(routine_index, _)) => {
                // valid index check
                if routine_index >= data_value.routines.len() {
                    let _errrr: Result<(), (String, String)> = contexted_err!(
                        "not exist routine",
                        format!("{:?} {routine_index} not exist routine index", message)
                    );
                }
            }
            _ => {}
        }

        // run event
        match message {
            UIEvent::OpenWidgetScene(widget_scene) => {
                match widget_scene {
                    WidgetScene::RoutineModify(routine_id, _) => {
                        data_value.tmp_modify_routine =
                            Some(data_value.routines[routine_id].clone());
                    }
                    _ => {}
                }
                data_value.current_widget_scene = widget_scene;
            }
            UIEvent::CreateNewRoutine(routine_method) => {
                let routine_index = data_value.routines.len();
                let routine_info = RoutineRunner::new(routine_method);

                data_value.routines.push(routine_info.clone());
                data_value.current_widget_scene = WidgetScene::RoutineModify(routine_index, true);
                data_value.tmp_modify_routine = Some(routine_info.clone());
            }
            UIEvent::CancelCreateRoutine => {
                data_value.routines.pop();
                data_value.current_widget_scene = WidgetScene::RoutineList;
                data_value.tmp_modify_routine = None;
            }
            UIEvent::UpdateOutView => {
                data_value.outview_trees = get_processes_root_node()?;
            }
            UIEvent::ChangeRoutineRunState(routine_index, is_running) => {
                data_value.routines[routine_index].state_is_running = is_running;
            }
            UIEvent::ChangeRoutineRunAtStartUpState(routine_index, state) => {
                data_value.routines[routine_index].state_run_at_startup = state;
            }
            UIEvent::DeleteRoutine(routine_index) => {
                data_value.routines.remove(routine_index);
                data_value.current_widget_scene = WidgetScene::RoutineList;
            }
            UIEvent::UpdateRoutine(routine_index, routine_info) => {
                data_value.routines[routine_index] = routine_info;
                data_value.tmp_modify_routine = None;
            }
            UIEvent::ModifyTempRoutine(routine_info) => {
                data_value.tmp_modify_routine = Some(routine_info);
                data_value.tmp_modify_routine = None;
            }
        }
        Ok(())
    }
}
