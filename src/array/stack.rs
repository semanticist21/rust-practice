pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { data: vec![] }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.data.len() == 0 {
            return None;
        }

        Some(self.data.last().unwrap())
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[test]
fn test_pop() {
    let mut stack = Stack::<i32>::new();

    stack.push(3);
    stack.push(1);
    stack.push(7);
    stack.push(2);
    stack.push(2);

    println!("{:?}", stack.peek().unwrap());
    println!("{:?}", stack.pop().unwrap());
    println!("{:?}", stack.pop().unwrap());
    println!("{:?}", stack.pop().unwrap());
    println!("{:?}", stack.pop().unwrap());
    println!("{:?}", stack.pop().unwrap());
}