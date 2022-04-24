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

    // Detach the subtree that this node is the root of.
    pub fn detach(&mut self) {
        match self.parent() {
            None => {
                // The node doesn't belong to a tree, nothing to do.
                // This also means that it shouldn't have any siblings.
                debug_assert!(self.previous_sibling().is_none());
                debug_assert!(self.next_sibling().is_none());
            },
            Some(mut parent) => {
                debug_assert!(parent.first_child().is_some());
                debug_assert!(parent.last_child().is_some());
                if parent.first_child().unwrap() == *self {
                    parent.first_child = self.next_sibling();
                }

                if parent.last_child().unwrap() == *self {
                    parent.last_child = self.previous_sibling();
                }

                // TODO The `None` case in these two matches should basically
                // correspond to the if-cases above.
                match self.previous_sibling() {
                    Some(mut prev) => {
                        prev.next_sibling = self.next_sibling();
                    },
                    None => (),
                }

                match self.next_sibling() {
                    Some(mut next) => {
                        next.previous_sibling = self.previous_sibling();
                    },
                    None => (),
                }

                self.previous_sibling = None;
                self.next_sibling = None;
                self.parent = None;
            },
        }
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
                last.next_sibling = Some(Dom::clone(&node));
            },
            None => {
                debug_assert!(self.first_child().is_none());
                self.first_child = Some(Dom::clone(&node));
                // self.last_child is set at the end of the match
            },
        }

        self.last_child = Some(Dom::clone(&node));
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
                first.previous_sibling = Some(Dom::clone(&node));
            },
            None => {
                debug_assert!(self.last_child().is_none());
                self.last_child = Some(Dom::clone(&node));
                // self.first_child is set at the end of the match
            },
        }

        self.first_child = Some(Dom::clone(&node));
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
                prev.next_sibling = Some(Dom::clone(&node));
            },
            None => {
                self.parent().unwrap().first_child = Some(Dom::clone(&node));
            },
        }

        self.previous_sibling = Some(Dom::clone(&node));
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
                next.previous_sibling = Some(Dom::clone(&node));
            },
            None => {
                self.parent().unwrap().last_child = Some(Dom::clone(&node));
            },
        }

        self.next_sibling = Some(Dom::clone(&node));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detach_new_node() {
        let mut node = Dom::new(Node::new());

        node.detach();

        assert!(node.parent().is_none());
        assert!(node.previous_sibling().is_none());
        assert!(node.next_sibling().is_none());
        assert!(node.first_child().is_none());
        assert!(node.last_child().is_none());
    }

    #[test]
    fn detach_node_without_siblings() {
        let mut parent = Dom::new(Node::new());
        let mut child = Dom::new(Node::new());

        parent.append(Dom::clone(&child));

        child.detach();

        assert!(child.parent().is_none());
        assert!(child.previous_sibling().is_none());
        assert!(child.next_sibling().is_none());
        assert!(child.first_child().is_none());
        assert!(child.last_child().is_none());

        assert!(parent.first_child().is_none());
        assert!(parent.last_child().is_none());
    }

    #[test]
    fn detach_node_with_siblings() {
        let mut parent = Dom::new(Node::new());
        let     first  = Dom::new(Node::new());
        let     last   = Dom::new(Node::new());
        let mut node   = Dom::new(Node::new());

        parent.append(Dom::clone(&first));
        parent.append(Dom::clone(&node));
        parent.append(Dom::clone(&last));

        node.detach();

        assert!(node.parent().is_none());
        assert!(node.previous_sibling().is_none());
        assert!(node.next_sibling().is_none());
        assert!(node.first_child().is_none());
        assert!(node.last_child().is_none());

        assert!(first.parent().is_some());
        assert!(last.parent().is_some());
        assert!(first.parent().unwrap() == parent);
        assert!(last.parent().unwrap() == parent);

        assert!(first.next_sibling().is_some());
        assert!(last.previous_sibling().is_some());
        assert!(first.next_sibling().unwrap() == last);
        assert!(last.previous_sibling().unwrap() == first);

        assert!(parent.first_child().is_some());
        assert!(parent.last_child().is_some());
        assert!(parent.first_child().unwrap() == first);
        assert!(parent.last_child().unwrap() == last);
    }

    #[test]
    fn detach_node_with_next_sibling() {
        let mut parent = Dom::new(Node::new());
        let mut node   = Dom::new(Node::new());
        let     next   = Dom::new(Node::new());

        parent.append(Dom::clone(&node));
        parent.append(Dom::clone(&next));

        node.detach();

        assert!(node.parent().is_none());
        assert!(node.previous_sibling().is_none());
        assert!(node.next_sibling().is_none());
        assert!(node.first_child().is_none());
        assert!(node.last_child().is_none());

        assert!(parent.first_child().is_some());
        assert!(parent.last_child().is_some());
        assert!(parent.first_child().unwrap() == parent.last_child().unwrap());

        assert!(next.parent().is_some());
        assert!(next.parent().unwrap() == parent);
        assert!(next.next_sibling().is_none());
        assert!(next.previous_sibling().is_none());
    }
}
