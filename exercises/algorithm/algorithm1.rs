/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
// I AM NOT DONE

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {   // 单链表节点
    val: T,
    next: Option<NonNull<Node<T>>>, // NonNull是一个裸指针
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {  // new方法
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {  // 单链表结构体
    length: u32,        // 节点数量
    start: Option<NonNull<Node<T>>>, // 首节点
    end: Option<NonNull<Node<T>>>,   // 尾节点
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        // 1. 创建一个新节点
        let mut node = Box::new(Node::new(obj));
        node.next = None;

        // 2. 将节点包装为 Option<NonNull<Node<T>>>
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });

        // 3. 判断当前链表是否为空
        match self.end {
            None => self.start = node_ptr,  // 当前链表为空，直接作为首节点
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr }, // 否则插到链表尾部
        }

        // 4. 更新尾指针和链表长度
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,   // 当前链表为空，直接返回None
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),    // 找到目标节点，返回值
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1), // 递归调用
            },
        }
    }

	pub fn merge(list_a:LinkedList<T>, list_b:LinkedList<T>) -> Self
	{
        let mut dummy = Box::new(Node::new(None));
        let mut tail = unsafe { NonNull::new_unchecked(&mut *dummy) };

        let mut current_a = list_a.start;
        let mut current_b = list_b.start;

        let mut new_length = list_a.length + list_b.length;

        unsafe {
            while current_a.is_some() && current_b.is_some() {
                let a = current_a.unwrap();
                let b = current_b.unwrap();

                if (*a.as_ptr()).val <= (*b.as_ptr()).val {
                    (*tail.as_ptr()).next = Some(a);
                    tail = a;
                    current_a = (*a.as_ptr()).next;
                } else {
                    (*tail.as_ptr()).next = Some(b);
                    tail = b;
                    current_b = (*b.as_ptr()).next;
                }
            }
        }

        if current_a.is_some() {
            (*tail.as_ptr()).next = current_a;
        }

        if current_b.is_some() {
            (*tail.as_ptr()).next = current_b;
        }

        Self {
            length: new_length,
            start: (*dummy.as_ptr()).next,
            end: if current_a.is_none() { list_b.end } else { list_a.end },
        }
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

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}