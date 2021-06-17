/*
    file: first.rs
    author: Yusuf Saquib

    Classic implementation of a (bad) recursive linked list
*/

use std::mem;

pub struct List 
{
    head: Link
} 
enum Link
{
    Nil,
    Node(Box<Node>),
}
struct Node{
    data: i32,
    next: Link,
}

impl List 
{
    pub fn new_list() -> Self
    {
        List { head: Link::Nil }    // Line without semicolon is returned
    }

    pub fn push(&mut self, data: i32)
    {
        let new_node = Box::new(Node{data, next: mem::replace(&mut self.head, Link::Nil)});
        self.head = Link::Node(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> 
    {
        match mem::replace(&mut self.head, Link::Nil) 
        {
            Link::Nil => None,
            Link::Node(node) =>
            {
                self.head = node.next;
                Some(node.data)
            }
        }
    }
}

impl Drop for List
{
    fn drop(&mut self)
    {
        /*  Replace the head with Nil so that when it goes out of scope, it does
            not cause any unbounded recursion. */
        let mut current_link = mem::replace(&mut self.head, Link::Nil);

        /*  'while let' == while this pattern matches.
            Replace the current link's next such that when the current link
            goes out of scope, the compiler ends by freeing that node and does 
            not try to free the next node, thus causing unbounded recursion. */
        while let Link::Node(mut boxed_node) = current_link
        {
            current_link = mem::replace(&mut boxed_node.next, Link::Nil);
        }
    }
}

#[cfg(test)]
mod test
{
    use super::List;

    #[test]
    fn basic_test()
    {
        let mut list = List::new_list();
        assert_eq!(list.pop(), None);
        
        list.push(1);
        list.push(2);

        list.push(3);
        list.push(4);
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        
        list.push(9);
        assert_eq!(list.pop(), Some(9));
        
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}