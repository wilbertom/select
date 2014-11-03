pub struct Selectable {
    children: Option<Box<Selectable>>,
    tag_name: &'static str,
    attributes: &'static str
}

impl Selectable {
    pub fn star(&self) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn element(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn has_attribute(&self, s: &str, a: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn attribute_with_value(&self, s: &str, a: &str, v: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn attribute_in(&self, s: &str, a: &str, v: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn attribute_starts_with(&self, s: &str, a: &str, v: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn attribute_ends_with(&self, s: &str, a: &str, v: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn attribute_contains(&self, s: &str, a: &str, v: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn root(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn nth_child(&self, s: &str, n: int) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn nth_last_child(&self, s: &str, n: int) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn nth_of_type(&self, s: &str, n: int) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn nth_last_of_type(&self, s: &str, n: int) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn first_child(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn last_child(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn first_of_type(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn last_of_type(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn only_child(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn only_of_type(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn empty(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn link(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn visited(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn active(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn hover(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn focus(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn target(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn lang(&self, s: &str, a: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn disabled(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn enabled(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn checked(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn not(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn first_line(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn first_letter(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn before(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn after(&self, s: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn class(&self, s: &str, a: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn id(&self, s: &str, a: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn descendant(&self, s: &str, a: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn child(&self, s: &str, a: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn preceded_immediately(&self, s: &str, a: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
    pub fn preceded(&self, s: &str, a: &str) -> Selectable { Selectable{children: None, tag_name: "html", attributes: ""} }
}
