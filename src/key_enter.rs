use crate::key_path::KeyPath;

/// A trait that provides the functions for entering the given type with a
/// [`KeyPath`].
pub trait KeyEnter: Sized {
    /// Enters `self` with the given [`KeyPath`], providing a reference to the
    /// field of the path.
    #[inline]
    fn enter<F>(&self, key_path: KeyPath<Self, F>) -> &F {
        // SAFETY: `offset` has to correctly point to the field.
        unsafe {
            let pointer = (self as *const Self).cast::<u8>();
            let field = pointer.add(key_path.offset).cast::<F>();
            &*field
        }
    }

    /// Enters `self` with the given [`KeyPath`], providing a mutable reference
    /// to the field of the path.
    #[inline]
    fn enter_mut<F>(&mut self, key_path: KeyPath<Self, F>) -> &mut F {
        // SAFETY: `offset` has to correctly point to the field.
        // Mutable access is also given.
        unsafe {
            let pointer = (self as *mut Self).cast::<u8>();
            let field = pointer.add(key_path.offset).cast::<F>();
            &mut *field
        }
    }
}

impl<T> KeyEnter for T {}
