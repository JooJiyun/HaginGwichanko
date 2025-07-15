use sysinfo::{Pid, System};
use uiautomation::types::Rect;
use uiautomation::{UIAutomation, UIElement, UITreeWalker};

use crate::{contexted_err, SResult};

#[derive(Clone, Debug, Default)]
pub struct ProcessRootViewElement {
    pub root_node: ViewElementNode,
    pub process_name: String,
    pub pid: i32,
}

#[derive(Clone, Debug, Default)]
pub struct ViewElementNode {
    pub info: ViewElementNodeInfo,
    pub childs: Vec<ViewElementNode>,
}

#[derive(Clone, Debug, Default)]
pub struct ViewElementNodeInfo {
    pub name: String,
    pub class: String,
    pub boundary: Rect,
    pub depth: usize,
}

pub fn get_processes_root_node() -> SResult<Vec<ProcessRootViewElement>> {
    // get root ui
    let automation: UIAutomation =
        UIAutomation::new().or_else(|e| contexted_err!("failed get ui viewer", e))?;
    let root = automation
        .get_root_element()
        .or_else(|e| contexted_err!("failed get ui viewer root", e))?;
    let walker = automation
        .create_tree_walker()
        .or_else(|e| contexted_err!("failed get ui viewer walker", e))?;

    // get process root ui element
    let mut process_root_ui_elements = vec![];
    if let Ok(child) = walker.get_first_child(&root) {
        process_root_ui_elements.push(child.clone());
        let mut next = child;
        while let Ok(sibling) = walker.get_next_sibling(&next) {
            process_root_ui_elements.push(sibling.clone());
            next = sibling;
        }
    }

    // get process name getter
    let mut sys = System::new_all();
    sys.refresh_all();

    // get process root
    let mut process_root_elements = vec![];
    for process_root_ui_element in process_root_ui_elements {
        // process id
        let process_id = process_root_ui_element.get_process_id().unwrap_or(0);
        if process_id == 0 {
            continue;
        }

        // process name
        let process_name = if let Some(process) = sys.process(Pid::from_u32(process_id as u32)) {
            process.name().to_str().unwrap_or("").to_string()
        } else {
            String::from("")
        };

        // traversal childs
        let root_element = get_element_recursive(&walker, &process_root_ui_element, 1);

        // make root element
        let process_root = ProcessRootViewElement {
            root_node: root_element,
            process_name: process_name,
            pid: process_id,
        };
        process_root_elements.push(process_root);
    }

    Ok(process_root_elements)
}

pub fn get_element_recursive(
    walker: &UITreeWalker,
    element: &UIElement,
    depth: usize,
) -> ViewElementNode {
    let mut node = ViewElementNode::default();
    node.info = ViewElementNodeInfo {
        name: element.get_name().unwrap_or(String::from("none")),
        class: element.get_classname().unwrap_or(String::from("none")),
        boundary: element.get_bounding_rectangle().unwrap_or(Rect::default()),
        depth: depth,
    };

    if let Ok(child) = walker.get_first_child(&element) {
        let this_child = get_element_recursive(walker, &child, depth + 1);
        node.childs.push(this_child);

        let mut next = child;
        while let Ok(sibling) = walker.get_next_sibling(&next) {
            let this_child = get_element_recursive(walker, &sibling, depth + 1);
            node.childs.push(this_child);
            next = sibling;
        }
    }

    return node;
}
