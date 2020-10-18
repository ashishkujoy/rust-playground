use std::mem;

struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_head = mem::replace(&mut self.head, None);

        while let Some(mut boxed_node) = current_head {
            current_head = mem::replace(&mut boxed_node.next, None)
        }
    }
}

impl<T> Iterator for List<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, t: T) {
        let node = Node {
            element: t,
            next: mem::replace(&mut self.head, None),
        };
        // Node now has element and self.head
        // self.head is now empty
        self.head = Some(Box::new(node))
    }

    fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.element)
            }
        }
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.element)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pop_on_empty_list() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn pop_on_list_of_one_elment() {
        let mut list = List::new();
        list.push(1);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn iterate_over_list() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.next(), Some(3));
        assert_eq!(list.next(), Some(2));
        assert_eq!(list.next(), Some(1));
        assert_eq!(list.next(), None);
    }

    #[test]
    fn peek_mut() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.peek_mut(), None);
        list.push(25);
        list.push(43);

        assert_eq!(list.peek_mut(), Some(&mut 43));
        list.peek_mut().map(|number| *number = 56);

        assert_eq!(list.peek(), Some(&56));
    }
}
