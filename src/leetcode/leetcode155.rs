struct MinStack {
    stack: Vec<i32>,
    sentry: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            stack: vec![],
            sentry: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        let top = if self.sentry.len() != 0 {
            self.get_min()
        } else {
            i32::MAX
        };
        self.sentry.push(if val < top { val } else { top });
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.sentry.pop();
    }

    // SAFETY: pop、top 和 getMin 操作总是在 非空栈 上调用。
    fn top(&self) -> i32 {
        let r = self.stack.last().unwrap();
        *r
    }

    // SAFETY: pop、top 和 getMin 操作总是在 非空栈 上调用。
    fn get_min(&self) -> i32 {
        let r = self.sentry.last().unwrap();
        *r
    }
}

#[test]
fn leetcode155_t1() {
    let mut obj = MinStack::new();
    obj.push(1);
    obj.push(3);
    obj.push(2);
    // obj.pop();
    println!("{}", obj.top());
    println!("{}", obj.get_min());
}
