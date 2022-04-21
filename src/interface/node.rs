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

impl Node {
    pub fn new() -> Self {
        Node {
            _top: unsafe { InterfaceID::zero() },
            parent: None,
            first_child: None,
            last_child: None,
            previous_sibling: None,
            next_sibling: None,
        }
    }

    pub fn parent(&self) -> Option<Dom<Node>> {
        self.parent.clone()
    }

    pub fn first_child(&self) -> Option<Dom<Node>> {
        self.first_child.clone()
    }

    pub fn last_child(&self) -> Option<Dom<Node>> {
        self.last_child.clone()
    }

    pub fn previous_sibling(&self) -> Option<Dom<Node>> {
        self.previous_sibling.clone()
    }

    pub fn next_sibling(&self) -> Option<Dom<Node>> {
        self.next_sibling.clone()
    }
}
