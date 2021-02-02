pub mod tree;


// an alias to aleviate code verbosity on Nodes links
pub(crate) type Link<N> = Option<Box<N>>;

// A trait that  should be implemented for all Tree components.
pub trait TreeElem {}

impl TreeElem for () {}



