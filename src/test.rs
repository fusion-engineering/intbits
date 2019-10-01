use super::Bits;
use core::ops::Bound;

#[test]
fn test_get() {
	assert_eq!(2u32.bit(0), false);
	assert_eq!(2u32.bit(1), true);
	assert_eq!(2u32.bit(2), false);
	assert_eq!(123u32.bits(0..0), 0);
	assert_eq!(255u32.bits(0..8), 255);
	assert_eq!(255u32.bits(0..9), 255);
	assert_eq!(255u32.bits(0..7), 127);
	assert_eq!(1234u32.bits(32..), 0);
	assert_eq!(1234u32.bits(..32), 1234);
	assert_eq!(1234u32.bits(0..32), 1234);
	assert_eq!(123u32.bits(0..=0), 1);
	assert_eq!(123u32.bits(0..=-1), 0);
	assert_eq!(1234u32.bits(0..=31), 1234);
	assert_eq!((-1i32).bits(0..), !0);
	assert_eq!((-1i32).bits(1..), !0 >> 1);
	assert_eq!(0xFFu32.bits((Bound::Excluded(1), Bound::Included(1))), 0);
	assert_eq!((!0u32).bits((Bound::Excluded(31), Bound::Included(31))), 0);
	assert_eq!((!0u32).bits((Bound::Excluded(-1), Bound::Included(31))), !0);
	assert_eq!(
		0x555u32.bits((Bound::Excluded(1), Bound::Included(7))),
		0x15
	);
	assert_eq!(
		0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAu128.bits(0..128),
		0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
	);
	assert_eq!(
		0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAu128.bits(0i8..=127i8),
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
	assert_eq!(0xFFu8.with_bits(4.., 2), 0x2F);
	assert_eq!(0xFFu8.with_bits(0..4, 2), 0xF2);
	assert_eq!(0xFFu8.with_bits(..4, 2), 0xF2);
	assert_eq!(0xFFu8.with_bits(8..8, 0), 0xFF);
	assert_eq!(0xFFu8.with_bits(8.., 0), 0xFF);
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

#[test]
#[should_panic(expected = "invalid bit index")]
fn test_get_panic_1() {
	123u32.bit(32);
}

#[test]
#[should_panic(expected = "invalid bit index")]
fn test_get_panic_2() {
	123u32.bit(-1);
}

#[test]
#[should_panic(expected = "invalid bit range")]
fn test_get_panic_3() {
	123u32.bits(-1..);
}

#[test]
#[should_panic(expected = "invalid bit range")]
fn test_get_panic_4() {
	123u32.bits(33..);
}

#[test]
#[should_panic(expected = "invalid bit range")]
fn test_get_panic_5() {
	123u32.bits(4294967295u32..);
}

#[test]
#[should_panic(expected = "invalid bit range")]
fn test_get_panic_6() {
	123u32.bits(0x10000000000000000000000000000000u128..);
}

#[test]
#[should_panic(expected = "invalid bit range")]
fn test_get_panic_7() {
	123u32.bits((Bound::Included(-2), Bound::Unbounded));
}

#[test]
#[should_panic(expected = "invalid bit range")]
fn test_get_panic_8() {
	123u32.bits((
		Bound::Included(-0x10000000000000000000000000000000i128),
		Bound::Unbounded,
	));
}

#[test]
#[should_panic(expected = "invalid bit range")]
fn test_get_panic_9() {
	123u128.bits(-128i8..);
}

#[test]
#[should_panic(expected = "invalid bit range")]
fn test_get_panic_10() {
	123u32.bits(..33);
}

#[test]
#[should_panic(expected = "invalid bit range")]
fn test_get_panic_11() {
	123u32.bits(..-1);
}

#[test]
#[should_panic(expected = "bits outside range")]
fn test_set_panic() {
	123u32.with_bits(4..8, 0x10);
}
