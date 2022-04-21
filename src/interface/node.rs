use crate::Dom;
use crate::{Interface, InterfaceID};

#[repr(C)]
pub struct Node {
    _top: InterfaceID,
    parent: Option<Dom<Node>>,
    first_child: Option<Dom<Node>>,
    last_child: Option<Dom<Node>>,
    previous_sibling: Option<Dom<Node>>,
    next_sibling: Option<Dom<Node>>,
}

impl Interface for Node {
    fn id() -> InterfaceID {
        InterfaceID::new(1)
    }
}
