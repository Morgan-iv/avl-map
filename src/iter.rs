use std::iter::{ExactSizeIterator, Iterator};
use crate::tree::Node;

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
    stack: Vec<StackElem<NT>>,
    remain: usize,
    reverse: bool,
}

impl<NT: Split> AvlMapIter<NT> {
    pub(crate) fn new(root: Option<NT>, remain: usize, reverse: bool) -> Self {
        AvlMapIter {
            stack: match root {
                None => Vec::new(),
                Some(n) => vec![StackElem::N(n)],
            },
            remain,
            reverse,
        }
    }

    fn walk(&mut self) -> Option<NT::ET> {
        self.stack.pop().map(|elem| {
            match elem {
                StackElem::V(value) => {
                    self.remain -= 1;
                    value
                }
                StackElem::N(node) => {
                    let (left, value, right) = node.split();
                    if !self.reverse {
                        if let Some(n) = right {
                            self.stack.push(StackElem::N(n))
                        };
                        self.stack.push(StackElem::V(value));
                        if let Some(n) = left {
                            self.stack.push(StackElem::N(n))
                        };
                    } else {
                        if let Some(n) = left {
                            self.stack.push(StackElem::N(n))
                        };
                        self.stack.push(StackElem::V(value));
                        if let Some(n) = right {
                            self.stack.push(StackElem::N(n))
                        };
                    }
                    self.walk().unwrap()
                }
            }
        })
    }
}

impl<NT: Split> Iterator for AvlMapIter<NT> {
    type Item = NT::ET;

    fn next(&mut self) -> Option<Self::Item> {
        self.walk()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remain, Some(self.remain))
    }
}

impl<NT: Split> ExactSizeIterator for AvlMapIter<NT> {}
