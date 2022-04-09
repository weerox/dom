use std::collections::HashMap;
use std::num::NonZeroU32;
use std::sync::RwLock;

use once_cell::sync::Lazy;

use crate::Dom;

// There has to be support for casting between the inherited interfaces.
//
// This is useful if you have any interface that belong to the tree and you
// want to retrieve its next sibling. Since the tree only operates on `Node`
// interfaces you will get a `Node` as the return type. This type has to be
// cast to an appropriate subtype (or supertype).
// 
// An example of an inheritance chain, where `Node` is the base:
// Node - Element - HTMLElement - HTMLParagraphElement
// 
// Inheritance is facilitated through composition, i.e. each interface is
// represented by a struct where the first field is the struct of the interface
// it inherits from.
// The struct for the `HTMLParagraphElement` interface is defined as
// ```
// struct HTMLParagraphElement {
//     element: HTMLElement,
//     ...
// }
// ```
//
// Since this will be inlined, we get the nice property that a pointer to any
// interface, `HTMLParagraphElement` in this case, is also a valid pointer to
// all of its supertypes. This makes upcasting somewhat easy to implement.
//
// Downcasting is bit more tricky. In the example above, when we get a `Node`
// from the tree, how do we know what subtype it originated from?
// Is it a `HTMLParagraphElement` or a `HTMLAnchorElement`?
// This can be solved by requiring the base (e.g. `Node` in the chain above)
// to have an ID as its first field (which consequently will also make it
// the first field of any of its subtypes). The ID represents the topmost
// subtype of the chain. We then map each interface ID to the ID of the
// interface's supertype using a HashMap, which means that we can extract
// the whole intheritance chain from this single topmost interface ID.

pub type InterfaceID = NonZeroU32;

// A convenience function to calculate if the interface whose ID is `top`
// has a supertype whose ID is `sought`.
// TODO This can probably be inserted into Interface::is()
fn is(top: InterfaceID, sought: InterfaceID) -> bool {
    let mut curr = Some(top);

    while let Some(id) = curr {
        if id == sought {
            break;
        }

        curr = HIERARCHY.read().unwrap().inherits_from(id);
    }

    curr.is_some()
}

// This trait MUST be implemented for each DOM interface.
pub trait Interface {
    // Simply gives the ID of the interface it is implemented for.
    fn id() -> InterfaceID;

    fn is<U: Interface>(&self) -> bool {
        // The base interface, which will be the innermost struct,
        // MUST have the ID of the topmost interface as its first field.
        let top_id = unsafe { *(self as *const Self as *const u32) };
        let top_id = NonZeroU32::new(top_id)
            .unwrap_or_else(|| panic!("The stored ID of the topmost interface was 0"));
        let sought_id = U::id();
        is(top_id, sought_id)
    }
}

// Keeps track of the interface hierarchy by mapping an interface ID to the ID
// of the interface it inherits from.
struct Hierarchy {
    map: HashMap<InterfaceID, Option<InterfaceID>>,
}

impl Hierarchy {
    fn inherits_from(&self, id: InterfaceID) -> Option<InterfaceID> {
        *self.map.get(&id)
            .unwrap_or_else(|| panic!("No interface with ID {} has been registered to the interface hierarchy", id))
    }

    // NOTE Hashmap::insert() returns the old value if the key was already
    //      present, but it isn't checked or handled. Should it be? I don't
    //      think that a mapping should ever have to be updated, so it might
    //      be a good idea to check if that happens, because it is probably
    //      an error.
    fn register(&mut self, interface: InterfaceID, inherited: Option<InterfaceID>) {
        let res = self.map.insert(interface, inherited);
        debug_assert!(res.is_none());
    }

    // The test could will do repeated initialization, so we have to introduce
    // the ability to clear the map when testing.
    #[cfg(test)]
    fn clear(&mut self) {
        self.map.clear();
    }
}

static HIERARCHY: Lazy<RwLock<Hierarchy>> = Lazy::new(|| {
    RwLock::new(Hierarchy {
        map: HashMap::new(),
    })
});

// This trait is implemented on types that contains interfaces, e.g. &T and
// Dom<T>, so that we can cast the contained interface.
// TODO The `cast` function should probably return an `Option` so it doesn't
//      have to panic if the cast wasn't possible to perform.
//      The `cast` could also be split into `upcast` and `downcast`. Upcasting
//      could then be statically checked using a trait that indicates that an
//      interface inherits from another interface.
pub trait Cast<T: Interface, U: Interface> {
    // The result of casting from T to U.
    type Res;

    fn cast(self) -> Self::Res;
}

impl<T: Interface, U: Interface> Cast<T, U> for Dom<T> {
    type Res = Dom<U>;

    fn cast(self) -> Self::Res {
        if self.is::<U>() {
            unsafe { std::mem::transmute::<Dom<T>, Dom<U>>(self) }
        } else {
            panic!();
        }
    }
}

impl<'a, T: Interface, U: 'a + Interface> Cast<T, U> for &'a T {
    type Res = &'a U;

    fn cast(self) -> Self::Res {
        if self.is::<U>() {
            unsafe { &*(self as *const T as *const U) }
        } else {
            panic!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct InterfaceA(InterfaceB);
    struct InterfaceB(InterfaceID, u32);

    impl Interface for InterfaceA {
        fn id() -> InterfaceID {
            NonZeroU32::new(13).unwrap()
        }
    }

    impl Interface for InterfaceB {
        fn id() -> InterfaceID {
            NonZeroU32::new(14).unwrap()
        }
    }

    // Each test function should call this initialization.
    fn interface_init() {
        // We have to clear the hierarchy first, since this function will be
        // called multiple times in the same binary, which would result in a
        // panic because we are trying to insert an InterfaceID that already
        // exist.
        let mut hier = HIERARCHY.write().unwrap();
        hier.clear();
        hier.register(InterfaceA::id(), Some(InterfaceB::id()));
        hier.register(InterfaceB::id(), None);
    }

    #[test]
    fn simple_cast() {
        interface_init();
        let a = Dom::new(InterfaceA(InterfaceB(InterfaceA::id(), 35)));
        let b: Dom<InterfaceB> = a.clone().cast();

        assert!(b.1 == 35);
    }

    #[test]
    fn simple_is_upcast() {
        interface_init(); 
        let a = Dom::new(InterfaceA(InterfaceB(InterfaceA::id(), 35)));

        assert!(a.is::<InterfaceB>());
    }

    #[test]
    fn simple_is_downcast() {
        interface_init(); 
        let a = Dom::new(InterfaceA(InterfaceB(InterfaceA::id(), 35)));
        let b: Dom<InterfaceB> = a.cast();

        assert!(b.is::<InterfaceA>());
    }
}
