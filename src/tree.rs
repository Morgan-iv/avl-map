use std::cmp::Ordering::*;

pub struct Node<K: Eq + Ord, V> {
    pub(crate) key: K,
    pub(crate) value: V,
    pub(crate) height: u8,
    pub(crate) left: Option<Box<Node<K, V>>>,
    pub(crate) right: Option<Box<Node<K, V>>>,
}

impl<K: Eq + Ord, V> Node<K, V> {
    pub(crate) fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            height: 1,
            left: None,
            right: None,
        }
    }

    #[inline]
    fn get_height(node: &Option<Box<Self>>) -> u8 {
        match node {
            Some(n) => n.height,
            None => 0,
        }
    }

    #[inline]
    fn bfactor(&self) -> i8 {
        Self::get_height(&self.right) as i8 - Self::get_height(&self.left) as i8
    }

    #[inline]
    fn fix_height(&mut self) {
        let hl = Self::get_height(&self.left);
        let hr = Self::get_height(&self.right);
        self.height = if hl > hr { hl } else { hr } + 1;
    }

    #[inline]
    fn rotate_right(self: Box<Self>) -> Box<Self> {
        let mut p = self;
        let mut q = p.left.unwrap();
        p.left = q.right;
        p.fix_height();
        q.right = Some(p);
        // q.right.as_mut().unwrap().fix_height();
        q.fix_height();
        q
    }

    #[inline]
    fn rotate_left(self: Box<Self>) -> Box<Self> {
        let mut q = self;
        let mut p = q.right.unwrap();
        q.right = p.left;
        q.fix_height();
        p.left = Some(q);
        // p.left.as_mut().unwrap().fix_height();
        p.fix_height();
        p
    }

    #[inline]
    fn balance(mut self: Box<Self>) -> Box<Self> {
        self.fix_height();
        if self.bfactor() == 2 {
            if self.right.as_ref().unwrap().bfactor() < 0 {
                self.right = Some(self.right.unwrap().rotate_right())
            }
            return self.rotate_left();
        }
        if self.bfactor() == -2 {
            if self.left.as_ref().unwrap().bfactor() > 0 {
                self.left = Some(self.left.unwrap().rotate_left())
            }
            return self.rotate_right();
        }
        self
    }

    pub(crate) fn insert(mut self: Box<Self>, key: K, value: V) -> (Box<Self>, Option<V>) {
        match key.cmp(&self.key) {
            Less => {
                let res = match self.left {
                    Some(n) => n.insert(key, value),
                    None => ((Box::new(Node::new(key, value))), None),
                };
                self.left = Some(res.0);
                (self.balance(), res.1)
            }
            Greater => {
                let res = match self.right {
                    Some(n) => n.insert(key, value),
                    None => ((Box::new(Node::new(key, value))), None),
                };
                self.right = Some(res.0);
                (self.balance(), res.1)
            }
            Equal => {
                let tmp = Some(self.value);
                self.value = value;
                (self, tmp)
            }
        }
    }

    fn find_remove_min(mut self: Box<Self>) -> (Option<Box<Self>>, Box<Self>) {
        match self.left {
            None => (self.right.take(), self),
            Some(n) => {
                let res = n.find_remove_min();
                self.left = res.0;
                (Some(self.balance()), res.1)
            }
        }
    }

    pub(crate) fn remove(mut self: Box<Self>, key: &K) -> (Option<Box<Self>>, Option<V>) {
        let mut value = None;
        match key.cmp(&self.key) {
            Less => {
                if let Some(n) = self.left {
                    let (res, v) = n.remove(key);
                    self.left = res;
                    value = v;
                }
                (Some(self.balance()), value)
            }
            Greater => {
                if let Some(n) = self.right {
                    let (res, v) = n.remove(key);
                    self.right = res;
                    value = v;
                }
                (Some(self.balance()), value)
            }
            Equal => {
                let q = self.left.take();
                let r = self.right.take();
                let v = self.value;
                match r {
                    None => (q, Some(v)),
                    Some(n) => {
                        let (r, mut min) = n.find_remove_min();
                        min.right = r;
                        min.left = q;
                        (Some(min.balance()), Some(v))
                    }
                }
            }
        }
    }

    pub(crate) fn find<'a>(node: &'a Option<Box<Self>>, key: &K) -> Option<&'a V> {
        node.as_ref().map(|n| match key.cmp(&n.key) {
            Less => Self::find(&n.left, key),
            Greater => Self::find(&n.right, key),
            Equal => Some(&n.value),
        }).flatten()
    }

    pub(crate) fn find_mut<'a>(node: &'a mut Option<Box<Self>>, key: &K) -> Option<&'a mut V> {
        node.as_mut().map(|n| match key.cmp(&n.key) {
            Less => Self::find_mut(&mut n.left, key),
            Greater => Self::find_mut(&mut n.right, key),
            Equal => Some(&mut n.value),
        }).flatten()
    }
}
