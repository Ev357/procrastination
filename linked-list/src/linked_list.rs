use std::{
    fmt::{self, Display},
    ptr::NonNull,
};

use crate::node::Node;

#[derive(Default)]
pub struct LinkedList<T> {
    pub first: Option<NonNull<Node<T>>>,
    pub last: Option<NonNull<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn insert(&mut self, value: T) {
        match (self.first, self.last) {
            (None, None) => {
                let node = Node {
                    value,
                    next: None,
                    prev: None,
                };

                self.first = Some(NonNull::from(&node));
                self.last = Some(NonNull::from(&node));
            }
            (Some(_), Some(mut last)) => unsafe {
                let node = Node {
                    value,
                    next: None,
                    prev: self.last,
                };

                last.as_mut().next = Some(NonNull::from(&node));

                self.last = Some(NonNull::from(&node));
            },
            _ => unreachable!(),
        };
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut display_string = String::new();

        let mut current_node = self.first;

        while let Some(node) = current_node {
            display_string.push_str(&format!("{node:?}\n"));

            unsafe {
                current_node = node.as_ref().next;
            }
        }

        write!(f, "{display_string}")
    }
}
