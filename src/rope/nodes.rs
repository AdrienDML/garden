
use crate::nodes::{Node, Leaf};

struct InternalNode<T>
where
    T : TreeElem
{
    inner : Node<T, InternalNodeData>,
    pd : PhantomData<T>
}
