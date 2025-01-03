If we can't borrow multiple mutable references to the same object, how to we do shared memory multi-threading?
Do we have threading primitives in Rust?
Is there a particular way to ensure synchronized access to shared memory between threads?

How do we use enums?

How do lifetimes work?

What are deref coercions?

What are the different types of macro?

How does Option<T> and None work?

What does the *const Self::Elem mean here?

/// Array representation trait.
///
/// For an array that meets the invariants of the `ArrayBase` type. This trait
/// does not imply any ownership or lifetime; pointers to elements in the array
/// may not be safe to dereference.
///
/// ***Note:*** `RawData` is not an extension interface at this point.
/// Traits in Rust can serve many different roles. This trait is public because
/// it is used as a bound on public methods.
#[allow(clippy::missing_safety_doc)] // not implementable downstream
pub unsafe trait RawData: Sized {
    /// The array element type.
    type Elem;

    #[doc(hidden)]
    // This method is only used for debugging
    #[deprecated(note="Unused", since="0.15.2")]
    fn _data_slice(&self) -> Option<&[Self::Elem]>;

    #[doc(hidden)]
    fn _is_pointer_inbounds(&self, ptr: *const Self::Elem) -> bool;

    private_decl! {}
}

How does this private_decl! {} macro work?

//! The public parts of this private module are used to create traits
//! that cannot be implemented outside of our own crate.  This way we
//! can feel free to extend those traits without worrying about it
//! being a breaking change for other implementations.

/// If this type is pub but not publicly reachable, third parties
/// can't name it and can't implement traits using it.
pub struct PrivateMarker;

macro_rules! private_decl {
    () => {
        /// This trait is private to implement; this method exists to make it
        /// impossible to implement outside the crate.
        #[doc(hidden)]
        fn __private__(&self) -> crate::private::PrivateMarker;
    }
}

macro_rules! private_impl {
    () => {
        fn __private__(&self) -> crate::private::PrivateMarker {
            crate::private::PrivateMarker
        }
    };
}