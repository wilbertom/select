///
/// # HTMLPie
/// Um, pie.
///
/// ## Selectors and Expresions
/// Great explanation of what selectors are from the spec:
/// > Selectors define the following function:
/// > `expression * element -> boolean`
/// > That is, given an element and a selector, this specification defines
///   whether that element matches the selector.
///   - http://dev.w3.org/csswg/selectors3/
///

use std::option::Option;

// All sequence types that we will implement.
// See the [spacification](http://dev.w3.org/csswg/selectors3/#selectors),
// it has a nice table with what all of this means
pub enum SequenceType {

    Star,    // *
    Element, // E


    // all pattern types related to attributes
    Attribute,                // E[foo]
    AttributeValue,           // E[foo="bar"]
    AttributeValueIn,         // E[foo~="bar"]
    AttributeValueStartsWith, // E[foo^="bar"]
    AttributeValueEndsWith,   // E[foo$="bar"]
    AttributeValueContains,   // E[foo*="bar"]


    // all pattern types with a `:` after the element
    Root,          // E:root

    NthChild,      // E:nth-child(n)
    NthLastChild,  // E:nth-last-child(n)
    NthOfType,     // E:nth-of-type(n)
    NthLastOfType, // E:nth-last-of-type(n)

    FirstChild,    // E:first-child
    LastChild,     // E:last-child
    FirstOfType,   // E:first-of-type
    LastOfType,    // E:last-of-type

    OnlyChild,     // E:only-child
    OnlyOfType,    // E:only-of-type

    Empty,         // E:empty

    Link,          // E:link
    Visited,       // E:Visited

    Active,        // E:active
    Hover,         // E:hover
    Focus,         // E:focus

    Target,        // E:target

    Lang,          // E:lang(l)

    Enabled,       // E:enabled
    Disabled,      // E:disabled

    Checked,       // E:checked

    Not,           // E:not

    // all patterns with `::`
    FirstLine,           // E::first-line
    FirstLetter,         // E::first-letter

    Before,              // E::before
    After,               // E::after

    // other patterns
    Class,               // E.class-name
    Id,                  // E#some-id


    // combinators
    Descendant,          // E F
    Child,               // E > F
    PrecededImmediately, // E + F
    Preceded,            // E ~ F
}

fn html_pie_eval(expr: &str) -> Option<SequenceType> {
    match expr {
        "*" => Some(Star),
        _ => None
    }
}
