pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

#[derive(Clone)]
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
        let node = Box::new(Node {
            data: element,
            next: self.head.take()
        });

        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.data),
            None => None
        }
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut rev_list = SimpleLinkedList::new();
        let mut head = &self.head;
        loop {
            match head {
                None => { break },
                Some(node) => {
                    let new_node = node.clone();
                    rev_list.push(new_node.data);
                    head = &node.next;
                    continue
                }
            }
        }
        rev_list
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        let mut linked_list = SimpleLinkedList::new();
        for i in item {
            linked_list.push(i.to_owned())
        }
        linked_list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
    }
}