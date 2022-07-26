use std::mem;

pub struct UnsafeLinked<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> UnsafeLinked<T> {
    pub fn new() -> Self {
        UnsafeLinked { head: None, tail: std::ptr::null_mut() }
    }

    pub fn push(&mut self, elem: T) {
        // 在尾端推入一个新节点时，新节点的下一个节点永远是 None
        let node = Node {elem, next: None};
        let mut new_tail = Box::new(node);

        let raw_tail: *mut _ = &mut *new_tail;

        // .is_null 会检查是否为 null, 在功能上等价于 `None` 的检查
        if !self.tail.is_null() {
            // 如果 old tail 存在，那将其指向新的 tail
            unsafe { (*self.tail).next = Some(new_tail); }
        } else {
            // 否则让 head 指向新的 tail
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }

            head.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::UnsafeLinked;
    #[test]
    fn basics() {
        let mut list = UnsafeLinked::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }
}
