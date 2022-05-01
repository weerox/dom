use crate::interface::Node;
use crate::{Interface, InterfaceID};

use std::ops::{Deref, DerefMut};

#[repr(C)]
pub struct Element {
    _inherited: Node,
}

impl Interface for Element {
    fn id() -> InterfaceID {
        InterfaceID::new(3)
    }
}

impl Deref for Element {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self._inherited
    }
}

impl DerefMut for Element {
    fn deref_mut(&mut self)  -> &mut Self::Target {
        &mut self._inherited
    }
}
