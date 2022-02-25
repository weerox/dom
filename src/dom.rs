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

impl<T> Clone for Dom<T> {
        fn clone(&self) -> Dom<T> {
            let dom = Dom {
                ptr: self.ptr.clone(),
            };

            let mut count = dom.meta().count.get();
            count += 1;
            dom.meta().count.set(count);
            debug_assert!(dom.meta().count.get() == self.meta().count.get());

            dom
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_increases_count() {
        let dom1 = Dom::new(1_u32);
        let dom2 = dom1.clone();

        assert_eq!(dom1.meta().count.get(), dom2.meta().count.get());
        assert_eq!(dom1.meta().count.get(), 2);
    }
}
