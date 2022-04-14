use crate::Dom;
use crate::{Interface, InterfaceID};

#[repr(C)]
pub struct Node {
    _top: InterfaceID,
    pub(super) parent: Option<Dom<Node>>,
    pub(super) first_child: Option<Dom<Node>>,
    pub(super) last_child: Option<Dom<Node>>,
    pub(super) previous_sibling: Option<Dom<Node>>,
    pub(super) next_sibling: Option<Dom<Node>>,
}

impl Interface for Node {
    fn id() -> InterfaceID {
        InterfaceID::new(1).unwrap()
    }
}

impl Node {
    // NOTE There isn't a Node::new() because a Node is not supposed to
    //      exist without being part of another interface.
    //      This is fine for all interfaces in this crate,
    //      since they can simply create the Node "from scratch".
    //      Interfaces defined outside of this crate has to use interfaces
    //      from this crate (such as Element) to access the functionality of a Node.

    // TODO I think there should be two methods of "interface creation". One method will simply
    // return the struct of the interface and the second method will wrap the struct in a Dom<T>.
    // The first should be used when the resulting interface is only used as a means of inheritance
    // and the second should be used when the result is the top-level interface. I guess there
    // could be a difference in what exactly these two methods does. The first method might be just
    // more bare bone and the second takes more parameters and does more setup, idk.

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

    pub fn append(&mut self, mut node: Dom<Node>) {
        // FIXME If the node were already participating in a tree,
        // we have to make sure that its siblings in that tree are
        // left in a consistent state. Probably best to create
        // a method specifically for this.
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
            },
        }

        self.last_child = Some(node.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_detached_node() {
        let node = Node {
            parent: None,
            first_child: None,
            last_child: None,
            previous_sibling: None,
            next_sibling: None,
        };

        let node = Dom::new(node);

        let tree = Node {
            parent: None,
            first_child: None,
            last_child: None,
            previous_sibling: None,
            next_sibling: None,
        };

        let mut tree = Dom::new(tree);

        tree.append(node.clone());

        assert!(tree.first_child == Some(node.clone()));
        assert!(tree.first_child == tree.last_child);
        assert!(node.parent == Some(tree.clone()));
        assert!(tree.parent == None);
        assert!(tree.previous_sibling == None);
        assert!(tree.next_sibling == None);
        assert!(node.previous_sibling == None);
        assert!(node.next_sibling == None);
        assert!(node.first_child == None);
        assert!(node.last_child == None);
    }
}
