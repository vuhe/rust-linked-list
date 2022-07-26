use std::rc::Rc;

pub struct SharedSinglyLinked<T> {
    head: Link<T>,
}

// 相比 singly linked 这里将 Box 改为用 Rc 包装
// Rc 可以实现引用计数，但是它是不可变的
type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
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

impl<T> SharedSinglyLinked<T> {
    pub fn new() -> Self {
        SharedSinglyLinked { head: None }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }

    pub fn pre_append(&self, elem: T) -> Self {
        // 此处的 clone 无复制开销，仅增加引用数
        let next = self.head.clone();
        let node = Node { elem, next };
        SharedSinglyLinked { head: Some(Rc::new(node)) }
    }

    pub fn per_tail(&self) -> Self {
        let head = self.head.as_ref();
        let next = head.and_then(|node| node.next.clone());
        SharedSinglyLinked { head: next }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

impl<T> Drop for SharedSinglyLinked<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::SharedSinglyLinked;

    #[test]
    fn basics() {
        let list = SharedSinglyLinked::new();
        assert_eq!(list.peek(), None);

        let list = list
            .pre_append(1).pre_append(2).pre_append(3);
        assert_eq!(list.peek(), Some(&3));

        let list = list.per_tail();
        assert_eq!(list.peek(), Some(&2));

        let list = list.per_tail();
        assert_eq!(list.peek(), Some(&1));

        let list = list.per_tail();
        assert_eq!(list.peek(), None);

        // Make sure empty tail works
        let list = list.per_tail();
        assert_eq!(list.peek(), None);
    }

    #[test]
    fn iter() {
        let list = SharedSinglyLinked::new()
            .pre_append(1).pre_append(2).pre_append(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
