struct Solution;

impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        // the remained counting of each byte in s.as_bytes()
        let mut counter: Vec<usize> = vec![0; 26];
        // visited: if the current stack contains the val from 'a' to 'z'
        let mut visited: Vec<bool> = vec![false; 26];
        for i in s.as_bytes().iter() {
            counter[(*i - b'a') as usize] += 1;
        }
        let mut stack = Vec::<u8>::new();

        for c in s.as_bytes().into_iter() {
            if visited[(*c - b'a') as usize] == false {
                let mut top_val=b'a';
                while match stack.last() {
                    None => false,
                    Some(&tmp) => {
                        top_val = tmp;
                        // only when the pop can make the prefix smaller and we can find the latter top_val, we can pop
                        // example:
                        //  for string bab, when the stack is b, we known that somehow the stack will turn to ab,which is better
                        //  so we just pop the b,push the a, compared with the normal one ba, it just acts like swap into ab
                        //  but in a totally different way
                        if (top_val > *c) && counter[(top_val - b'a') as usize] != 0 {
                            true
                        } else {
                            false
                        }
                    }
                } {
                    // if we pop one from the top, then it no longer exists in the stack
                    visited[(top_val - b'a') as usize] = false;
                    stack.pop();
                }
                visited[(*c - b'a') as usize] = true;
                // we are about to push one val into the stack, or simply drop it, anyway, the counter should dec
                stack.push(*c);
            }
            counter[(*c - b'a') as usize] -= 1;
        }
        String::from_utf8(stack).unwrap()
    }
}

#[test]
fn leetcode316_t1() {
    let s = "cbacdcb".into();
    let r = Solution::smallest_subsequence(s);
    println!("{}", &r);

    let s = "bcabc".into();
    let r = Solution::smallest_subsequence(s);
    println!("{}", &r);

    let s = "aabbcc".into();
    let r = Solution::smallest_subsequence(s);
    println!("{}", &r);
}
