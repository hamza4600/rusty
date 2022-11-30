// don,t error on dead code
#![allow(dead_code)]
#[derive(Debug)]

struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn length(&self) -> usize {
        self.stack.len()
    }

    fn pop(&mut self) -> Option<T> {
        if self.length() == 0 {
            None
        } else {
            Some(self.stack.remove(self.length() - 1))
        }
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }

    
}


fn main() {
    let mut stack: Stack<isize> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.push(40);
    println!("Length of stack is {}", stack.length());
    let rm = stack.pop();
    println!("Removed item is {:?}", rm);
    println!("Length of stack is {}", stack.length());
 
}