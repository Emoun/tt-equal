tt-equal
=============================

[![Build Status](https://api.travis-ci.org/Emoun/tt-equal.svg?branch=master)](https://travis-ci.org/Emoun/tt-equal)
[![Latest Version](https://img.shields.io/crates/v/tt-equal.svg)](https://crates.io/crates/tt-equal)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/tt-equal/0.1/tt_equal/)

**This library is an entry in the [`tt_call`](https://crates.io/crates/tt-call) series of modular interoperable tt-muncher building blocks.**

Inludes the procedural macro `tt_equal` that acts as a predicate for whether two token trees are equal. 
Intended for use with [`tt_if`](https://docs.rs/tt-call/1.0.6/tt_call/macro.tt_if.html).

```rust
use tt_equal::tt_equal;
use tt_call::tt_if;

macro_rules! same_ident{
    {
        $id1:ident, $id2:ident
    } => {
        tt_if!{
            condition = [{tt_equal}]
            input = [{ $id1 $id2 }]         // The two identifiers are here passed to 'tt_equal'
            true = [{
                const $id1: bool = true;
            }]
            false = [{
                const $id1: bool = false;
            }]
        }
    }
}

same_ident!(AN_IDENT, AN_IDENT);            // Equal identifiers result in a true constant
same_ident!(A_DIFFERENT_IDENT, AN_IDENT);   // Different identifiers result in a false constant

fn main() {
    assert_eq!(AN_IDENT, true);
    assert_eq!(A_DIFFERENT_IDENT, false);
}
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in this crate by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
</sub>

