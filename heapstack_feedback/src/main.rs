pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }
    
    pub fn push(&mut self, element: T) {
        self.elements.push(element);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }
    
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test1() {
        let mut stack: Stack<i32> = Stack::new();
        assert_eq!(stack.pop(), None);
    }
    
    #[test]
    fn test2() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(stack.peek(), None);
    }
    
    #[test]
    fn test_lifo() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
    
    #[test]
    fn test3() {
        let mut stack = Stack::new();
        stack.push(42);
        assert_eq!(stack.peek(), Some(&42));
        assert_eq!(stack.peek(), Some(&42));
    }
    
    #[test]
    fn test4() {
        let mut stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        stack.push(10);
        assert!(!stack.is_empty());
        stack.pop();
        assert!(stack.is_empty());
    }
}

fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());
    
    println!("Observerer Vec capacity under push:");
    for i in 1..=20 {
        stack.push(i);
        println!("Efter push {}: len={}, capacity={}", 
            i, 
            stack.elements.len(), 
            stack.elements.capacity()
        );
    }
}