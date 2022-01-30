use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    Node(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {

    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let link = mem::replace(&mut self.head, Link::Empty);
        let new_node = Box::new(Node {
            elem,
            next: link,
        });

        self.head = Link::Node(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let head = mem::replace(&mut self.head, Link::Empty);
        match head {
            Link::Empty => None,
            Link::Node(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

}

impl Drop for List {
    fn drop(&mut self) {
        let mut link = mem::replace(&mut self.head, Link::Empty);

        while let Link::Node(mut boxed_node) = link {
            link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::{List, Link, Node};

    #[test]
    fn new_creates_empty_list() {
        let list = List::new();
        assert!(matches!(list.head, Link::Empty))
    }

    #[test]
    fn push_pushes_element_to_list() {
        let mut list = List::new();
        list.push(1);
        match &list.head {
            Link::Node(node) => {
                assert_eq!(node.elem, 1)
            },
            _ => panic!("Expected node")
        }
    }

    #[test]
    fn pop_pops_element_from_list() {
        let mut list = List {
            head: Link::Node(Box::new(Node {
                elem: 1,
                next: Link::Empty
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
}