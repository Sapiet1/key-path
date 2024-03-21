use std::marker::PhantomData;

/// A path from struct `V` to its field of type `F`.
pub struct KeyPath<V, F> {
    pub(crate) offset: usize,
    _value: PhantomData<fn() -> V>,
    _field: PhantomData<fn() -> F>,
}

impl<V, F> KeyPath<V, F> {
    /// Constructs a new [`KeyPath`] with the given offset to the expected
    /// field of type `F`. There are two invariants:
    ///
    /// 1. The offset is accurately calculated. This can be given through the
    /// [`std::mem::offset_of`] macro. Notably, this also suggests that one
    /// should not store the [`KeyPath`] for different runs of the program;
    /// doing so will result in Undefined Behavior.
    ///
    /// 2. The offset does indeed point to a field of type `F`. This is not
    /// checked from within the function and has to be correctly done.
    ///
    /// Both invariants are held within the provided [`crate::key`] macro, which is
    /// the recommended way of constructing a [`KeyPath`].
    #[inline]
    pub const unsafe fn new(offset: usize) -> KeyPath<V, F> {
        KeyPath {
            offset,
            _value: PhantomData,
            _field: PhantomData,
        }
    }

    #[doc(hidden)]
    #[inline]
    // A `new` method containing type-hints for `F`. This is intended for usage
    // in the `key` macro.
    pub const unsafe fn new_hinted(offset: usize, _: *const F) -> KeyPath<V, F> {
        KeyPath::new(offset)
    }

    /// Appends one [`KeyPath`] onto another [`KeyPath`], creating a new path
    /// towards the field of type `T` from `V` (skips `F` in `V` -> `F` -> `T`).
    pub fn append<T>(self, other: KeyPath<F, T>) -> KeyPath<V, T> {
        KeyPath {
            offset: self.offset + other.offset,
            _value: PhantomData,
            _field: PhantomData,
        }
    }
}

impl<V, F> Clone for KeyPath<V, F> {
    fn clone(&self) -> Self {
        KeyPath {
            offset: self.offset,
            _value: PhantomData,
            _field: PhantomData,
        }
    }
}

impl<V, F> Copy for KeyPath<V, F> {}

/// Constructs a [`KeyPath`] conforming to the invariants written in
/// [`KeyPath::new`].
#[macro_export]
macro_rules! key {
    // TODO: This relies on type-inference, so although (almost) everything can
    //  be done in `const`, it is unable to be clearly marked as such currently.
    // When inline-const is stablized, change it to an `unsafe const` block.
    // TODO: When `std::mem::offset_of!` is able to do nesting, switch to it.
    // Currently, casting to integers blocks `const` usage.
    ($value:ty$([$field:tt])+) => {
        // SAFETY: `offset` is accurately calculated and the types are checked.
        unsafe {
            let empty_value = std::mem::MaybeUninit::<$value>::uninit();

            let start = empty_value.as_ptr();
            let end = std::ptr::addr_of!((*start)$(.$field)+);

            let offset = end as usize - start as usize;

            $crate::KeyPath::<$value, _>::new_hinted(offset, end)
        }
    };
}
