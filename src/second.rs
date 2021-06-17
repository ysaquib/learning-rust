/*
    file: first.rs
    author: Yusuf Saquib

    Classic implementation of a (mediocre) recursive linked list
    using "Learning Rust With Entirely Too Many Lists"

    https://rust-unofficial.github.io/too-many-lists/second-into-iter.html
*/


pub struct List<T> 
{
    head: Link<T>
} 
type Link<T> = Option<Box<Node<T>>>;
struct Node<T>
{
    data: T,
    next: Link<T>,
}


impl<T> List<T> 
{
    pub fn new_list() -> Self
    {
        List { head: None }    // Line without semicolon is return value
    }

    pub fn push(&mut self, data: T)
    {
        let new_node = Box::new(
        Node {
            data, 
            next: self.head.take()
        });
        self.head = Some(new_node)
    }

    pub fn pop(&mut self) -> Option<T> 
    {
        self.head.take().map(|node|
        {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T>
    {
        self.head.as_ref().map(|node| {&node.data})
    }

    pub fn peek_mut(&mut self) -> Option<&mut T>
    {
        self.head.as_mut().map(|node| {&mut node.data})
    }

    pub fn into_iter(self) -> IntoIter<T>
    {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T>
    {
        Iter {next: self.head.as_deref()}
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T>
    {
        IterMut {next: self.head.as_deref_mut()}
    }
}

impl<T> Drop for List<T>
{
    fn drop(&mut self)
    {
        /*  Replace the head with Nil so that when it goes out of scope, it does
            not cause any unbounded recursion. */
        let mut current_link = self.head.take();

        /*  'while let' == while this pattern matches.
            Replace the current link's next such that when the current link
            goes out of scope, the compiler ends by freeing that node and does 
            not try to free the next node, thus causing unbounded recursion. */
        while let Some(mut boxed_node) = current_link
        {
            current_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.0.pop()
    }
}

pub struct Iter<'a, T>
{
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.next.map(|node| {
            // self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &*node);
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

pub struct IterMut<'a, T>
{
    next: Option<&'a mut Node<T>>,
}


impl<'a, T> Iterator for IterMut<'a, T>
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
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

    #[test]
    fn into_iter() 
    {
        let mut list = List::new_list();
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
    fn peek() 
    {
        let mut list = List::new_list();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2); 
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| 
        {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }
    #[test]
    fn iter() {
        let mut list = List::new_list();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new_list();
        list.push(1); 
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}