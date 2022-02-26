use std::fmt::Debug;

pub struct Node<T: Copy + Clone + Debug> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Copy + Clone + Debug> Node<T> {
    pub fn new(val: T) -> Self {
        Self {
            val: val,
            next: None,
        }
    }
}

pub struct LinkedList<T: Copy + Clone + Debug> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Copy + Clone + Debug> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn insert(&mut self, val: T) {
        if self.head.is_none() {
            self.head = Option::from(Box::new(Node::new(val)));
        } else if self.head.as_ref().unwrap().next.is_none() {
            self.head.as_mut().unwrap().next = Option::from(Box::new(Node::new(val)));
        } else {
            let mut iter: &mut Node<T> = &mut self.head.as_mut().unwrap();
            while iter.next.is_some() {
                iter = iter.next.as_mut().unwrap();
            }
            iter.next = Option::from(Box::new(Node::new(val)));
        }
        self.size += 1;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn print(&self) {
        let mut iter: &Option<Box<Node<T>>> = &self.head;
        while iter.is_some() {
            print!("{:?} -> ", iter.as_ref().unwrap().val);
            iter = &iter.as_ref().unwrap().next;
        }
        println!("None");
    }
}

#[cfg(test)]
mod test {}
