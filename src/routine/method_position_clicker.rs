use uiautomation::inputs::Mouse;
use uiautomation::types::Point;

use crate::contexted_err;
use crate::routine::method::RoutineMethodBase;
use crate::routine::variable::{RoutineVariablePosition, RoutineVariableTime};
use crate::SResult;

const METHOD_NAME: &str = "position clicker";

#[derive(Clone, Debug)]
pub struct MethodPositionClicker {
    pub mouse_speed: RoutineVariableTime,
    pub target_position: RoutineVariablePosition,
}

impl Default for MethodPositionClicker {
    fn default() -> Self {
        Self {
            mouse_speed: RoutineVariableTime::MOUSE_MOVE_TIME_DEFAULT,
            target_position: (0, 0),
        }
    }
}

impl RoutineMethodBase for MethodPositionClicker {
    fn method_name(&self) -> &'static str {
        METHOD_NAME
    }

    fn run_method(&self) -> SResult<()> {
        let mouse = Mouse::default().move_time(self.mouse_speed.as_millis_value());
        mouse
            .click(Point::new(self.target_position.0, self.target_position.1))
            .or_else(|e| contexted_err!("failed to get mouse handle", e))?;

        Ok(())
    }
}
