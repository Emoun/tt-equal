//#![feature(trace_macros)] //trace_macros!(true);

use tt_equal::tt_equal;
use tt_call::tt_if;

tt_if!{
	condition = [{tt_equal}]
	input = [{ true true }]
	true = [{
		const v: bool = true;
	}]
	false = [{
		const v: bool = false;
	}]
}

#[test]
fn test_v(){
	assert!(v);
}