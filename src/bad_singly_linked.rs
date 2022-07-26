pub struct BadSinglyLinked {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl Default for Link {
    fn default() -> Self {
        Link::Empty
    }
}

struct Node {
    elem: i32,
    next: Link,
}

impl BadSinglyLinked {
    pub fn new() -> Self {
        BadSinglyLinked { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let elem = elem;
        // 此处只能使用 replace 替换原值，如果直接将原值拿出来会出现：
        // head: ???
        // 即无法确定原值是啥，有点像内存泄露，replace 可以用一个临时值替换，
        // 本质是一个 unsafe 绕过这个限制
        // let next = std::mem::replace(&mut self.head, Link::Empty);
        // 和上面的本质是一致的，只不过第二个参数使用 default() 生成
        let next = std::mem::take(&mut self.head);
        let node = Node { elem, next };
        self.head = Link::More(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        // 这里使用 replace 的原因和 push 相同
        let head = std::mem::take(&mut self.head);
        match head {
            Link::Empty => {
                self.head = Link::Empty;
                None
            }
            Link::More(n) => {
                self.head = n.next;
                Some(n.elem)
            }
        }
    }
}

impl Drop for BadSinglyLinked {
    fn drop(&mut self) {
        let mut cur_link = std::mem::take(&mut self.head);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = std::mem::take(&mut boxed_node.next);
            // boxed_node 在这里超出作用域并被 drop,
            // 由于它的 `next` 字段拥有的 `Node` 被设置为 Link::Empty,
            // 因此这里并不会有无边界的递归发生
        }
    }
}

#[cfg(test)]
mod test {
    use super::BadSinglyLinked;

    #[test]
    fn basics() {
        let mut list = BadSinglyLinked::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
