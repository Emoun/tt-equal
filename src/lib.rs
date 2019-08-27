extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree, Delimiter, Group};
use std::iter::FromIterator;

///
/// A predicate for whether two token trees are equal.
/// <sup>**[[tt-call](https://docs.rs/tt-call/)]**</sup>
///
/// Given two token trees, it compares them and returns whether they are equal or not.
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
/// same_ident!(SomeIdent, SomeOtherIdent);     // Different identifiers result in a false constant
/// same_ident!(AnotherIdent, AnotherIdent);    // Equal identifiers result in a true constant
///
/// fn main() {
///     assert!(!SomeIdent);
///     assert!(AnotherIdent);
/// }
///
/// ```
///
/// # Caveat
///
/// This is a procedural macro and therefore has corresponding restrictions on where it can be used.
/// E.g. As of rust 1.37, it cannot be used within an expression context.
#[proc_macro]
pub fn tt_equal(item: TokenStream) -> TokenStream {
    let mut iter = item.into_iter();
    
    // First save the caller tokens
    let caller = iter.next().expect("'tt_equal' did not receive caller's tt bundle.");
    
    let key =iter.next()
        .expect("'tt_equal' expects a key-value pair as input, but did not receive a key.");
    
    if key.to_string().trim() != "input".to_string() {
        panic!("'tt_equal' expects its input's key to be named 'input' but it was '{}'",
               key.to_string().trim())
    }
    
    let separator = iter.next().expect("'tt_equal' expects a key value pair as input but did not receive it.").
        to_string();
    if separator != "=".to_string() {
        panic!("'tt_equal' expects its input key-value pairs to be separated by a '=' \
            but instead received '{}'", separator);
    }
    
    let value_group = iter.next()
        .expect("'tt_equal' expects a key-value pair as input but received no value.");
    
    if iter.next().is_some() {
        panic!("'tt_equal' expects only a key-value pair as input but received more.")
    }
    
    let mut unbracketed_group = expect_group(value_group, Delimiter::Bracket).into_iter();
    let braced_group = unbracketed_group.next()
        .expect("'tt_equal' expects its input value to be within '[{..}]' \
            but the '{..}' was not given.");
    
    if unbracketed_group.next().is_some() {
        panic!("'tt_equal' expects its input value to be within '[{..}]' \
            but it received additional tokens after the braces ('{..}').")
    }
    
    let mut clean_value = expect_group(braced_group, Delimiter::Brace).into_iter();
    let lhs = clean_value.next()
        .expect("'tt_equal' expects two token tree to compare but received none.");
    let rhs = clean_value.next()
        .expect("'tt_equal' expects two token tree to compare but received only one.");
    
    if clean_value.next().is_some() {
        panic!("'tt_equal' expects two token tree to compare but received more.")
    }
    
    if lhs.to_string() == rhs.to_string() {
        return return_to_tt(caller, true);
    } else{
        return return_to_tt(caller, false);
    }
    
}

fn expect_group(tt: TokenTree, expected_delimiter: Delimiter) -> TokenStream
{
    if let TokenTree::Group(g) = tt {
        if expected_delimiter == g.delimiter() {
            g.stream()
        } else {
            panic!("Expected delimiter '{:?}', got '{:?}'.", expected_delimiter, g.delimiter());
        }
    } else {
        panic!("Expected group token, got: {:?}", tt);
    }
}

fn return_to_tt(caller: TokenTree, b: bool) -> TokenStream
{
    let mut result: Vec<TokenTree> = Vec::new();
    let output2: TokenStream =  "tt_call::tt_return!".parse().unwrap();
    result.extend(output2);
    let output: TokenStream =  format!("is_equal = [ {{ {} }} ]", b).parse().unwrap();
    let mut body: Vec<_> = Vec::new();
    body.push(caller);
    body.extend(output);
    let to_return = TokenTree::from(Group::new(Delimiter::Brace,
                           TokenStream::from_iter(body.into_iter())));
    result.push(to_return);
    return TokenStream::from_iter(result.into_iter());
}