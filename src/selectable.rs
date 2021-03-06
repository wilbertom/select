
pub trait Selectable {
    // fn star(&self) -> (Self);
    fn element(&self, s: &'static str) -> Vec<Self>;
    // fn has_attribute(&self, s: &'static str, a: &'static str) -> (Self);
    // fn attribute_with_value(&self, s: &'static str, a: &'static str, v: &'static str) -> (Self);
    // fn attribute_in(&self, s: &'static str, a: &'static str, v: &'static str) -> (Self);
    // fn attribute_starts_with(&self, s: &'static str, a: &'static str, v: &'static str) -> (Self);
    // fn attribute_ends_with(&self, s: &'static str, a: &'static str, v: &'static str) -> (Self);
    // fn attribute_contains(&self, s: &'static str, a: &'static str, v: &'static str) -> (Self);
    // fn root(&self, s: &'static str) -> (Self);
    // fn nth_child(&self, s: &'static str, n: int) -> (Self);
    // fn nth_last_child(&self, s: &'static str, n: int) -> (Self);
    // fn nth_of_type(&self, s: &'static str, n: int) -> (Self);
    // fn nth_last_of_type(&self, s: &'static str, n: int) -> (Self);
    // fn first_child(&self, s: &'static str) -> (Self);
    // fn last_child(&self, s: &'static str) -> (Self);
    // fn first_of_type(&self, s: &'static str) -> (Self);
    // fn last_of_type(&self, s: &'static str) -> (Self);
    // fn only_child(&self, s: &'static str) -> (Self);
    // fn only_of_type(&self, s: &'static str) -> (Self);
    // fn empty(&self, s: &'static str) -> (Self);
    // fn link(&self, s: &'static str) -> (Self);
    // fn visited(&self, s: &'static str) -> (Self);
    // fn active(&self, s: &'static str) -> (Self);
    // fn hover(&self, s: &'static str) -> (Self);
    // fn focus(&self, s: &'static str) -> (Self);
    // fn target(&self, s: &'static str) -> (Self);
    // fn lang(&self, s: &'static str, a: &'static str) -> (Self);
    // fn disabled(&self, s: &'static str) -> (Self);
    // fn enabled(&self, s: &'static str) -> (Self);
    // fn checked(&self, s: &'static str) -> (Self);
    // fn not(&self, s: &'static str) -> (Self);
    // fn first_line(&self, s: &'static str) -> (Self);
    // fn first_letter(&self, s: &'static str) -> (Self);
    // fn before(&self, s: &'static str) -> (Self);
    // fn after(&self, s: &'static str) -> (Self);
    // fn class(&self, s: &'static str, a: &'static str) -> (Self);
    // fn id(&self, s: &'static str, a: &'static str) -> (Self);
    // fn descendant(&self, s: &'static str, a: &'static str) -> (Self);
    // fn child(&self, s: &'static str, a: &'static str) -> (Self);
    // fn preceded_immediately(&self, s: &'static str, a: &'static str) -> (Self);
    // fn preceded(&self, s: &'static str, a: &'static str) -> (Self);
}
