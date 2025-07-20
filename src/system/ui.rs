use std::sync::{Arc, Mutex};

use iced_wgpu::Renderer;
use iced_winit::core::{Element, Theme};
use iced_winit::runtime::{Program, Task};

use crate::system::data::AppData;
use crate::system::outview::get_processes_root_node;
use crate::system::routine::RoutineInfo;
use crate::system::{RoutineChangeEvent, UIEvent, WidgetScene};
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
            Err((msg, err)) => {}
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
        match message {
            UIEvent::OpenWidgetScene(widget_scene) => {
                let mut data_value = self
                    .system_data
                    .lock()
                    .or_else(|e| contexted_err!("failed get arc mutex", e))?;
                data_value.current_widget_scene = widget_scene;
            }
            UIEvent::RoutineChanged(routine_change_event, routine_index) => {
                self.run_routine_event(routine_change_event, routine_index)?;
            }
            UIEvent::CreateNewRoutine(routine_method) => {
                let mut data_value = self
                    .system_data
                    .lock()
                    .or_else(|e| contexted_err!("failed get arc mutex", e))?;
                let routine_index = data_value.routines.len();
                data_value.routines.push(RoutineInfo::new(routine_method));
                data_value.current_widget_scene = WidgetScene::RoutineModify(routine_index, true);
            }
            UIEvent::UpdateViewState => {
                let mut data_value = self
                    .system_data
                    .lock()
                    .or_else(|e| contexted_err!("failed get arc mutex", e))?;
                data_value.outview_trees = get_processes_root_node()?;
            }
        }
        Ok(())
    }

    fn run_routine_event(
        &mut self,
        routine_change_event: RoutineChangeEvent,
        routine_index: usize,
    ) -> SResult<()> {
        let mut data_value = self
            .system_data
            .lock()
            .or_else(|e| contexted_err!("failed get arc mutex", e))?;

        // valid index check
        if routine_index >= data_value.routines.len() {
            let _errrr: Result<(), (String, String)> = contexted_err!(
                "not exist routine",
                format!("{routine_index} not exist routine index")
            );
        }

        match routine_change_event {
            crate::system::RoutineChangeEvent::ChangeRunState(is_running) => {
                data_value.routines[routine_index].is_running = is_running;
            }
            crate::system::RoutineChangeEvent::Delete => {
                data_value.routines.remove(routine_index);
                data_value.current_widget_scene = WidgetScene::RoutineList;
            }
            crate::system::RoutineChangeEvent::SetRunAtStartup(run_at_start_up) => {
                data_value.routines[routine_index].run_at_startup = run_at_start_up;
            }
        }

        Ok(())
    }
}
