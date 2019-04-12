pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None
        }
    }

    pub fn len(&self) -> usize {
        let mut length: usize = 0;
        let mut head = &self.head;
        loop {
            match head {
                Some(v) => {
                    length += 1;
                    head = &v.next;
                },
                None => {
                    return length
                }
            }
        }
    }

    pub fn push(&mut self, element: T) {
        let node = Node {
            data: element,
            next: None
        };

        loop {
            match &self.head {
                Some(v) => {
                    self.head = v.next;
                    continue;
                },
                None => {
                    self.head = Some(Box::new(node));
                }
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        None
    }

    pub fn peek(&self) -> Option<&T> {
        None
    }
}


impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        unimplemented!()
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        unimplemented!()
    }
}