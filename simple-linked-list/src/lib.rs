use std::iter::FromIterator;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {head: None}
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        let mut node = &self.head;
        while node.is_some() {
            length += 1;
            node = &node.as_ref().unwrap().next;
        }
        length
    }

    pub fn push(&mut self, data: T) {
        self.head = Some(Box::new(Node {
            data: data,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().and_then(|head| {
            self.head = head.next;
            Some(head.data)
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().and_then(|head| {
            Some(&head.data)
        })
    }

    pub fn rev(self) -> SimpleLinkedList<T>
    where SimpleLinkedList<T>: Into<Vec<T>> {
        let mut vec: Vec<T> = self.into();
        vec.reverse();
        vec.into_iter().collect()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut llist: SimpleLinkedList<T> = SimpleLinkedList::new();
        for val in iter {
            llist.push(val);
        }
        llist
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T>
where T: std::marker::Copy {
    fn into(self) -> Vec<T> {
        let mut v: Vec<T> = vec![];
        let mut node = &self.head;
        while let Some(box_val) = node.as_ref() {
            v.push(box_val.data);
            node = &node.as_ref().unwrap().next;
        }
        v.reverse();
        v
    }
}
