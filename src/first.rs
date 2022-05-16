use std::mem;

pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        // this replaces the current head of our list with an empty element
        // the current head with the pointer to the list's tail is returned by the replace method
        let current_head = mem::replace(&mut self.head, Link::Empty);

        // now check whether the current head is empty. If NOT make the next pointer of the current head
        // the new head of the list. return the node element from the box.
        match current_head {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut current_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = current_link {
            current_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::first::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));

        assert_eq!(list.pop(), None);
    }
}
