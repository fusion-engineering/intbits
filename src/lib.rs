#![no_std]

use core::ops::{Bound, RangeBounds};

pub trait IntBit {
	type Bits;
	fn bit<I>(self, i: I) -> bool
	where
		I: BitIndex<Self>,
		Self: Sized;
	fn bits<I, R>(self, range: R) -> Self::Bits
	where
		I: BitIndex<Self>,
		R: RangeBounds<I>,
		Self: Sized;
}

pub trait BitIndex<T> {
	fn bit(value: T, index: Self) -> bool;
	fn bits<R>(value: T, range: R) -> <T as IntBit>::Bits
	where
		T: IntBit,
		R: RangeBounds<Self>;
}

macro_rules! int_bit {
	($t:tt, $ut:tt, $n:tt, $i:tt) => {
		#[allow(unused_comparisons)]
		impl BitIndex<$t> for $i {
			#[inline]
			fn bit(v: $t, i: Self) -> bool {
				assert!(i >= 0 && i <= $n, "Invalid bit index.");
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
		}
	};
	($t:tt, $ut:tt, $n:tt) => {
		impl IntBit for $t {
			type Bits = $ut;
			fn bit<I>(self, i: I) -> bool
			where
				I: BitIndex<Self>,
			{
				I::bit(self, i)
			}
			fn bits<I, R>(self, range: R) -> $ut
			where
				I: BitIndex<Self>,
				R: RangeBounds<I>,
			{
				I::bits(self, range)
			}
		}
		int_bit!($t, $ut, $n, i8);
		int_bit!($t, $ut, $n, u8);
		int_bit!($t, $ut, $n, i16);
		int_bit!($t, $ut, $n, u16);
		int_bit!($t, $ut, $n, i32);
		int_bit!($t, $ut, $n, u32);
		int_bit!($t, $ut, $n, i64);
		int_bit!($t, $ut, $n, u64);
		int_bit!($t, $ut, $n, i128);
		int_bit!($t, $ut, $n, u128);
		int_bit!($t, $ut, $n, isize);
		int_bit!($t, $ut, $n, usize);
	};
}

int_bit!(i8, u8, 7);
int_bit!(u8, u8, 7);
int_bit!(i16, u16, 15);
int_bit!(u16, u16, 15);
int_bit!(i32, u32, 31);
int_bit!(u32, u32, 31);
int_bit!(i64, u64, 63);
int_bit!(u64, u64, 63);
int_bit!(i128, u128, 127);
int_bit!(u128, u128, 127);

#[cfg(target_pointer_width = "32")]
int_bit!(isize, usize, 31);

#[cfg(target_pointer_width = "32")]
int_bit!(usize, usize, 31);

#[cfg(target_pointer_width = "64")]
int_bit!(isize, usize, 63);

#[cfg(target_pointer_width = "64")]
int_bit!(usize, usize, 63);

#[test]
fn test() {
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
}
