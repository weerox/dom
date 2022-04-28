use crate::interface::Node;
use crate::{Interface, InterfaceID};

#[repr(C)]
pub struct Document {
    _inherited: Node,
}

impl Interface for Document {
    fn id() -> InterfaceID {
        InterfaceID::new(2)
    }
}
