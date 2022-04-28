mod node;
mod document;
mod element;

pub use node::Node;
pub use document::Document;
pub use element::Element;

// An interface is represented by a struct which has the methods of the
// interface implemented on it. Each interface must be uniquely identified by
// an ID. Inheritance is accomplished through struct composition and casting is
// attained by keeping track of the inheritance hierarchy of all interfaces and
// storing the ID of the top-most interface in the base interface.
//
// When creating an interface, let's call if `Foo`, you must follow these steps:
// 1.  Create the struct `Foo`, optionally containing relevant fields.
// 2a. If the interface `Foo` inherits from another interface `Bar`, then the
//     FIRST field in the struct `Foo` MUST be a `Bar` struct. To make the
//     purpose of the field clear, it SHOULD be named `_inherited`.
// 2b. If the interface `Foo` does not inherit from another interface, i.e. it
//     is a base interface, then the FIRST field of the struct `Foo` MUST be a
//     `InterfaceID`. This ID will be set to the ID of the top-most interface.
//     To make the purpose of the field clear, it SHOULD be named `_top'.
// 3.  Mark the struct `Foo` as `#[repr(C)]` to make sure that the field
//     created in 2a or 2b is actually stored as the first field in memory.
// 4.  Implement the `id` function of the `Interface` trait for `Foo`. The ID
//     that it returns MUST be unique among all implemented interfaces.
// 5.  Add a hierarchy registration call in the `init` method in the crate root.
