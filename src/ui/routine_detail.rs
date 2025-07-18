use iced_widget::{button, column, row, text};

use crate::system::routine_state::RoutineMethod;
use crate::system::system_data::SystemData;
use crate::ui::style_utils::default_container_style;

const TEXT_MODIFY: &str = "modify";
const TEXT_BACK: &str = "back";

pub fn view(
    system_data: &SystemData,
) -> iced_core::Element<'static, crate::system::UIEvent, iced_core::Theme, iced_wgpu::Renderer> {
    let selected_routine = &system_data.routines[system_data.selected_routine_index];

    iced_widget::container(column![
        row![button(text(TEXT_MODIFY)), button(text(TEXT_BACK)),],
        text(selected_routine.name.clone()),
        routine_method_view(&selected_routine.routin_method)
    ])
    .style(default_container_style)
    .padding(0)
    .into()
}

fn routine_method_view(
    routine_method: &RoutineMethod,
) -> iced_core::Element<'static, crate::system::UIEvent, iced_core::Theme, iced_wgpu::Renderer> {
    let method_inner = match routine_method {
        RoutineMethod::None => row![],
        RoutineMethod::ClickPosition(click_position_info) => row![text("click_position_info")],
        RoutineMethod::ClickButtonIfFind(click_button_if_find_info) => {
            row![text("click_button_if_find_info")]
        }
    };
    iced_widget::container(method_inner)
        .style(default_container_style)
        .padding(0)
        .into()
}
