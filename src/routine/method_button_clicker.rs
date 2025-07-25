use uiautomation::inputs::Mouse;
use uiautomation::{UIAutomation, UIElement};

use crate::routine::method::RoutineMethodBase;
use crate::routine::variable::{
    RoutineVariableFilter, RoutineVariableFilterList, RoutineVariableTime,
};
use crate::{contexted_err, SResult};

const METHOD_NAME: &str = "button clicker";

#[derive(Clone, Debug)]
pub struct MethodButtonClicker {
    pub filter_list: RoutineVariableFilterList,
    pub mouse_speed: RoutineVariableTime,
    pub find_time_limit: RoutineVariableTime,
}

impl Default for MethodButtonClicker {
    fn default() -> Self {
        Self {
            filter_list: vec![],
            mouse_speed: RoutineVariableTime::MOUSE_MOVE_TIME_DEFAULT,
            find_time_limit: RoutineVariableTime::FIND_TIME_DEFAULT,
        }
    }
}

impl RoutineMethodBase for MethodButtonClicker {
    fn method_name(&self) -> &'static str {
        METHOD_NAME
    }

    fn run_method(&self) -> SResult<()> {
        let automation: UIAutomation =
            UIAutomation::new().or_else(|e| contexted_err!("failed to find view", e))?;
        let root = automation
            .get_root_element()
            .or_else(|e| contexted_err!("failed to find target button", e))?;
        self.run_inner_recursive(&root, 0, &automation)
    }
}

impl MethodButtonClicker {
    fn run_inner_recursive(
        &self,
        root: &UIElement,
        filter_index: usize,
        automation: &UIAutomation,
    ) -> SResult<()> {
        let this_filter = &self.filter_list[filter_index];

        // find target button and click
        if filter_index == self.filter_list.len() - 1 {
            self.find_button_and_click(root, this_filter, automation)?;
        }
        // filter
        else {
            let filtered_elements = automation
                .create_matcher()
                .from(root.clone())
                .timeout(self.find_time_limit.as_millis_value())
                .depth(this_filter.depth)
                .contains_name(this_filter.text.clone())
                .find_all()
                .or_else(|e| contexted_err!("failed to find filtered element", e))?;

            match this_filter.index {
                Some(target_index) => {
                    if target_index >= filtered_elements.len() {
                        return Ok(());
                    }
                    self.run_inner_recursive(
                        &filtered_elements[target_index],
                        filter_index + 1,
                        automation,
                    )?;
                }
                None => {
                    for filtered_element in &filtered_elements {
                        self.run_inner_recursive(filtered_element, filter_index + 1, automation)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn find_button_and_click(
        &self,
        root: &UIElement,
        button_filter: &RoutineVariableFilter,
        automation: &UIAutomation,
    ) -> SResult<()> {
        // run target button filter
        let target_buttons = automation
            .create_matcher()
            .from(root.clone())
            .timeout(self.find_time_limit.as_millis_value())
            .depth(button_filter.depth)
            .control_type(uiautomation::controls::ControlType::Button)
            .find_all()
            .or_else(|e| contexted_err!("failed to find target buttons", e))?;

        // filter button text, click
        let mut current_index = 0;
        let target_index = button_filter.index.unwrap_or(0);
        for target_button in target_buttons {
            let button_text = target_button
                .get_name()
                .or_else(|e| contexted_err!("failed to find button text", e))?;
            if button_text == button_filter.text {
                if current_index == target_index {
                    self.click_element(&target_button)?;
                }
                current_index += 1;
            }
        }

        Ok(())
    }

    fn click_element(&self, element: &UIElement) -> SResult<()> {
        element.try_focus();
        let button_point = element
            .get_clickable_point()
            .or_else(|e| contexted_err!("failed getting button point", e))?;

        if let Some(point) = button_point {
            let mouse = Mouse::default().move_time(self.mouse_speed.as_millis_value());
            mouse
                .click(point)
                .or_else(|e| contexted_err!("failed click mouse", e))?;
        }

        Ok(())
    }
}
