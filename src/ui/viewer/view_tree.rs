use uiautomation::{UIAutomation, UIElement, UITreeWalker};

use crate::{contexted_err, SResult};

fn print_all_element() -> SResult<()> {
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
    let process_ui_roots = automation
        .create_matcher()
        .from(root)
        .timeout(10000)
        .find_all()
        .or_else(|e| contexted_err!("failed get processes ui", e))?;

    // get text
    for process_ui_root in process_ui_roots {
        print_element_recursive(&walker, &process_ui_root, 1)?;
    }

    Ok(())
}

fn print_element_recursive(
    walker: &UITreeWalker,
    element: &UIElement,
    level: usize,
) -> SResult<()> {
    println!("{:?} {:?}", level, element);

    // 자식 탐색
    if let Ok(child) = walker.get_first_child(&element) {
        print_element_recursive(walker, &child, level + 1)?;

        let mut next = child;
        while let Ok(sibling) = walker.get_next_sibling(&next) {
            print_element_recursive(walker, &sibling, level + 1)?;
            next = sibling;
        }
    }

    Ok(())
}
