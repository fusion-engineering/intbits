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

use core::ops::{Bound, RangeBounds};

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
	const N_BITS: usize;

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

macro_rules! bits {
	($t:tt, $ut:tt, $n:tt, $i:tt) => {
		#[allow(unused_comparisons)]
		impl BitsIndex<$t> for $i {
			#[inline]
			fn bit(v: $t, i: Self) -> bool {
				assert!(i >= 0 && i <= $n, "invalid bit index");
				v >> i & 1 != 0
			}

			#[inline]
			fn bits<R>(v: $t, range: R) -> $ut
			where
				R: RangeBounds<Self>,
			{
				use Bound::{Excluded, Included, Unbounded};

				let v = match range.end_bound() {
					Unbounded => v,
					Included(&i) if i.checked_add(1) == Some(0) => 0,
					Excluded(&i) if i.checked_sub(1) == Some($n) => v,
					Included(&i) if i <= $n && i >= 0 => v & (!0 >> ($n - i)),
					Excluded(&i) if i <= $n && i >= 0 => v & (!0 >> 1 >> ($n - i)),
					_ => panic!("invalid bit range"),
				} as $ut;

				match range.start_bound() {
					Unbounded => v,
					Included(&i) if i.checked_sub(1) == Some($n) => 0,
					Excluded(&i) if i.checked_add(1) == Some(0) => v,
					Included(&i) if i <= $n && i >= 0 => v >> i,
					Excluded(&i) if i <= $n && i >= 0 => v >> 1 >> i,
					_ => panic!("invalid bit range"),
				}
			}

			#[inline]
			fn set_bit(v: &mut $t, i: Self, bit: bool) {
				assert!(i >= 0 && i <= $n, "invalid bit index");
				*v &= !(1 << i);
				*v |= (bit as $t) << i;
			}

			#[inline]
			fn set_bits<R>(v: &mut $t, range: R, bits: $ut)
			where
				R: RangeBounds<Self>,
			{
				use Bound::{Excluded, Included, Unbounded};

				let mask1 = match range.end_bound() {
					Unbounded => !0,
					Included(&i) if i.checked_add(1) == Some(0) => 0,
					Excluded(&i) if i.checked_sub(1) == Some($n) => !0,
					Included(&i) if i <= $n && i >= 0 => !0 >> ($n - i),
					Excluded(&i) if i <= $n && i >= 0 => !0 >> 1 >> ($n - i),
					_ => panic!("invalid bit range"),
				};

				let (mask2, shift) = match range.start_bound() {
					Unbounded => (!0, 0),
					Included(&i) if i.checked_sub(1) == Some($n) => (0, 0),
					Excluded(&i) if i.checked_add(1) == Some(0) => (!0, 0),
					Included(&i) if i <= $n && i >= 0 => (!0 << i, i),
					Excluded(&i) if i <= $n && i >= 0 => (!0 << 1 << i, i.wrapping_add(1) & $n),
					_ => panic!("invalid bit range"),
				};

				let and_mask = !(mask1 & mask2);
				let or_mask = bits << shift;

				if or_mask & and_mask != 0 {
					panic!("bits outside range");
				}

				*v &= and_mask as $t;
				*v |= or_mask as $t;
			}
		}
	};
	($t:tt, $ut:tt, $n:tt) => {
		impl Bits for $t {
			type Bits = $ut;
			const N_BITS: usize = $n + 1;
			fn bit<I>(self, i: I) -> bool
			where
				I: BitsIndex<Self>,
			{
				I::bit(self, i)
			}
			fn bits<I, R>(self, range: R) -> $ut
			where
				I: BitsIndex<Self>,
				R: RangeBounds<I>,
			{
				I::bits(self, range)
			}
			fn set_bit<I>(&mut self, i: I, bit: bool)
			where
				I: BitsIndex<Self>,
			{
				I::set_bit(self, i, bit)
			}
			fn set_bits<I, R>(&mut self, range: R, bits: $ut)
			where
				I: BitsIndex<Self>,
				R: RangeBounds<I>,
			{
				I::set_bits(self, range, bits)
			}
			fn with_bit<I>(mut self, i: I, bit: bool) -> Self
			where
				I: BitsIndex<Self>,
			{
				I::set_bit(&mut self, i, bit);
				self
			}
			fn with_bits<I, R>(mut self, range: R, bits: $ut) -> Self
			where
				I: BitsIndex<Self>,
				R: RangeBounds<I>,
			{
				I::set_bits(&mut self, range, bits);
				self
			}
		}
		bits!($t, $ut, $n, i8);
		bits!($t, $ut, $n, u8);
		bits!($t, $ut, $n, i16);
		bits!($t, $ut, $n, u16);
		bits!($t, $ut, $n, i32);
		bits!($t, $ut, $n, u32);
		bits!($t, $ut, $n, i64);
		bits!($t, $ut, $n, u64);
		bits!($t, $ut, $n, i128);
		bits!($t, $ut, $n, u128);
		bits!($t, $ut, $n, isize);
		bits!($t, $ut, $n, usize);
	};
}

bits!(i8, u8, 7);
bits!(u8, u8, 7);
bits!(i16, u16, 15);
bits!(u16, u16, 15);
bits!(i32, u32, 31);
bits!(u32, u32, 31);
bits!(i64, u64, 63);
bits!(u64, u64, 63);
bits!(i128, u128, 127);
bits!(u128, u128, 127);

#[cfg(target_pointer_width = "32")]
bits!(isize, usize, 31);

#[cfg(target_pointer_width = "32")]
bits!(usize, usize, 31);

#[cfg(target_pointer_width = "64")]
bits!(isize, usize, 63);

#[cfg(target_pointer_width = "64")]
bits!(usize, usize, 63);

#[test]
fn test_get() {
	assert_eq!(123u32.bits(0..0), 0);
	assert_eq!(255u32.bits(0..8), 255);
	assert_eq!(255u32.bits(0..9), 255);
	assert_eq!(255u32.bits(0..7), 127);
	assert_eq!(1234u32.bits(0..32), 1234);
	assert_eq!(123u32.bits(0..=0), 1);
	assert_eq!(123u32.bits(0..=-1), 0);
	assert_eq!(1234u32.bits(0..=31), 1234);
	assert_eq!((-1i32).bits(0..32), !0);
	assert_eq!((-1i32).bits(1..32), !0 >> 1);
	assert_eq!(
		0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAu128.bits(0..128),
		0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
	);
	assert_eq!(
		0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAu128.bits(64..),
		0xAAAAAAAAAAAAAAAA
	);
	assert_eq!(0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAu128.bits(127..), 1);
}

#[test]
fn test_set() {
	assert_eq!(0xFFu8.with_bits(4..8, 2), 0x2F);
	assert_eq!(0xFFu8.with_bits(0..4, 2), 0xF2);
	assert_eq!(0u32.with_bits(5..9, 0xF), 0b111100000);
	assert_eq!(
		0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAu128
			.with_bit(127, false)
			.with_bit(126, true),
		0x6AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
	);
	assert_eq!(
		0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAu128.with_bits(126..128, 1),
		0x6AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
	);
}
