use std::{marker::PhantomData, usize};

pub(crate) mod node;
use node::Node;




struct InternalNodeData {
    byte_len : usize
}


pub(crate) enum NodeType<T>
where
    T : TreeElem
{
    Node(InternalNode<T>),
    Leaf(Leaf<T>),
}

impl<T> NodeType<T> 
where
    T : TreeElem
{
    pub fn new_empty() -> Self {
        Self::Leaf(
            Leaf::new_empty()
        )
    }
}

