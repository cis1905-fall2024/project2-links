#[allow(unused_imports)]
use std::{fmt::Display, mem};

#[derive(Debug)]
pub enum ListNode<T> {
    Nil,
    Cons(T, Box<ListNode<T>>),
}

impl<T> ListNode<T> {
  // Use the implementation of this method to guide your implementation of
  // `insert` and `reverse`
  /// Deletes a node from the list
  pub fn delete(&mut self) {
    // Temporarily replaces the current node with default value (Nil).
    // In exchange, we get to take ownership of the current node instead of just
    // having it by mutable reference.
    let as_owned: ListNode<T> = mem::take(self);
    match as_owned {
      ListNode::Nil => {}
      ListNode::Cons(_, next) => {
        // Write the next node to the current node
        *self = *next;
      }
    }
  }
}

// Required methods for `ListNode<T>`
impl<T> ListNode<T> {
    pub fn new() -> Self {
        todo!()
    }
    pub fn insert(&mut self, value: T) -> &mut Self {
        todo!()
    }
    pub fn reverse(&mut self) {
        todo!()
    }
}

// Implement `PartialEq` for `ListNode<T>`
// TODO:

// Implement `Display` for `ListNode<T>`
// TODO:

// Implement `Eq` for `ListNode<T>`
// TODO:

// Implement `Default` for `ListNode<T>`
impl<T> Default for ListNode<T> {
    fn default() -> Self {
        todo!()
    }
}

// Implement `From<Vec<T>>` for `ListNode<T>`
// TODO:

// Implement `From<ListNode<T>>` for `Vec<T>`
// TODO:
