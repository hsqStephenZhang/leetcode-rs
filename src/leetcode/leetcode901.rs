// (first:the current smallest val,second:the accumulate count)
struct StockSpanner {
    stack: Vec<(i32, usize)>,
}

impl StockSpanner {
    fn new() -> Self {
        Self { stack: vec![] }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut res = 1;
        while match self.stack.last() {
            None => false,
            Some(&(top_price, _)) => top_price <= price,
        } {
            let (_, v) = self.stack.pop().unwrap();
            res += v;
        }
        self.stack.push((price, res));
        res as i32
    }
}

#[test]
fn leetcode901_t1() {
    let mut obj = StockSpanner::new();
    for &val in vec![100, 80, 60, 70, 60, 75, 85].iter() {
        println!("{}", obj.next(val))
    }
}
