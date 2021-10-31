use std::collections::HashMap;

struct Solution;

#[inline]
fn foursome(c: char) -> usize {
    match c {
        'A' => 0,
        'C' => 1,
        'G' => 2,
        'T' => 3,
        _ => unreachable!(),
    }
}

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 10 {
            return vec![];
        }
        let mut res = Vec::with_capacity(10);
        let mut sum = 0;
        const M: usize = 1 << 18;

        let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
        for c in s.chars().take(10) {
            sum = sum * 4 + foursome(c);
        }
        groups.insert(sum, vec![0]);

        for ((index, left), right) in s.chars().enumerate().zip(s.chars().skip(10)) {
            sum -= foursome(left) * M;
            sum = sum * 4 + foursome(right);
            let entry = groups.entry(sum).or_default();
            entry.push(index + 1);
            if entry.len() == 2 {
                let s = String::from(&s[(index + 1)..(index + 11)]);
                res.push(s);
            }
        }

        res
    }
}

#[test]
fn leetcode187_t1() {
    // ["AAAAACCCCC","CCCCCAAAAA"]
    let s = String::from("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT");
    let r = Solution::find_repeated_dna_sequences(s);
    println!("{:?}", r);

    // let s = String::from("CCCCCCCCCCC");
    // let r = Solution::find_repeated_dna_sequences(s);
    // println!("{:?}", r);
}
