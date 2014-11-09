/// # SelectTree
///
/// SelectTree is a data structure that implements the `Selectable`
/// trait. It's here as an example and for testing.
///
/// CSS selectors can be used to query different data types,
/// not just HTML. This simple tree makes that clear.
///

use std::collections::HashMap;

#[deriving(PartialEq, Show, Decodable, Encodable)]
pub struct SelectTree {
    pub id: Option<&'static str>,
    pub kind: &'static str,
    pub children: Vec<SelectTree>,
    pub attributes: HashMap<&'static str, &'static str>
}

impl SelectTree {
    pub fn new(k: &'static str) -> SelectTree {
        SelectTree {
            id: None,
            kind: k,
            children: vec![],
            attributes: HashMap::new()
        }
    }
}
