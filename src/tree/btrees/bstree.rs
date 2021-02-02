use std::mem::take;
use std::fmt::Debug;
use crate::Link;


#[derive(Debug)]
pub struct BSTree<D>
where
    D : PartialOrd + Debug,
{
    value : Option<D>,
    lsb : Link<Self>,
    rsb : Link<Self>
}


impl<D> BSTree<D> 
where
    D : PartialOrd + Debug,
{
    pub fn new_empty() -> Self {
        Self {
            value : None,
            lsb : None,
            rsb : None,
        }
    }

    pub fn new_leaf(value : D) -> Self {
        Self {
            value : Some(value),
            lsb : None,
            rsb : None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_some()
    }

    pub fn is_leaf(&self) -> bool {
        self.lsb.is_none() && self.rsb.is_none()
    }

    pub fn add(&mut self, value : D) {
        if let Some(val) = &mut self.value {
            if value == *val {return;}
            if value > *val {
                if let Some(tree) = &mut self.rsb {
                    tree.add(value);
                }
                else {
                    self.rsb = Some(Box::new(Self::new_leaf(value)));
                }
            }
            else {
                if let Some(tree) = &mut self.lsb {
                    tree.add(value);
                }
                else {
                    self.lsb = Some(Box::new(Self::new_leaf(value)));
                }
            }
        }
        else {
            *self = Self::new_leaf(value);
        }
    }

    pub fn search(&self, value : D) -> bool {
        if let Some(val) = &self.value {
            if value == *val {
                true
            }
            else {
                if value > *val {
                    if let Some(tree) = &self.rsb {
                        tree.search(value)
                    }
                    else {
                        false
                    }
                }
                else {
                    if let Some(tree) = &self.lsb {
                        tree.search(value)
                    }
                    else {
                        false
                    }
                }
            }
        }
        else {
            false
        }
    }
    
    pub fn pop_min(&mut self) -> Option<(Link<Self>, D)> {
        if let Some(_) = &mut self.value {
            if let Some(tree) = &mut self.lsb {
                tree.pop_min()
            }
            else {
                return Some((take(&mut self.rsb), take(&mut self.value).unwrap()));
            }
        }
        else {
            None
        }
    }

    pub fn pop_max(&mut self) -> Option<(Link<Self>, D)> {
        if let Some(_) = &mut self.value {
            if let Some(tree) = &mut self.rsb {
                tree.pop_max()
            }
            else {
                return Some((take(&mut self.lsb), take(&mut self.value).unwrap()));
            }
        }
        else {
            None
        }
    }

    pub fn delete(&mut self, value : D) {

        if let Some(val) = &self.value {
            if *val == value {
                if let Some(tree) = &mut self.rsb {
                    let (sub_tree, min_val) = tree.pop_min().unwrap();
                    self.value = Some(min_val);
                    self.rsb = sub_tree;
                }
                else if let Some(tree) = &mut self.lsb {
                    let (sub_tree, max_val) = tree.pop_max().unwrap();
                    self.value = Some(max_val);
                    self.lsb = sub_tree;
                }
                else {
                    self.value = None;
                }
            }
            else {
                if *val > value {
                    if let Some(tree) = &mut self.lsb {
                        tree.delete(value)
                    }
                }
                else {
                    if let Some(tree) = &mut self.rsb {
                        tree.delete(value)
                    }
                }
            }
        }
    }
}




#[cfg(test)]
mod test {
    use super::BSTree;

    #[test]
    fn addsearch() {
        let mut btree : BSTree<u32> = BSTree::new_empty();
        btree.add(10);
        btree.add(20);
        btree.add(15);
        btree.add(5);
        assert!(btree.search(10));
        assert!(btree.search(5));
        assert!(!btree.search(60));
    }

    #[test]
    fn popminmax() {
        let mut btree : BSTree<u32> = BSTree::new_empty();
        btree.add(10);
        btree.add(20);
        btree.add(15);
        btree.add(5);
        assert_eq!(btree.pop_min().unwrap().1, 5);
        assert!(!btree.search(5));
        assert_eq!(btree.pop_max().unwrap().1, 20);
        assert!(!btree.search(20));
    }

    #[test]
    fn delete() {
        let mut btree : BSTree<u32> = BSTree::new_empty();
        btree.add(12);
        btree.add(5);
        btree.add(3);
        btree.add(1);
        btree.add(7);
        btree.add(9);
        btree.add(8);
        btree.add(9);
        btree.add(11);
        btree.add(15);
        btree.add(13);
        btree.add(14);
        btree.add(17);
        btree.add(20);
        btree.add(18);
        btree.delete(15);
        assert!(!btree.search(15));
        assert!(btree.search(20));
    }

}