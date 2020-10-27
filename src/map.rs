use crate::tree::Node;
use crate::iter::AvlMapIter;
use std::iter::IntoIterator;

pub struct AvlMap<K: Ord, V> {
    pub(crate) root: Option<Box<Node<K, V>>>,
    pub(crate) size: usize,
}

impl<K: Ord, V> AvlMap<K, V> {
    pub fn new() -> Self {
        AvlMap {
            root: None,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        Node::find(&self.root, key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        Node::find_mut(&mut self.root, key)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let res = match self.root.take() {
            None => (Box::new(Node::new(key, value)), None),
            Some(n) => n.insert(key, value),
        };
        self.size += 1;
        self.root = Some(res.0);
        res.1
    }

    pub fn remove(&mut self, key: &K) {
        if let Some(n) = self.root.take() {
            self.root = n.remove(key).0;
        }
        self.size -= 1;
    }

    pub fn pop(&mut self, key: &K) -> Option<V> {
        match self.root.take() {
            None => None,
            Some(n) => {
                let res = n.remove(key);
                self.size -= 1;
                self.root = res.0;
                res.1
            }
        }
    }

    pub fn iter(&self) -> AvlMapIter<&Box<Node<K, V>>> {
        AvlMapIter::new(self.root.as_ref(), self.size)
    }
    
    pub fn iter_mut(&mut self) -> AvlMapIter<&mut Box<Node<K, V>>> {
        AvlMapIter::new(self.root.as_mut(), self.size)
    }
}

impl<K: Ord, V> IntoIterator for AvlMap<K, V> {
    type Item = (K, V);
    type IntoIter = AvlMapIter<Box<Node<K, V>>>;

    fn into_iter(self) -> Self::IntoIter {
        AvlMapIter::new(self.root, self.size)
    }
}

impl<'a, K: Ord, V> IntoIterator for &'a mut AvlMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = AvlMapIter<&'a mut Box<Node<K, V>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, K: Ord, V> IntoIterator for &'a AvlMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = AvlMapIter<&'a Box<Node<K, V>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
