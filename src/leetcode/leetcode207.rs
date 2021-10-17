use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        if prerequisites.len() == 0 {
            return true;
        }
        let mut res = 0;
        let (table, mut in_degrees) = Self::to_neighbor_table(num_courses, &prerequisites);

        // dbg!(table.clone());
        // dbg!(in_degrees.clone());
        let mut deque: VecDeque<usize> = VecDeque::new();
        for (i, &val) in in_degrees.iter().enumerate() {
            if val == 0 {
                deque.push_back(i);
            }
        }
        // dbg!(deque.clone());

        while let Some(val) = deque.pop_front() {
            res += 1;
            for &neighbor in table[val].iter() {
                in_degrees[neighbor as usize] -= 1;
                if in_degrees[neighbor as usize] == 0 {
                    deque.push_back(neighbor as usize);
                }
            }
        }
        res == num_courses as usize
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
    let prerequisites = vec![vec![1, 0], vec![0, 1]];
    let num_courses = 2;
    let r = Solution::can_finish(num_courses, prerequisites);
    assert_eq!(r, false);

    let prerequisites = vec![vec![0, 1]];
    let num_courses = 2;
    let r = Solution::can_finish(num_courses, prerequisites);
    assert_eq!(r, true);

    let prerequisites = vec![vec![1, 4], vec![2, 4], vec![3, 1], vec![3, 2]];
    let num_courses = 5;
    let r = Solution::can_finish(num_courses, prerequisites);
    assert_eq!(r, true);
}
