use std::sync::atomic::{AtomicUsize, Ordering};

use crate::routine::runner::RoutineRunner;
use crate::system::outview::RootViewElement;
use crate::system::WidgetScene;

static ROUTINE_THREAD_ATOMIC_ID: AtomicUsize = AtomicUsize::new(1);

pub fn generate_new_routine_thread_id() -> usize {
    ROUTINE_THREAD_ATOMIC_ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone, Debug, Default)]
pub struct AppData {
    pub current_widget_scene: WidgetScene,

    pub routines: Vec<RoutineRunner>,
    pub outview_trees: Vec<RootViewElement>,
    pub tmp_modify_routine: Option<RoutineRunner>,

    pub version_info: String,
}

impl AppData {
    pub fn save(&self) {}

    pub fn load() -> Self {
        Self::default()
    }
}
