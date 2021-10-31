struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.is_empty() || s.len() < t.len() { return "".to_string(); }
        let char_to_u8 = |x: char| { ((x as u32) & 0xff) as u8 };

        const LEN: usize = 256;
        let (mut current, mut target) = ([0; LEN], [0; LEN]);

        t.chars().for_each(|x| { target[char_to_u8(x) as usize] += 1; });
        let (mut min_width, mut min_start, mut wnd_start, mut appeared) = (i32::MAX, 0, 0, 0);

        let ss: Vec<usize> = s.chars().map(|x| { (x as u32) as usize }).collect();
        let mut itr = ss.iter().enumerate();

        while let Some((index, &tp)) = itr.next() {
            let index = *x.1;
            if target[index] > 0 {
                current[index] += 1;
                if current[index] <= target[index] {
                    appeared += 1;
                }
            }

            if appeared == t.len() {
                let mut tmp = ss.iter().skip(wnd_start as usize);
                while let Some(y) = tmp.next() {
                    let idx = *y;
                    if (current[idx] > target[idx]) || (target[idx] == 0) {
                        current[idx] -= 1;
                        wnd_start += 1;
                    } else {
                        break;
                    }
                }

                let tp = tp as i32;
                if min_width > (1 + tp - wnd_start) {
                    min_width = 1 + tp - wnd_start;
                    min_start = wnd_start;
                }
            }
        }

        if min_width == std::i32::MAX { "".to_string() } else { s.chars().skip(min_start as usize).take(min_width as usize).collect() }
    }
}

#[test]
fn leetcode76_t1() {
    let s = String::from("");
    let t = String::from("");
    let r = Solution::min_window(s, t);
    println!("{}", r);
}