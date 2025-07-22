use std::collections::VecDeque;

#[derive(Debug)]
struct MyStack {
    main_queue: VecDeque<i32>,
    helper_queue: VecDeque<i32>
}

impl MyStack {

    fn new() -> Self {
        MyStack {
            main_queue: VecDeque::new(),
            helper_queue: VecDeque::new()
        }
    }

    fn push(&mut self, x: i32) {
        self.helper_queue.push_back(x);

        while let Some(val) = self.main_queue.pop_front() {
            self.helper_queue.push_back(val);
        }

        std::mem::swap(&mut self.main_queue, &mut self.helper_queue);
    }
    fn pop(&mut self) -> i32 {
        self.main_queue.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.main_queue.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.main_queue.is_empty()
    }
}


fn main() {
    let mut my_stack = MyStack::new();

    my_stack.push(1);
    my_stack.push(2);
    my_stack.push(3);

    // [3, 2, 1]
    println!("{:?}", my_stack);

    my_stack.pop();

    // [2, 1]
    println!("{:?}", my_stack);

    let top_element = my_stack.top();

    // 2
    println!("{}", top_element);

    let is_empty = my_stack.empty();

    // false
    println!("{}", is_empty);
}
