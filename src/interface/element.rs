use crate::interface::Node;
use crate::{Interface, InterfaceID};

#[repr(C)]
pub struct Element {
    _inherited: Node,
}

impl Interface for Element {
    fn id() -> InterfaceID {
        InterfaceID::new(2).unwrap()
    }
}

impl Element {
    pub fn new() -> Element {
        Element {
            _inherited: Node {
                parent: None,
                first_child: None,
                last_child: None,
                previous_sibling: None,
                next_sibling: None,
            },
        }
    }
}
