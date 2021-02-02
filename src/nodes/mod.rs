mod node;

use std::marker::PhantomData;

pub(crate) use node::*;
use crate::TreeElem;
struct Leaf<T>
where
    T : TreeElem
{
    inner : Node<T, String>,
    pd : PhantomData<T>
}

impl<T> Leaf<T> 
where
    T : TreeElem 
{
    pub fn new_empty() -> Self {
        Self {
            inner : Node::new_empty("".to_string()),
            pd : PhantomData
        }
    }
}