mod dom;
mod cast;

pub use crate::dom::Dom;

pub use crate::cast::Cast;
pub use crate::cast::{Interface, InterfaceID};
pub use crate::cast::HIERARCHY;

pub mod interface;

use crate::interface::{Node, Document, Element};

pub fn init() {
    let mut hier = HIERARCHY.write().unwrap();

    hier.register(Node::id(), None);
    hier.register(Document::id(), Some(Node::id()));
    hier.register(Element::id(), Some(Node::id()));
}

#[cfg(test)]
pub(crate) mod tests_init {
    use crate::init;
    use crate::cast::HIERARCHY;

    pub fn hierarchy_init() {
        HIERARCHY.write().unwrap().clear();
        init();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
