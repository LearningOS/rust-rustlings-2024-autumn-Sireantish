/*
    queue
    This question requires you to use queues to implement the functionality of the stack
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value);
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

pub struct MyStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::new(),
            q2: Queue::new(),
        }
    }

    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
    }
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty()
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        // 检查栈是否为空
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }
    
        // 将 q1 中的元素转移到 q2 中，直到只剩下一个元素
        while self.q1.size() > 1 {
            let value = self.q1.dequeue()?; // 从 q1 中移除元素
            self.q2.enqueue(value); // 将其放入 q2
        }
    
        // 弹出 q1 中剩下的最后一个元素
        let popped_value = self.q1.dequeue()?; 
    
        // 将 q2 中的所有元素再转回 q1 中
        while !self.q2.is_empty() {
            let value = self.q2.dequeue().unwrap(); // 从 q2 中获取元素
            self.q1.enqueue(value); // 将元素放回 q1
        }
    
        Ok(popped_value) // 返回弹出的元素
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut s: MyStack<i32> = MyStack::new();    
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
