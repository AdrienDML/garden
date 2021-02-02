use crate::{TreeElem, Link};
use std::io::{self, Read, Write};

pub struct Rope<T>
where
    T : TreeElem
{
    inner : Link<T>,
}

impl<T> Rope<T>
where
    T : TreeElem
{
    
    pub fn new_empty() -> Self {
        todo!()
    }

    pub fn from_str(text : &str) {
        todo!()
    }

    pub fn from_reader<R : Read>(reader : R) -> io::Result<Self> {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn write_to<W : Write>(&self, writer : W) -> io::Result<()> {
        todo!()
    }

    pub fn insert(&mut self, idx : usize, text : &str) {
        todo!()
    }

    pub fn remove(&mut self, idx : usize) {
        todo!()
    }

    pub fn split(&mut self, idx : usize) {
        todo!()
    }

    pub fn concat(&mut self, other : Self) {
        todo!()
    }
} 