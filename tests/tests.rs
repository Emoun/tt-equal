//#![feature(trace_macros)] //trace_macros!(true);

use tt_call::tt_if;
use tt_equal::tt_equal;

///
/// We use this macro to invoke 'tt_equal' and produce a bool const of whether the
/// two given tokens were equal.
///
/// The first argument is the name of the resulting const, while the two following arguments
/// are to be compared.
///
macro_rules! invoke_tt_equal {
    {
        $id1:ident $tt1:tt $tt2:tt
    } => {
        tt_if!{
            condition = [{tt_equal}]
	            input = [{ $tt1 $tt2 }]
	            true = [{
                	const $id1: bool = true;
	            }]
	            false = [{
                	const $id1: bool = false;
	            }]
        }
    }
}

invoke_tt_equal!(COLONS : :);
invoke_tt_equal!(COLON_EQUAL : =);
invoke_tt_equal!(DOUBLE_SINGLE_COLON :: :);
invoke_tt_equal!(SINGLE_DOUBLE_COLON : ::);
invoke_tt_equal!(DOUBLE_DOUBLE_COLON :: ::);
invoke_tt_equal!(INCLUSIVE_RANGE_DOUBLE_COLON ..= ::);

///
/// Tests that `tt_equal` produces the correct equality result for all invocations.
///
#[test]
fn test_tt_equal_invocations() {
    assert!(COLONS);
    assert!(!COLON_EQUAL);
    assert!(!DOUBLE_SINGLE_COLON);
    assert!(!SINGLE_DOUBLE_COLON);
    assert!(DOUBLE_DOUBLE_COLON);
    assert!(!INCLUSIVE_RANGE_DOUBLE_COLON);
}
