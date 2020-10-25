use std::iter::{DoubleEndedIterator, ExactSizeIterator, Iterator};
use std::collections::VecDeque;
use crate::tree::Node;

macro_rules! walk {
    ($name:ident, $pop:ident, $push:ident, $swap:ident) => {
        fn $name(&mut self) -> Option<NT::ET> {
            const SWAP: bool = $swap;
            self.deque.$pop().map(|elem| {
                match elem {
                    StackElem::V(value) => {
                        self.remain -= 1;
                        value
                    }
                    StackElem::N(node) => {
                        let (left, value, right) = if !SWAP {
                            let (left, value, right) = node.split();
                            (left, value, right)
                        } else {
                            let (left, value, right) = node.split();
                            (right, value, left)
                        };
                        if let Some(n) = right {
                            self.deque.$push(StackElem::N(n))
                        };
                        self.deque.$push(StackElem::V(value));
                        if let Some(n) = left {
                            self.deque.$push(StackElem::N(n))
                        };
                        self.$name().unwrap()
                    }
                }
            })
        }
    }
}

pub trait Split
where
    Self: std::marker::Sized,
{
    type ET;
    fn split(self) -> (Option<Self>, Self::ET, Option<Self>);
}

impl<K: Eq + Ord, V> Split for Box<Node<K, V>> {
    type ET = (K, V);

    fn split(self) -> (Option<Self>, Self::ET, Option<Self>) {
        (self.left, (self.key, self.value), self.right)
    }
}

impl<'a, K: Eq + Ord, V> Split for &'a mut Box<Node<K, V>> {
    type ET = (&'a K, &'a mut V);

    fn split(self) -> (Option<Self>, Self::ET, Option<Self>) {
        (self.left.as_mut(), (&self.key, &mut self.value), self.right.as_mut())
    }
}

impl<'a, K: Eq + Ord, V> Split for &'a Box<Node<K, V>> {
    type ET = (&'a K, &'a V);

    fn split(self) -> (Option<Self>, Self::ET, Option<Self>) {
        (self.left.as_ref(), (&self.key, &self.value), self.right.as_ref())
    }
}

pub enum StackElem<NT: Split> {
    N(NT),
    V(NT::ET),
}

pub struct AvlMapIter<NT: Split> {
    deque: VecDeque<StackElem<NT>>,
    remain: usize,
}

impl<NT: Split> AvlMapIter<NT> {
    pub(crate) fn new(root: Option<NT>, remain: usize) -> Self {
        AvlMapIter {
            deque: match root {
                None => VecDeque::new(),
                Some(n) => VecDeque::from(vec![StackElem::N(n)]),
            },
            remain,
        }
    }

    walk!(walk_l, pop_back, push_back, false);
    walk!(walk_r, pop_front, push_front, true);
}

impl<NT: Split> Iterator for AvlMapIter<NT> {
    type Item = NT::ET;

    fn next(&mut self) -> Option<Self::Item> {
        self.walk_l()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remain, Some(self.remain))
    }
}

impl<NT: Split> DoubleEndedIterator for AvlMapIter<NT> {
    fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
        self.walk_r()
    }
}

impl<NT: Split> ExactSizeIterator for AvlMapIter<NT> {}
