use std::ptr::NonNull;

#[derive(Debug, Default)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<NonNull<Node<T>>>,
    pub prev: Option<NonNull<Node<T>>>,
}
