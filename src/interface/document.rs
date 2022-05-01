use crate::Dom;
use crate::Cast;
use crate::interface::{Node, Element};
use crate::{Interface, InterfaceID};

use std::ops::{Deref, DerefMut};

#[repr(C)]
pub struct Document {
    _inherited: Node,
}

impl Interface for Document {
    fn id() -> InterfaceID {
        InterfaceID::new(2)
    }
}

impl Deref for Document {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self._inherited
    }
}

impl DerefMut for Document {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._inherited
    }
}

impl Document {
    pub fn new() -> Self {
        Document {
            _inherited: Node::new()
        }
    }

    pub fn create() -> Dom<Self> {
        let mut document = Document::new();

        // Set the appropriate interface ID.
        unsafe { *std::mem::transmute::<&mut Document, &mut InterfaceID>(&mut document) = Document::id(); }

        Dom::new(document)
    }

    // Returns the document element, if it exists.
    pub fn element(&self) -> Option<Dom<Element>> {
        let mut curr = self.first_child();
        while let Some(x) = curr {
            if x.is::<Element>() {
                #[cfg(debug_assertions)]
                {
                    let mut curr = x.next_sibling();
                    while let Some(x) = curr {
                        debug_assert!(!x.is::<Element>());
                        curr = x.next_sibling();
                    }
                }
                return Some(x.cast());
            }

            curr = x.next_sibling();
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests_init::hierarchy_init;

    #[test]
    fn document_with_no_element() {
        let document = Document::create();

        assert!(document.element().is_none());
    }

    #[test]
    fn document_with_single_element() {
        hierarchy_init();

        let mut document = Document::create();
        let element = Element::create();

        document.append(element.cast());

        assert!(document.element().is_some());
    }

    #[test]
    #[should_panic]
    fn document_with_multiple_elements() {
        hierarchy_init();

        let mut document = Document::create();
        let first_element = Element::create();
        let second_element = Element::create();

        document.append(first_element.cast());
        document.append(second_element.cast());

        assert!(document.element().is_some());
    }
}
