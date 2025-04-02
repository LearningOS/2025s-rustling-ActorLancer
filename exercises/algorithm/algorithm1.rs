use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T: Ord + Clone> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self {
        let mut merged_list = LinkedList::new();
        let mut a_current = list_a.start;
        let mut b_current = list_b.start;
    
        while let (Some(a_ptr), Some(b_ptr)) = (a_current, b_current) {
            unsafe {
                let a_val = &(*a_ptr.as_ptr()).val;
                let b_val = &(*b_ptr.as_ptr()).val;
    
                if a_val <= b_val {
                    merged_list.add((*a_val).clone());
                    a_current = (*a_ptr.as_ptr()).next;
                } else {
                    merged_list.add((*b_val).clone());
                    b_current = (*b_ptr.as_ptr()).next;
                }
            }
        }
    
        let remaining = if a_current.is_some() { a_current } else { b_current };
        let mut current = remaining;
        while let Some(ptr) = current {
            unsafe {
                merged_list.add((*ptr.as_ptr()).val.clone());
                current = (*ptr.as_ptr()).next;
            }
        }
    
        merged_list
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.start;
    
        // 反转链表
        while let Some(mut current_ptr) = current {
            unsafe {
                let next = (*current_ptr.as_ptr()).next; // 保存下一个节点
                (*current_ptr.as_ptr()).next = prev;     // 将当前节点的 next 指向前一个节点
                prev = Some(current_ptr);                // 更新 prev 为当前节点
                current = next;                          // 移动到下一个节点
            }
        }
    
        // 更新 start 和 end 指针
        self.start = prev;
        // 如果链表为空，end 设为 None；否则找到最后一个节点
        self.end = if self.start.is_none() {
            None
        } else {
            let mut temp = self.start;
            unsafe {
                while let Some(ptr) = temp {
                    if (*ptr.as_ptr()).next.is_none() {
                        temp = Some(ptr);
                        break;
                    }
                    temp = (*ptr.as_ptr()).next;
                }
            }
            temp
        };
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    // #[test]
    // fn test_reverse_linked_list_1() {
    //     let mut list = LinkedList::<i32>::new();
    //     let vec = vec![2, 3, 5, 11, 9, 7];
    //     for &val in &vec {
    //         list.add(val);
    //     }
    //     println!("Linked List is {}", list);
    //     list.reverse();
    //     println!("Reversed Linked List is {}", list);
    //     for (i, &val) in vec.iter().rev().enumerate() {
    //         assert_eq!(val, *list.get(i as i32).unwrap());
    //     }
    // }

    // #[test]
    // fn test_reverse_linked_list_2() {
    //     let mut list = LinkedList::<i32>::new();
    //     let vec = vec![34, 56, 78, 25, 90, 10, 19, 34, 21, 45];
    //     for &val in &vec {
    //         list.add(val);
    //     }
    //     println!("Linked List is {}", list);
    //     list.reverse();
    //     println!("Reversed Linked List is {}", list);
    //     for (i, &val) in vec.iter().rev().enumerate() {
    //         assert_eq!(val, *list.get(i as i32).unwrap());
    //     }
    // }
}