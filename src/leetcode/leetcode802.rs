use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut table = vec![vec![]; n];
        for (i, v) in graph.iter().enumerate() {
            for &item in v.iter() {
                table[item as usize].push(i);
            }
        }
        let mut in_degrees = graph.iter().map(|x| x.len()).collect::<Vec<usize>>();

        let mut deque = in_degrees
            .iter()
            .enumerate()
            .filter(|&(_, &v)| v == 0)
            .map(|(i, _)| i)
            .collect::<VecDeque<usize>>();

        while let Some(v) = deque.pop_front() {
            for &val in table[v].iter() {
                in_degrees[val as usize] -= 1;
                if in_degrees[val as usize] == 0 {
                    deque.push_back(val as usize);
                }
            }
        }

        in_degrees
            .iter()
            .enumerate()
            .filter(|&(_, &v)| v == 0)
            .map(|(i, _)| i as i32)
            .collect()
    }
}

#[test]
fn leetcode802_t1() {
    let graph = vec![
        vec![1, 2],
        vec![2, 3],
        vec![5],
        vec![0],
        vec![5],
        vec![],
        vec![],
    ];
    let r = Solution::eventual_safe_nodes(graph);
    println!("{:?}", r);

    let graph = vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]];
    let r = Solution::eventual_safe_nodes(graph);
    println!("{:?}", r);
}
