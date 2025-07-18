mod loading;
pub mod root;
mod routine_detail;
mod routine_filter;
mod routine_list;
mod routine_modify;
mod styles;

mod const_text {
    pub const TEXT_RUN_AT_STARTUP: &str = "run at startup";
    pub const TEXT_NOT_RUN_AT_STARTUP: &str = "not run at startup";
    pub const TEXT_RUN: &str = "run";
    pub const TEXT_STOP: &str = "stop";
    pub const TEXT_DELETE: &str = "delete";
    pub const TEXT_DETAIL: &str = "detail";
    pub const TEXT_MODIFY: &str = "modify";
    pub const TEXT_DONE: &str = "done";
    pub const TEXT_BACK: &str = "back";
}
