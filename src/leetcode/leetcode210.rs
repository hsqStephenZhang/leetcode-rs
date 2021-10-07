use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::with_capacity(num_courses as usize);
        if prerequisites.len() == 0 {
            return (0..num_courses as usize).into_iter().map(|i| i as i32).collect();
        }
        let (table, mut in_degrees) = Self::to_neighbor_table(num_courses, &prerequisites);

        // dbg!(table.clone());
        // dbg!(in_degrees.clone());
        let mut deque: VecDeque<i32> = VecDeque::new();
        for (i, &val) in in_degrees.iter().enumerate() {
            if val == 0 {
                deque.push_back(i as i32);
            }
        }
        // dbg!(deque.clone());

        while let Some(val) = deque.pop_front() {
            res.push(val);
            for &neighbor in table[val as usize].iter() {
                in_degrees[neighbor as usize] -= 1;
                if in_degrees[neighbor as usize] == 0 {
                    deque.push_back(neighbor);
                }
            }
        }
        if res.len() == num_courses as usize {
            res
        } else {
            vec![]
        }
    }

    fn to_neighbor_table(n: i32, prerequisites: &Vec<Vec<i32>>) -> (Vec<Vec<i32>>, Vec<usize>) {
        let mut in_degrees = vec![0; n as usize];
        let mut table = vec![vec![]; n as usize];
        for v in prerequisites.iter() {
            debug_assert_eq!(v.len(), 2);
            table[v[1] as usize].push(v[0]);
            in_degrees[v[0] as usize] += 1;
        }
        (table, in_degrees)
    }
}

#[test]
fn leetcode207_t1() {
    let prerequisites = vec![vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]];
    let num_courses = 5;
    let r = Solution::find_order(num_courses, prerequisites);
    assert_eq!(r, [0, 4, 1, 2, 3]);
}
