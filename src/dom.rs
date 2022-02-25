use std::cell::Cell;
use std::ptr::NonNull;
use std::alloc::Layout;

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

impl<T> From<&T> for Dom<T> {
    fn from(reference: &T) -> Dom<T> {
        let layout = unsafe {
            Layout::from_size_align_unchecked(
                Layout::new::<DomMeta<()>>().size(),
                std::mem::align_of_val(reference)
            ).pad_to_align()
        };

        let byte_offset = layout.size() as isize;

        // SAFETY This is safe as long as the incoming &T came from an existing Dom<T>.
        let ptr: *mut DomMeta<T> = unsafe {
            (reference as *const T as *mut u8).offset(-byte_offset) as *mut DomMeta<T>
        };

        let dom = Dom {
            ptr: unsafe { NonNull::new_unchecked(ptr) },
        };

        let mut count = dom.meta().count.get();
        count += 1;
        dom.meta().count.set(count);

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

    #[test]
    fn from_ref_increases_count() {
        let dom = Dom::new(1_u32);
        let r = &dom.meta().value;
        let dom_from = Dom::from(r);
        assert_eq!(dom.meta().count.get(), dom_from.meta().count.get());
        assert_eq!(dom.meta().count.get(), 2);
    }
}
