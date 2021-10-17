use std::ops::{Add, Div, Mul, Sub};

struct Solution;

impl Solution {
    // SAFETY: we known the test cases are valid, so we can use unwrap
    //         otherwise we should handle the None case explictly
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<String> = Vec::with_capacity(tokens.len() / 2);
        // cannot move stack into the closure,
        // because we have to use the mut reference of stack to push into the result of ops
        let apply = |s: &mut Vec<String>, f: fn(i32, i32) -> i32| {
            let right = s.pop().unwrap();
            let left = s.pop().unwrap();
            let right = right.as_str().parse::<i32>().unwrap();
            let left = left.as_str().parse::<i32>().unwrap();
            let r = f(left, right);
            s.push(r.to_string());
        };
        tokens.iter().for_each(|t| match t.as_str() {
            "*" => apply(&mut stack, i32::mul),
            "/" => apply(&mut stack, i32::div),
            "+" => apply(&mut stack, i32::add),
            "-" => apply(&mut stack, i32::sub),
            _ => {
                stack.push(t.clone());
            }
        });
        stack.pop().unwrap().as_str().parse::<i32>().unwrap()
    }
}

fn into(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|&s| s.into()).collect()
}

#[test]
fn leetcode150_t1() {
    let s = into(vec!["2", "1", "+", "3", "*"]);
    let r = Solution::eval_rpn(s);
    println!("{}", r);
}
