use obj::craft::Craft;
use obj::node::Node;
use std::collections::LinkedList;
pub trait Algorithm {
    fn evade(&mut self) -> LinkedList<Node>;
}
