use std::ptr::NonNull;

struct DomMeta<T> {
    count: usize,
    value: T,
}

pub struct Dom<T> {
    ptr: NonNull<DomMeta<T>>,
}

impl<T> Dom<T> {
    pub fn new(value: T) -> Dom<T> {
        let meta = DomMeta {
            count: 1,
            value: value,
        };

        Dom {
            ptr: NonNull::from(Box::leak(Box::new(meta))),
        }
    }
}
