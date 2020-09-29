#![no_std]

//! This crates provides two functions for accessing the individual bits of
//! integers:
//!
//! - [`.bit(i)`][Bits::bit] to get one specific bit.
//! - [`.bits(i..j)`][Bits::bits] to get a range of bits.
//!
//! It also provides similar functions for changing specific bits of integers:
//!
//! - [`.set_bit(i, bit)`][Bits::set_bit] to set one specific bit.
//! - [`.set_bits(i..j, bits)`][Bits::set_bits] to set a range of bits.
//!
//! These variants return a new integer, instead of modifying it:
//!
//! - [`.with_bit(i, bit)`][Bits::with_bit]
//! - [`.with_bits(i..j, bits)`][Bits::with_bits]
//!
//! # Example
//!
//! ```
//! use intbits::Bits;
//!
//! assert_eq!(2.bit(0), false);
//! assert_eq!(2.bit(1), true);
//! assert_eq!(2.bit(2), false);
//!
//! assert_eq!(0b1011u32.bits(0..2), 0b11);
//! assert_eq!(0b1011u32.bits(2..4), 0b10);
//!
//! assert_eq!(0xFFu8.with_bit(3, false), 0xF7);
//! assert_eq!(0xFFu8.with_bits(4..8, 3), 0x3F);
//! ```

use core::ops::RangeBounds;

/// Extension trait to provide access to individual bits of integers.
pub trait Bits {
	/// The (unsigned) type used to represent bits of this type.
	///
	/// For unsigned integers, this is `Self`.
	/// For signed integers, this is the unsigned variant of `Self`.
	///
	/// # Example
	///
	/// ```
	/// # use intbits::Bits;
	/// assert_eq!(0 as <u8 as Bits>::Bits, 0u8);
	/// assert_eq!(0 as <i64 as Bits>::Bits, 0u64);
	/// assert_eq!(0 as <usize as Bits>::Bits, 0usize);
	/// assert_eq!(0 as <isize as Bits>::Bits, 0usize);
	/// ```
	type Bits;

	/// The number of bits this type has.
	///
	/// # Example
	///
	/// ```
	/// # use intbits::Bits;
	/// assert_eq!(u8::N_BITS, 8);
	/// assert_eq!(i64::N_BITS, 64);
	/// ```
	const N_BITS: u32;

	/// Get a specific bit.
	///
	/// Panics if the index is out of range.
	///
	/// # Example
	///
	/// ```
	/// # use intbits::Bits;
	/// assert_eq!(2u8.bit(0), false);
	/// assert_eq!(2u8.bit(1), true);
	/// assert_eq!(2u8.bit(2), false);
	/// ```
	fn bit<I>(self, i: I) -> bool
	where
		I: BitsIndex<Self>,
		Self: Sized;

	/// Get a range of bits.
	///
	/// The bits are returned in the least significant bits of the return
	/// value. The other bits, if any, will be 0.
	///
	/// Empty ranges are allowed, and will result in 0.
	///
	/// Panics when the range bounds are out of range.
	///
	/// # Example
	///
	/// ```
	/// # use intbits::Bits;
	/// assert_eq!(0x45u8.bits(0..4), 5);
	/// assert_eq!(0x45u8.bits(4..8), 4);
	/// assert_eq!(0xF1u8.bits(1..), 0x78);
	/// assert_eq!(0xF1u8.bits(..7), 0x71);
	/// assert_eq!(0xF1u8.bits(8..), 0);
	/// assert_eq!(0xF1u8.bits(..0), 0);
	/// ```
	fn bits<I, R>(self, range: R) -> Self::Bits
	where
		I: BitsIndex<Self>,
		R: RangeBounds<I>,
		Self: Sized;

	/// Set a specific bit.
	///
	/// Panics if the index is out of range.
	///
	/// # Example
	///
	/// ```
	/// # use intbits::Bits;
	/// let mut a = 0xFFu8;
	/// a.set_bit(3, false);
	/// assert_eq!(a, 0xF7);
	/// ```
	fn set_bit<I>(&mut self, i: I, bit: bool)
	where
		I: BitsIndex<Self>,
		Self: Sized;

	/// Set a range of bits.
	///
	/// The bits should be given in the least significant bits of the second
	/// argument. The other bits should be 0.
	///
	/// Panics when the range bounds are out of range or when the irrelevant
	/// bits of the second argument are not 0.
	///
	/// # Example
	///
	/// ```
	/// # use intbits::Bits;
	/// let mut a = 0xFFu8;
	/// a.set_bits(4..8, 3);
	/// assert_eq!(a, 0x3F);
	/// ```
	fn set_bits<I, R>(&mut self, range: R, bits: Self::Bits)
	where
		I: BitsIndex<Self>,
		R: RangeBounds<I>,
		Self: Sized;

	/// Get a new integer with one bit set to a specific value.
	///
	/// Panics if the index is out of range.
	///
	/// # Example
	///
	/// ```
	/// # use intbits::Bits;
	/// assert_eq!(0xFFu8.with_bit(3, false), 0xF7);
	/// ```
	fn with_bit<I>(self, i: I, bit: bool) -> Self
	where
		I: BitsIndex<Self>,
		Self: Sized;

	/// Get a new integer with a range of bits set to specific values.
	///
	/// The bits should be given in the least significant bits of the second
	/// argument. The other bits should be 0.
	///
	/// Panics when the range bounds are out of range or when the irrelevant
	/// bits of the second argument are not 0.
	///
	/// # Example
	///
	/// ```
	/// # use intbits::Bits;
	/// assert_eq!(0xFFu8.with_bits(4..8, 3), 0x3F);
	/// ```
	fn with_bits<I, R>(self, range: R, bits: Self::Bits) -> Self
	where
		I: BitsIndex<Self>,
		R: RangeBounds<I>,
		Self: Sized;
}

/// Trait for types that can be used to index the bits of `T`.
pub trait BitsIndex<T> {
	/// See [`Bits::bit`].
	fn bit(value: T, index: Self) -> bool;
	/// See [`Bits::bits`].
	fn bits<R>(value: T, range: R) -> <T as Bits>::Bits
	where
		T: Bits,
		R: RangeBounds<Self>;
	/// See [`Bits::set_bit`].
	fn set_bit(value: &mut T, index: Self, bit: bool);
	/// See [`Bits::set_bits`].
	fn set_bits<R>(value: &mut T, range: R, bits: <T as Bits>::Bits)
	where
		T: Bits,
		R: RangeBounds<Self>;
}

mod impls;

#[cfg(test)]
mod test;
