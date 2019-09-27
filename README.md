# intbits

Easy access to individual bits of integers

```rust
use intbits::Bits;

assert_eq!(2.bit(0), false);
assert_eq!(2.bit(1), true);
assert_eq!(2.bit(2), false);

assert_eq!(0b1011u32.bits(0..2), 0b11);
assert_eq!(0b1011u32.bits(2..4), 0b10);
```

See [the documentation](https://docs.rs/intbits).
