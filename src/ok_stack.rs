
type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T> {
    list: List<T>
}

pub struct Iter<'a, T>{
    next: Option<&'a Node<T>>
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {

    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let link = self.head.take();
        let new_node = Box::new(Node {
            elem,
            next: link,
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.head.take();
        head.map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { next: self.head.as_deref() }
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut { next: self.head.as_deref_mut() }
    }

}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();

        while let Some(mut boxed_node) = link {
            link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use super::{List, Node};

    #[test]
    fn new_creates_empty_list() {
        let list = List::<i32>::new();
        assert!(matches!(list.head, None))
    }

    #[test]
    fn push_pushes_element_to_list() {
        let mut list = List::<i32>::new();
        list.push(1);
        match &list.head {
            Some(node) => {
                assert_eq!(node.elem, 1)
            },
            _ => panic!("Expected node")
        }
    }

    #[test]
    fn pop_pops_element_from_list() {
        let mut list = List {
            head: Some(Box::new(Node {
                elem: 1,
                next: None
            }))
        };
        match list.pop() {
            Some(n) => assert_eq!(n, 1),
            None => panic!("Expected some")
        }
        match list.pop() {
            Some(_) => panic!("Expected none"),
            _ => ()
        }
    }

    #[test]
    fn peek_returns_reference_to_top_element() {
        let list = List {
            head: Some(Box::new(Node{
                elem: 1,
                next: None
            }))
        };
        match list.peek() {
            Some(n) => assert_eq!(*n, 1),
            None => panic!("Expected some")
        }
    }

    #[test]
    fn peek_mut_returns_reference_to_top_element() {
        let mut list = List {
            head: Some(Box::new(Node{
                elem: 1,
                next: None
            }))
        };
        match list.peek_mut() {
            Some(n) => assert_eq!(*n, 1),
            None => panic!("Expected some")
        }
    }

    #[test]
    fn into_iter_produces_iterator() {
        let mut list = List::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_produces_iterator() {
        let mut list = List::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut_produces_mutable_iterator() {
        let mut list = List::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }
}
