use std::cell::Cell;
use std::ptr::NonNull;
use std::alloc::Layout;
use std::ops::{Deref, DerefMut};

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

impl<T> Deref for Dom<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.ptr.as_ref().value }
    }
}

// SAFETY We have to mutably borrow the Dom<T> to be able to aquire
//        a mutable reference to the underlying data T.
//        This means that the normal borrowing rules will apply as long as
//        we aquire the references through the same Dom<T>,
//        see multi_mut_same_dom() and ref_and_mut_same_dom().
//        If we have two Dom<T> copies that point to the same value T,
//        then it is possible to circumvent the borrowing rules,
//        e.g. to aquire two &mut T to the same value T, see test3().
//        In practice, it would be quite unlikely to have to use two copies
//        of a Dom<T> at the same time.
// NOTE I will leave the Deref and DerefMut impls as is and later evaluate
//      whether there is a need for stricter control that guarantees
//      the XOR rule.
impl<T> DerefMut for Dom<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.ptr.as_mut().value }
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

    #[test]
    fn multi_mut_same_dom() {
        let mut dom = Dom::new(1234_u32);

        #[allow(unused_variables)]
        let x: &mut u32 = dom.deref_mut();
        let y: &mut u32 = dom.deref_mut();

        // NOTE As expected, we can't uncomment this,
        // because that would require us to have two
        // &mut u32 through the same Dom<u32>,
        // which isn't possible.
        //let a = *x;
        let b = *y;

        assert_eq!(b, 1234);
    }

    #[test]
    fn ref_and_mut_same_dom() {
        let mut dom = Dom::new(1234_u32);

        #[allow(unused_variables)]
        let x: &u32 = dom.deref();
        let y: &mut u32 = dom.deref_mut();

        // NOTE As expected, we can't uncomment this,
        // because that would require us to have both a
        // &u32 and a &mut u32 through the same Dom<u32>,
        // which isn't possible.
        //let a = *x;
        let b = *y;

        assert_eq!(b, 1234);
    }

    #[test]
    fn multi_mut_different_dom() {
        let mut dom1 = Dom::new(1234_u32);
        let mut dom2 = dom1.clone();

        // NOTE This is possible because
        // the different references are aquired through
        // different Dom<u32> copies.
        let x: &mut u32 = dom1.deref_mut();
        let y: &mut u32 = dom2.deref_mut();

        let a = *x;
        let b = *y;

        assert_eq!(a, b);
    }
}
