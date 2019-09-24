use std::mem;

struct MyCircularDeque {
    len: usize,
    capacity: usize,
    front: Vec<i32>,
    back: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {

    /** Initialize your data structure here. Set the size of the deque to be k. */
    fn new(k: i32) -> Self {
        Self {
            len: 0,
            capacity: k as usize,
            front: Vec::new(),
            back: Vec::new(),
        }
    }

    /** Adds an item at the front of Deque. Return true if the operation is successful. */
    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.front.push(value);
            self.len += 1;
            true
        }
    }

    /** Adds an item at the rear of Deque. Return true if the operation is successful. */
    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.back.push(value);
            self.len += 1;
            true
        }
    }

    /** Deletes an item from the front of Deque. Return true if the operation is successful. */
    fn delete_front(&mut self) -> bool {
        if let Some(_) = self.front.pop() {
            self.len -= 1;
            true
        } else if self.back.len() > 0 {
            self.back.reverse();
            self.front = mem::replace(&mut self.back, Vec::new());
            self.front.pop();
            self.len -= 1;
            true
        } else {
            false
        }
    }

    /** Deletes an item from the rear of Deque. Return true if the operation is successful. */
    fn delete_last(&mut self) -> bool {
        if let Some(_) = self.back.pop() {
            self.len -= 1;
            true
        } else if self.front.len() > 0 {
            self.front.reverse();
            self.back = mem::replace(&mut self.front, Vec::new());
            self.back.pop();
            self.len -= 1;
            true
        } else {
            false
        }
    }

    /** Get the front item from the deque. */
    fn get_front(&self) -> i32 {
        if self.front.len() > 0 {
            self.front.last().cloned().unwrap()
        } else if self.back.len() > 0 {
            self.back[0]
        } else {
            -1
        }
    }

    /** Get the last item from the deque. */
    fn get_rear(&self) -> i32 {
        if self.back.len() > 0 {
            self.back.last().cloned().unwrap()
        } else if self.front.len() > 0 {
            self.front[0]
        } else {
            -1
        }
    }

    /** Checks whether the circular deque is empty or not. */
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    /** Checks whether the circular deque is full or not. */
    fn is_full(&self) -> bool {
        self.len == self.capacity
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */

#[cfg(test)]
mod tests {
    use super::MyCircularDeque;

    #[test]
    fn ex1() {
        let mut deque = MyCircularDeque::new(3);
        assert!(deque.insert_last(1));
        assert!(deque.insert_last(2));
        assert!(deque.insert_front(3));
        assert!(! deque.insert_front(4));
        assert_eq!(2, deque.get_rear());
        assert!(deque.is_full());
        assert!(deque.delete_last());
        assert!(deque.insert_front(4));
        assert_eq!(4, deque.get_front());
    }

    #[test]
    fn delete_back_wrap_front() {
        let mut deque = MyCircularDeque::new(3);
        assert!(deque.is_empty());
        assert!(! deque.is_full());
        assert!(deque.insert_front(1));
        assert!(deque.insert_front(2));
        assert!(deque.insert_front(3));
        assert!(deque.is_full());
        assert_eq!(1, deque.get_rear());
        assert_eq!(3, deque.get_front());
        assert!(deque.delete_last());
        assert_eq!(2, deque.get_rear());
        assert_eq!(3, deque.get_front());
        assert!(deque.delete_last());
        assert_eq!(3, deque.get_rear());
        assert_eq!(3, deque.get_front());
        assert!(deque.delete_last());
        assert_eq!(-1, deque.get_rear());
        assert_eq!(-1, deque.get_front());
        assert!(deque.is_empty());
        assert!(! deque.is_full());
    }

    #[test]
    fn delete_front_wrap_back() {
        let mut deque = MyCircularDeque::new(3);
        assert!(deque.is_empty());
        assert!(! deque.is_full());
        assert!(deque.insert_last(1));
        assert!(deque.insert_last(2));
        assert!(deque.insert_last(3));
        assert!(deque.is_full());
        assert_eq!(3, deque.get_rear());
        assert_eq!(1, deque.get_front());
        assert!(deque.delete_front());
        assert_eq!(3, deque.get_rear());
        assert_eq!(2, deque.get_front());
        assert!(deque.delete_front());
        assert_eq!(3, deque.get_rear());
        assert_eq!(3, deque.get_front());
        assert!(deque.delete_front());
        assert_eq!(-1, deque.get_rear());
        assert_eq!(-1, deque.get_front());
        assert!(deque.is_empty());
        assert!(! deque.is_full());
    }
}
