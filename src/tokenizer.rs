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
use selectable::Selectable;

// All sequence types that we will implement.
// See the [spacification](http://dev.w3.org/csswg/selectors3/#selectors),
// it has a nice table with what all of this means
pub enum SequenceType {

    Star,          // *
    Element(&'static str),   // E

    // all pattern types related to attributes
    Attribute(&'static str, &'static str),                              // E[foo]
    AttributeValue(&'static str, &'static str, &'static str),           // E[foo="bar"]
    AttributeValueIn(&'static str, &'static str, &'static str),         // E[foo~="bar"]
    AttributeValueStartsWith(&'static str, &'static str, &'static str), // E[foo^="bar"]
    AttributeValueEndsWith(&'static str, &'static str, &'static str),   // E[foo$="bar"]
    AttributeValueContains(&'static str, &'static str, &'static str),   // E[foo*="bar"]

    // all pattern types with a `:` after the element
    Root(&'static str), // E:root

    NthChild(&'static str, int),      // E:nth-child(n)
    NthLastChild(&'static str, int),  // E:nth-last-child(n)
    NthOfType(&'static str, int),     // E:nth-of-type(n)
    NthLastOfType(&'static str, int), // E:nth-last-of-type(n)

    FirstChild(&'static str),    // E:first-child
    LastChild(&'static str),     // E:last-child
    FirstOfType(&'static str),   // E:first-of-type
    LastOfType(&'static str),    // E:last-of-type

    OnlyChild(&'static str),     // E:only-child
    OnlyOfType(&'static str),    // E:only-of-type

    Empty(&'static str),         // E:empty

    Link(&'static str),          // E:link
    Visited(&'static str),       // E:Visited

    Active(&'static str),        // E:active
    Hover(&'static str),         // E:hover
    Focus(&'static str),         // E:focus

    Target(&'static str),        // E:target

    Lang(&'static str, &'static str),    // E:lang(l)

    Enabled(&'static str),       // E:enabled
    Disabled(&'static str),      // E:disabled

    Checked(&'static str),       // E:checked

    Not(&'static str),           // E:not

    // all patterns with `::`
    FirstLine(&'static str),           // E::first-line
    FirstLetter(&'static str),         // E::first-letter

    Before(&'static str),              // E::before
    After(&'static str),               // E::after

    // other patterns
    Class(&'static str, &'static str),               // E.class-name
    Id(&'static str, &'static str),                  // E#some-id

    // combinators
    Descendant(&'static str, &'static str),          // E F
    Child(&'static str, &'static str),               // E > F
    PrecededImmediately(&'static str, &'static str), // E + F
    Preceded(&'static str, &'static str),            // E ~ F

    Unknown
}

fn html_pie_parse(expr: &'static str) -> SequenceType {
    match expr {
        "*" => Star,
        _ => Unknown
    }
}

fn html_pie_query(st: SequenceType, s: Selectable) -> Selectable {

    match st {
        Star => s.star(),
        Element(e) => s.element(e),
        Attribute(e, a) => s.has_attribute(e, a),
        AttributeValue(e, a, v) => s.attribute_with_value(e, a, v),
        AttributeValueIn(e, a, v) => s.attribute_in(e, a, v),
        AttributeValueStartsWith(e, a, v) => s.attribute_starts_with(e, a, v),
        AttributeValueEndsWith(e, a, v) => s.attribute_ends_with(e, a, v),
        AttributeValueContains(e, a, v) => s.attribute_contains(e, a, v),
        Root(e) => s.root(e),
        NthChild(e, i) => s.nth_child(e, i),
        NthLastChild(e, i) => s.nth_last_child(e, i),
        NthOfType(e, i) => s.nth_of_type(e, i),
        NthLastOfType(e, i) => s.nth_last_of_type(e, i),
        FirstChild(e) => s.first_child(e),
        LastChild(e) => s.last_child(e),
        FirstOfType(e) => s.first_of_type(e),
        LastOfType(e) => s.last_of_type(e),
        OnlyChild(e) => s.only_child(e),
        OnlyOfType(e) => s.only_of_type(e),
        Empty(e) => s.empty(e),
        Link(e) => s.link(e),
        Visited(e) => s.visited(e),
        Active(e) => s.active(e),
        Hover(e) => s.hover(e),
        Focus(e) => s.focus(e),
        Target(e) => s.target(e),
        Lang(e, l) => s.lang(e, l),
        Enabled(e) => s.enabled(e),
        Disabled(e) => s.disabled(e),
        Checked(e) => s.checked(e),
        Not(e) => s.not(e),
        FirstLine(e) => s.first_line(e),
        FirstLetter(e) => s.first_letter(e),
        Before(e) => s.before(e),
        After(e) => s.after(e),
        Class(e, s_class) => s.class(e, s_class),
        Id(e, s_id) => s.id(e, s_id),
        Descendant(e1, e2) => s.descendant(e1, e2),
        Child(e1, e2) => s.child(e1, e2),
        PrecededImmediately(e1, e2) => s.preceded_immediately(e1, e2),
        Preceded(e1, e2) => s.preceded(e1, e2),
        Unknown => s
    }
}
