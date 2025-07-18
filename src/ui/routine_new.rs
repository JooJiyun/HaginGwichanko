use iced_widget::row;

pub fn view(
) -> iced_core::Element<'static, crate::system::UIEvent, iced_core::Theme, iced_wgpu::Renderer> {
    iced_widget::container(row![]).padding(0).into()
}
