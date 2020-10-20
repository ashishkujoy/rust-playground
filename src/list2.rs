use std::rc::Rc;

#[derive(PartialEq, Debug)]
struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn append(&self, t: T) -> Self {
        let head = Node {
            element: t,
            next: self.head.clone(),
        };
        List {
            head: Some(Rc::new(head)),
        }
    }

    fn tail(&self) -> Self {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }
}

type Link<T> = Option<Rc<Node<T>>>;

#[derive(PartialEq, Debug)]
struct Node<T> {
    element: T,
    next: Link<T>,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn head() {
        let list: List<i32> = List::new();

        assert_eq!(list.head(), None);
        assert_eq!(list.append(2).head(), Some(&2));
    }

    #[test]
    fn append() {
        let list: List<i32> = List::new();

        let list2 = list.append(1);
        let list3 = list2.append(2);

        assert_eq!(list.head(), None);
        assert_eq!(list2.head(), Some(&1));
        assert_eq!(list3.head(), Some(&2));
    }

    #[test]
    fn tail() {
        let list: List<i32> = List::new();

        let list2 = list.append(1);
        let list3 = list2.append(2);

        assert_eq!(list3.tail(), list2);
        assert_eq!(list2.tail(), list);
        assert_eq!(list.tail(), List::new());
    }
}
