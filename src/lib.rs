mod dom;
mod cast;

pub use crate::dom::Dom;

pub use crate::cast::Cast;
pub use crate::cast::{Interface, InterfaceID};
pub use crate::cast::HIERARCHY;

pub fn init() {
    let mut hier = HIERARCHY.write().unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
