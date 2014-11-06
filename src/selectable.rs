use std::collections::HashMap;

pub trait Selectable {
    pub fn star(&self) -> Selectable;
    pub fn element(&self, s: &str) -> [Selectable];
    pub fn has_attribute(&self, s: &str, a: &str) -> [Selectable];
    pub fn attribute_with_value(&self, s: &str, a: &str, v: &str) -> [Selectable];
    pub fn attribute_in(&self, s: &str, a: &str, v: &str) -> [Selectable];
    pub fn attribute_starts_with(&self, s: &str, a: &str, v: &str) -> [Selectable];
    pub fn attribute_ends_with(&self, s: &str, a: &str, v: &str) -> [Selectable];
    pub fn attribute_contains(&self, s: &str, a: &str, v: &str) -> [Selectable];
    pub fn root(&self, s: &str) -> [Selectable];
    pub fn nth_child(&self, s: &str, n: int) -> [Selectable];
    pub fn nth_last_child(&self, s: &str, n: int) -> [Selectable];
    pub fn nth_of_type(&self, s: &str, n: int) -> [Selectable];
    pub fn nth_last_of_type(&self, s: &str, n: int) -> [Selectable];
    pub fn first_child(&self, s: &str) -> [Selectable];
    pub fn last_child(&self, s: &str) -> [Selectable];
    pub fn first_of_type(&self, s: &str) -> [Selectable];
    pub fn last_of_type(&self, s: &str) -> [Selectable];
    pub fn only_child(&self, s: &str) -> [Selectable];
    pub fn only_of_type(&self, s: &str) -> [Selectable];
    pub fn empty(&self, s: &str) -> [Selectable];
    pub fn link(&self, s: &str) -> [Selectable];
    pub fn visited(&self, s: &str) -> [Selectable];
    pub fn active(&self, s: &str) -> [Selectable];
    pub fn hover(&self, s: &str) -> [Selectable];
    pub fn focus(&self, s: &str) -> [Selectable];
    pub fn target(&self, s: &str) -> [Selectable];
    pub fn lang(&self, s: &str, a: &str) -> [Selectable];
    pub fn disabled(&self, s: &str) -> [Selectable];
    pub fn enabled(&self, s: &str) -> [Selectable];
    pub fn checked(&self, s: &str) -> [Selectable];
    pub fn not(&self, s: &str) -> [Selectable];
    pub fn first_line(&self, s: &str) -> [Selectable];
    pub fn first_letter(&self, s: &str) -> [Selectable];
    pub fn before(&self, s: &str) -> [Selectable];
    pub fn after(&self, s: &str) -> [Selectable];
    pub fn class(&self, s: &str, a: &str) -> [Selectable];
    pub fn id(&self, s: &str, a: &str) -> Selectable;
    pub fn descendant(&self, s: &str, a: &str) -> [Selectable];
    pub fn child(&self, s: &str, a: &str) -> [Selectable];
    pub fn preceded_immediately(&self, s: &str, a: &str) -> [Selectable];
    pub fn preceded(&self, s: &str, a: &str) -> [Selectable];
}