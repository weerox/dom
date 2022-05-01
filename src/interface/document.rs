use crate::interface::Node;
use crate::Dom;
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
}
