use iced_widget::{container, row};

pub fn view(
) -> iced_core::Element<'static, crate::system::UIEvent, iced_core::Theme, iced_wgpu::Renderer> {
    container(row![]).padding(0).into()
}
