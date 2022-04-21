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

    // Append `node` as the last child of `self`.
    pub fn append(&mut self, mut node: Dom<Node>) {
        debug_assert!(node.parent().is_none());
        debug_assert!(node.next_sibling().is_none());
        debug_assert!(node.previous_sibling().is_none());
        debug_assert!(node != *self);

        node.previous_sibling = self.last_child();
        node.next_sibling = None;
        node.parent = Some(Dom::from(&*self));

        match self.last_child() {
            Some(mut last) => {
                last.next_sibling = Some(node.clone());
            },
            None => {
                debug_assert!(self.first_child().is_none());
                self.first_child = Some(node.clone());
                // self.last_child is set at the end of the match
            },
        }

        self.last_child = Some(node.clone());
    }

    // Prepend `node` as the first child of `self`.
    pub fn prepend(&mut self, mut node: Dom<Node>) {
        debug_assert!(node.parent().is_none());
        debug_assert!(node.next_sibling().is_none());
        debug_assert!(node.previous_sibling().is_none());
        debug_assert!(node != *self);

        node.previous_sibling = None;
        node.next_sibling = self.first_child();
        node.parent = Some(Dom::from(&*self));

        match self.first_child() {
            Some(mut first) => {
                first.previous_sibling = Some(node.clone());
            },
            None => {
                debug_assert!(self.last_child().is_none());
                self.last_child = Some(node.clone());
                // self.first_child is set at the end of the match
            },
        }

        self.first_child = Some(node.clone());
    }

    // Insert `node` before `self`.
    // NOTE This is not exactly the same as the `insertBefore` method that is
    //      defined on the `Node` interface in the DOM standard, but the
    //      outcome should be the same.
    pub fn insert_before(&mut self, mut node: Dom<Node>) {
        debug_assert!(node.parent().is_none());
        debug_assert!(node.next_sibling().is_none());
        debug_assert!(node.previous_sibling().is_none());
        debug_assert!(node != *self);

        node.previous_sibling = self.previous_sibling();
        node.next_sibling = Some(Dom::from(&*self));
        node.parent = self.parent();

        match self.previous_sibling() {
            Some(mut prev) => {
                prev.next_sibling = Some(node.clone());
            },
            None => {
                self.parent().unwrap().first_child = Some(node.clone());
            },
        }

        self.previous_sibling = Some(node.clone());
    }

    // Insert `node` after `self`.
    pub fn insert_after(&mut self, mut node: Dom<Node>) {
        debug_assert!(node.parent().is_none());
        debug_assert!(node.next_sibling().is_none());
        debug_assert!(node.previous_sibling().is_none());
        debug_assert!(node != *self);

        node.previous_sibling = Some(Dom::from(&*self));
        node.next_sibling = self.next_sibling();
        node.parent = self.parent();

        match self.next_sibling() {
            Some(mut next) => {
                next.previous_sibling = Some(node.clone());
            },
            None => {
                self.parent().unwrap().last_child = Some(node.clone());
            },
        }

        self.next_sibling = Some(node.clone());
    }
}
