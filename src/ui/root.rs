use iced_wgpu::Renderer;
use iced_widget::container;
use iced_winit::core::{Element, Length::*, Theme};

use crate::system::system_data::SystemData;
use crate::system::UIEvent;
use crate::ui::style_utils::default_container_style;
use crate::ui::{loading, routine_detail, routine_list, routine_new};

pub fn view(system_data: &SystemData) -> Element<'static, UIEvent, Theme, Renderer> {
    container(match system_data.current_widget_scene {
        crate::system::WidgetScene::Loading => loading::view(),
        crate::system::WidgetScene::RoutineList => routine_list::view(system_data),
        crate::system::WidgetScene::RoutineDetail(_) => routine_detail::view(system_data),
        crate::system::WidgetScene::RoutineNew => routine_new::view(),
    })
    .padding(10)
    .style(default_container_style)
    .into()
}
