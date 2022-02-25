use std::cell::Cell;
use std::ptr::NonNull;

struct DomMeta<T> {
    count: Cell<usize>,
    value: T,
}

pub struct Dom<T> {
    ptr: NonNull<DomMeta<T>>,
}

impl<T> Dom<T> {
    fn meta(&self) -> &DomMeta<T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Dom<T> {
    pub fn new(value: T) -> Dom<T> {
        let meta = DomMeta {
            count: Cell::new(1),
            value: value,
        };

        Dom {
            ptr: NonNull::from(Box::leak(Box::new(meta))),
        }
    }
}
