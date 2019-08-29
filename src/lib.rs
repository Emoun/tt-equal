extern crate proc_macro;
use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use std::iter::FromIterator;

///
/// A predicate for whether two token trees are equal.
/// <sup>**[[tt-call](https://docs.rs/tt-call/)]**</sup>
///
/// Given two token trees, it compares them and returns whether they are equal.
/// Intended for use with [tt_if](https://docs.rs/tt-call/1.0.6/tt_call/macro.tt_if.html).
///
/// # Input
///
/// - `input = [{` exactly two token trees `}]`
///
/// # Output
///
/// - `is_equal = [{` either true or false `}]`
///
/// # Example
///
/// ```
/// use tt_equal::tt_equal;
/// use tt_call::tt_if;
///
/// macro_rules! same_ident{
///     {
///         $id1:ident, $id2:ident
///     } => {
///         tt_if!{
///             condition = [{tt_equal}]
///	            input = [{ $id1 $id2 }]         // The two identifiers are here passed to 'tt_equal'
///	            true = [{
///                 const $id1: bool = true;
///	            }]
///	            false = [{
///                 const $id1: bool = false;
///	            }]
///         }
///     }
/// }
///
/// same_ident!(AN_IDENT, AN_IDENT);            // Equal identifiers result in a true constant
/// same_ident!(A_DIFFERENT_IDENT, AN_IDENT);   // Different identifiers result in a false constant
///
/// fn main() {
///     assert_eq!(AN_IDENT, true);
///     assert_eq!(A_DIFFERENT_IDENT, false);
/// }
///
/// ```
///
/// # Caveat
///
/// This is a procedural macro and therefore has corresponding restrictions on where it can be used.
/// E.g. As of Rust 1.37, it cannot be used within an expression context.
///
/// # Hint
///
/// This macro only accepts a single token tree on each 'side' of the comparison.
/// To compare multiple token trees, parantheses, brackets, or braces can be used to wrap
/// the tokens and make them into a single token tree.
///
/// Example:
///
/// ```
/// use tt_equal::tt_equal;
/// use tt_call::tt_if;
///
/// tt_if!{
///	    condition = [{tt_equal}]
///	    input = [{ (Two tokens) (Two tokens) }]
///	    true = [{
///		    const SHOULD_BE_TRUE: bool = true;
///	    }]
///	    false = [{
///		    const SHOULD_BE_TRUE: bool = false;
///	    }]
/// }
///
/// tt_if!{
///	    condition = [{tt_equal}]
///	    input = [{ (Two tokens) (Three tokens here) }]
///	    true = [{
///		    const SHOULD_BE_FALSE: bool = true;
///	    }]
///	    false = [{
///		    const SHOULD_BE_FALSE: bool = false;
///	    }]
/// }
///
/// fn main() {
///     assert_eq!(SHOULD_BE_TRUE, true);
///     assert_eq!(SHOULD_BE_FALSE, false);
/// }
///
/// ```
#[proc_macro]
pub fn tt_equal(item: TokenStream) -> TokenStream {
    let (caller, lhs, rhs) = validate(item);

    if lhs.to_string() == rhs.to_string() {
        return return_to_tt(caller, true);
    } else {
        return return_to_tt(caller, false);
    }
}

///
/// Validates that the input to 'tt_equal' is correct and returns:
/// 0. The callers opaque tt bundle
/// 1. The left-hand side of the input to compare
/// 2. The right-hand side of the input to compare
///
fn validate(item: TokenStream) -> (TokenTree, TokenTree, TokenTree) {
    let mut iter = item.into_iter();

    let caller = iter
        .next()
        .expect("'tt_equal' did not receive caller's tt bundle.");
    let key = iter
        .next()
        .expect("'tt_equal' expects a key-value pair as input, but did not receive a key.");
    if key.to_string().trim() != "input".to_string() {
        panic!(
            "'tt_equal' expects its input's key to be named 'input' but it was '{}'",
            key.to_string().trim()
        )
    }
    let separator = iter
        .next()
        .expect("'tt_equal' expects a key value pair as input but did not receive it.")
        .to_string();
    if separator != "=".to_string() {
        panic!(
            "'tt_equal' expects its input key-value pairs to be separated by a '=' \
             but instead received '{}'",
            separator
        );
    }
    let value_group = iter
        .next()
        .expect("'tt_equal' expects a key-value pair as input but received no value.");
    if iter.next().is_some() {
        panic!("'tt_equal' expects only a key-value pair as input but received more.")
    }
    let mut unbracketed_group = expect_group(value_group, Delimiter::Bracket).into_iter();
    let braced_group = unbracketed_group.next().expect(
        "'tt_equal' expects its input value to be within '[{..}]' \
         but the '{..}' was not given.",
    );
    if unbracketed_group.next().is_some() {
        panic!(
            "'tt_equal' expects its input value to be within '[{..}]' \
             but it received additional tokens after the braces ('{..}')."
        )
    }
    let mut clean_value = expect_group(braced_group, Delimiter::Brace).into_iter();
    let lhs = clean_value
        .next()
        .expect("'tt_equal' expects two token tree to compare but received none.");
    let rhs = clean_value
        .next()
        .expect("'tt_equal' expects two token tree to compare but received only one.");
    if clean_value.next().is_some() {
        panic!("'tt_equal' expects two token tree to compare but received more.")
    }
    (caller, lhs, rhs)
}

///
/// Unwraps a token tree, assuming it has the given delimiter, and returns
/// its contents
///
fn expect_group(tt: TokenTree, expected_delimiter: Delimiter) -> TokenStream {
    if let TokenTree::Group(g) = tt {
        if expected_delimiter == g.delimiter() {
            g.stream()
        } else {
            panic!(
                "'tt_equal' expects delimiter '{:?}' but got '{:?}'.",
                expected_delimiter,
                g.delimiter()
            );
        }
    } else {
        panic!(
            "'tt_equal' expects a group of tokens inside {:?} but got '{:?}'",
            expected_delimiter, tt
        );
    }
}

///
/// Constructs the result of 'tt_equal'
///
fn return_to_tt(caller: TokenTree, b: bool) -> TokenStream {
    let return_call: TokenStream = "tt_call::tt_return!".parse().expect(
        "'tt_equal' internal error 1. Please file a bug with the tt-equal crate maintainers.",
    );
    let return_value: TokenStream = format!("is_equal = [ {{ {} }} ]", b).parse().expect(
        "'tt_equal' internal error 2.  Please file a bug with the tt-equal crate maintainers.",
    );

    let mut return_body: Vec<_> = Vec::new();
    return_body.push(caller);
    return_body.extend(return_value);
    let return_call_argument = TokenTree::from(Group::new(
        Delimiter::Brace,
        TokenStream::from_iter(return_body.into_iter()),
    ));

    let mut result: Vec<TokenTree> = Vec::new();
    result.extend(return_call);
    result.push(return_call_argument);

    return TokenStream::from_iter(result.into_iter());
}
