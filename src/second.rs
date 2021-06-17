/*
    file: first.rs
    author: Yusuf Saquib

    Classic implementation of a (bad) recursive linked list
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
        List { head: None }    // Line without semicolon is returned
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
}