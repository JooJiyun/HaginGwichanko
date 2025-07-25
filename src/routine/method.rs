use crate::routine::method_button_clicker::MethodButtonClicker;
use crate::routine::method_position_clicker::MethodPositionClicker;
use crate::SResult;

pub trait RoutineMethodBase {
    fn run_method(&self) -> crate::SResult<()>;
    fn method_name(&self) -> &'static str;
}

#[derive(Debug, Clone)]
pub enum RoutineMethod {
    ButtonClicker(MethodButtonClicker),
    PositiionClicker(MethodPositionClicker),
}

impl RoutineMethod {
    pub fn get_defaults() -> Vec<Self> {
        vec![
            RoutineMethod::ButtonClicker(MethodButtonClicker::default()),
            RoutineMethod::PositiionClicker(MethodPositionClicker::default()),
        ]
    }

    pub fn run_method(&self) -> SResult<()> {
        match self {
            RoutineMethod::ButtonClicker(method_button_clicker) => {
                method_button_clicker.run_method()
            }
            RoutineMethod::PositiionClicker(method_position_clicker) => {
                method_position_clicker.run_method()
            }
        }
    }

    pub fn method_name(&self) -> &'static str {
        match self {
            RoutineMethod::ButtonClicker(method_button_clicker) => {
                method_button_clicker.method_name()
            }
            RoutineMethod::PositiionClicker(method_position_clicker) => {
                method_position_clicker.method_name()
            }
        }
    }
}
