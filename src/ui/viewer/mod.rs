mod view_tree;
mod view_tree_item;

pub fn view(
    system_data: &crate::system::data::SystemData,
) -> iced_core::Element<'static, crate::system::UIEvent, iced_core::Theme, iced_wgpu::Renderer> {
    let mut view_trees = iced_widget::row![];
    for process_root_view_element in &system_data.view_tree_processes {
        view_trees = view_trees.push(view_tree::view(process_root_view_element));
    }

    iced_widget::container(view_trees).padding(0).into()
}
