use libc::size_t;

/// Common interface for all C++ vector types generated by the crate
///
/// You'll need to import this trait to use any of the C++ vector wrappers, usually imported as
/// part of the prelude.
pub trait VectorTrait<'i>: Sized {
	type Arg;

	fn with_capacity(capacity: size_t) -> Self;

	/// Create a Vector from iterator
	fn from_iter(s: impl IntoIterator<Item=Self::Arg>) -> Self {
		let s = s.into_iter();
		let (lo, hi) = s.size_hint();
		let mut out = Self::with_capacity(hi.unwrap_or(lo));
		s.for_each(|x| out.push(x));
		out
	}

	/// Add new element
	fn push(&mut self, val: Self::Arg);

	/// Insert a new element at the specified `index`
	fn insert(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()>;

	/// Set element at the specified `index`
	fn set(&mut self, index: size_t, val: Self::Arg) -> crate::Result<()>;

	/// Same as `set()` but without bounds checking
	unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg);
}
