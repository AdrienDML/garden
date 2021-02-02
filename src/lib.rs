pub mod tree;
pub mod rope;

mod nodes;

pub(crate) type Link<T> = Option<Box<T>>;
pub(crate) trait TreeElem {}

impl TreeElem for () {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
