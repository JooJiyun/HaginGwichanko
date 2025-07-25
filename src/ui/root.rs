use iced_core::Length;
use iced_widget::{container, scrollable};

use crate::system::data::AppData;
use crate::system::WidgetScene;
use crate::ui::loading::view_loading;
use crate::ui::routine_detail::view_routine_detail;
use crate::ui::routine_list::view_routine_list;
use crate::ui::routine_modify::view_routine_modify;
use crate::ui::styles::default_container_style;
use crate::ui::AppUIElement;

pub fn view(system_data: &AppData) -> AppUIElement {
    container(scrollable(match system_data.current_widget_scene {
        WidgetScene::Loading => view_loading(),
        WidgetScene::RoutineList => view_routine_list(system_data),
        WidgetScene::RoutineDetail(routine_index) => {
            view_routine_detail(system_data, routine_index)
        }
        WidgetScene::RoutineModify(routine_index, is_new_routine) => {
            view_routine_modify(system_data, routine_index, is_new_routine)
        }
    }))
    .padding(10)
    .style(default_container_style)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
