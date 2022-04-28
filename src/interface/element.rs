use crate::interface::Node;
use crate::{Interface, InterfaceID};

#[repr(C)]
pub struct Element {
    _inherited: Node,
}

impl Interface for Element {
    fn id() -> InterfaceID {
        InterfaceID::new(3)
    }
}
