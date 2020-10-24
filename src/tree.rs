pub struct Node<K: Eq + Ord + Clone, V> {
    key: K,
    value: V,
    height: u8,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K: Eq + Ord + Clone, V> Node<K, V> {
    pub fn new(key: &K, value: V) -> Node<K, V> {
        Node {
            key: key.clone(),
            value,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn get_height(node: &Option<Box<Self>>) -> u8 {
        match node {
            Some(n) => n.height,
            None => 0,
        }
    }

    fn bfactor(&self) -> i8 {
        Self::get_height(&self.right) as i8 - Self::get_height(&self.left) as i8
    }

    fn fix_height(&mut self) {
        let hl = Self::get_height(&self.left);
        let hr = Self::get_height(&self.right);
        self.height = if hl > hr { hl } else { hr } + 1;
    }

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

    pub fn insert(mut self: Box<Self>, key: &K, value: V) -> Box<Self> {
        if key < &self.key {
            self.left = match self.left {
                Some(n) => Some(n.insert(key, value)),
                None => Some(Box::new(Node::new(key, value))),
            }
        } else {
            self.right = match self.right {
                Some(n) => Some(n.insert(key, value)),
                None => Some(Box::new(Node::new(key, value))),
            }
        }
        self.balance()
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

    pub fn remove(mut self: Box<Self>, key: &K) -> Option<Box<Self>> {
        use std::cmp::Ordering::*;
        match key.cmp(&self.key) {
            Less => {
                if let Some(n) = self.left {
                    self.left = n.remove(key);
                }
            }
            Greater => {
                if let Some(n) = self.right {
                    self.right = n.remove(key);
                }
            }
            Equal => {
                let q = self.left.take();
                let r = self.right.take();
                drop(self);
                return match r {
                    None => q,
                    Some(n) => {
                        let (r, mut min) = n.find_remove_min();
                        min.right = r;
                        min.left = q;
                        Some(min.balance())
                    }
                };
            }
        };
        Some(self.balance())
    }
}
