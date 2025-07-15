use crate::system::view_func::ProcessRootViewElement;

#[derive(Clone, Debug, Default)]
pub struct SystemData {
    pub count: usize,
    pub view_tree_processes: Vec<ProcessRootViewElement>,
}
