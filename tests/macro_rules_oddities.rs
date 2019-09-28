//!
//! Tests an oddity that irises when a macro creates a macro_rules.
//! This happens when a macro wants to partially apply `tt_equal` and then
//! use the resulting macro as a predicate (in this case for use in `tt_call::replace`.
//!
//! In the predicate macro `is_placeholder`, 'some_placeholder' is inserted as a constant identifier.
//! This makes it into a `Group` and not a single token.
//! When `tt_replace` then calls it with a token, this will not be a group. So even when
//! replace gives is 'some_placeholder' their `to_string()` won't be equal, because the first
//! is a gropu of 1 token and the second is just a token.
//! Their string representations will be "  some_placeholder  " and "some_placeholder".
//! The spaces will cause the comparison to return false.
//!
//! We will test for this uses macro_rules! such that it is future-proof.
//! Even if the implementation of macros changes, and the above is no longer the case,
//! this test will ensure we notice of our solution stops working.
//!

macro_rules! duplicate_for_bool{

	{
		$dollar:tt $placeholder:ident
		$($rest:tt)*
	} => {
		macro_rules! is_placeholder {
            {
                $dollar caller:tt
                input = [{ $dollar body:tt }]
            } => {
                tt_equal::tt_equal! {
                    $dollar caller
                    input = [{ $placeholder $dollar body }]
                }
            }
        }

		mod for_true{
			tt_call::tt_call! {
				macro = [{ tt_call::tt_replace }]
				condition = [{ is_placeholder }]
				replace_with = [{ true }]
				input = [{ $($rest)* }]
			}
        }
        mod for_false{
			tt_call::tt_call! {
				macro = [{ tt_call::tt_replace }]
				condition = [{ is_placeholder }]
				replace_with = [{ false }]
				input = [{ $($rest)* }]
			}
		}
	};
}
duplicate_for_bool! {
    $some_placeholder
    pub const BOOL: bool = some_placeholder;
}
#[test]
fn test_duplicate() {
    assert!(for_true::BOOL);
    assert!(!for_false::BOOL);
}
