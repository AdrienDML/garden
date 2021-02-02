use crate::{Link, TreeElem};


struct BinaryMetadata {
    degree : u8,
    hight : u8,

}
/// A node that has at most 2 childs
pub(crate) struct BinaryNode<T, D>
where
    T : TreeElem
{
    parent : Link<T>,
    left : Link<T>,
    right : Link<T>,
    data : D,
}
impl<T, D> TreeElem for BinaryNode<T, D> 
where
    T : TreeElem
{}

impl<T, D> BinaryNode<T, D>
where
    T : TreeElem
{
    pub fn new_empty(data : D) -> Self {
        Self {
            parent : None,
            left : None,
            right : None,
            data
        }
    }

    pub fn new_as_leaf(parent : Link<T>, data : D) -> Self {
        Self {
            parent,
            left : None,
            right : None,
            data,
        }
    }
    
    pub fn is_root(&self) -> bool {
        if let Some(_) = self.parent {
            true
        }
        else {
            false
        }
    }
    
    pub fn is_leaf(&self) -> bool {
        !(self.has_left_child() || self.has_right_child())
    }
    pub fn has_left_child(&self) -> bool {
        if let Some(_) = self.left {
            true
        }
        else {
            false
        }
    }
    
    pub fn has_right_child(&self) -> bool {
        if let Some(_) = self.right {
            true
        }
        else {
            false
        }
    }

    pub fn value(&self) -> D {
        self.data
    }

    pub fn set_value(&mut self, data : D) {
        self.data = data;
    }

}

/// A node that can have multiple childs
pub(crate) struct Node<T, D>
where
    T : TreeElem
{
    parent : Link<T>,
    childs : Vec<Link<T>>,
    data : D,
}
impl<T, D> TreeElem for Node<T, D> 
where
    T : TreeElem
{}
impl<T, D> Node<T, D>
where
    T : TreeElem
{

    pub fn new_empty(data : D) -> Self {
        Self {
            parent : None,
            childs : Vec::new(),
            data
        }
    }
}