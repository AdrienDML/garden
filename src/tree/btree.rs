use crate::nodes::BinaryNode;


pub struct BTree<T, D>
where
    T : TreeElem
{
    root : Link<BinaryNode<T, D>>,
}

impl<T, D> BTree<T, D> 
where
    T : TreeElem
{
    pub fn new_empty() {
        
    }   
}