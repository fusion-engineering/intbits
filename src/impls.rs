use super::{Bits, BitsIndex};
use core::ops::{Bound, RangeBounds};
use core::ops::Bound::{Excluded, Included, Unbounded};

trait BitRange<T>: Bits {
	fn mask(end: Bound<&T>) -> Self::Bits;
	fn shift(end: Bound<&T>) -> Option<T>;
}

macro_rules! bits {
	($t:tt, $ut:tt, $n:tt, $i:tt) => {
		#[allow(unused_comparisons)]
		impl BitRange<$i> for $t {
			#[inline]
			fn mask(end: Bound<&$i>) -> $ut {
				match end {
					Unbounded => !0,
					Excluded(&i) if i > $n && i - 1 == $n => !0,
					Excluded(&i) if i <= $n && i >= 0 => !0 >> 1 >> ($n - i),
					Included(&i) if i < 0 && i + 1 == 0 => 0,
					Included(&i) if i <= $n && i >= 0 => !0 >> ($n - i),
					_ => panic!("invalid bit range"),
				}
			}

			#[inline]
			fn shift(start: Bound<&$i>) -> Option<$i> {
				match start {
					Unbounded => Some(0),
					Included(&i) if i > $n && i - 1 == $n => None,
					Included(&i) if i <= $n && i >= 0 => Some(i),
					Excluded(&i) if i == $n => None,
					Excluded(&i) if i < $n && i + 1 >= 0 => Some(i + 1),
					_ => panic!("invalid bit range"),
				}
			}
		}

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
				let mask = $t::mask(range.end_bound());
				if let Some(shift) = $t::shift(range.start_bound()) {
					(v as $ut & mask) >> shift
				} else {
					0
				}
			}

			#[inline]
			fn set_bit(v: &mut $t, i: Self, bit: bool) {
				assert!(i >= 0 && i <= $n, "invalid bit index");
				*v = *v & !(1 << i) | (bit as $t) << i;
			}

			#[inline]
			fn set_bits<R>(v: &mut $t, range: R, bits: $ut)
			where
				R: RangeBounds<Self>,
			{
				let mask = $t::mask(range.end_bound());
				if let Some(shift) = $t::shift(range.start_bound()) {
					let and_mask = !(mask & !0 << shift);
					let or_mask = bits << shift;
					if or_mask & and_mask != 0 {
						panic!("bits outside range");
					}
					*v = *v & and_mask as $t | or_mask as $t;
				}
			}
		}
	};
	($t:tt, $ut:tt, $n:tt) => {
		impl Bits for $t {
			type Bits = $ut;
			const N_BITS: usize = $n + 1;
			#[inline]
			fn bit<I>(self, i: I) -> bool
			where
				I: BitsIndex<Self>,
			{
				I::bit(self, i)
			}
			#[inline]
			fn bits<I, R>(self, range: R) -> $ut
			where
				I: BitsIndex<Self>,
				R: RangeBounds<I>,
			{
				I::bits(self, range)
			}
			#[inline]
			fn set_bit<I>(&mut self, i: I, bit: bool)
			where
				I: BitsIndex<Self>,
			{
				I::set_bit(self, i, bit)
			}
			#[inline]
			fn set_bits<I, R>(&mut self, range: R, bits: $ut)
			where
				I: BitsIndex<Self>,
				R: RangeBounds<I>,
			{
				I::set_bits(self, range, bits)
			}
			#[inline]
			fn with_bit<I>(mut self, i: I, bit: bool) -> Self
			where
				I: BitsIndex<Self>,
			{
				I::set_bit(&mut self, i, bit);
				self
			}
			#[inline]
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
