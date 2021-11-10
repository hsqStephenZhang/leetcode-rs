struct Solution;

impl Solution {
    // 关键在于，什么情况下步数+1
    // 将我们走过的每一步记为 next，那么，只有当遍历的下标等于 next 的时候，step 才能加一
    // 并且，当我们走到 next 位置时，需要更新目标位置，也就是 max_pos
    // 在走到下一个 next 位置之前，max_pos 想走多远走多远，越远越好，next 都不需要关心
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut next = nums[0] as usize;
        let mut max_pos = next;
        let mut step = 0;
        let last = nums.len() - 1;
        for (index, val) in nums.iter().map(|v| *v as usize).enumerate().skip(1) {
            max_pos = max_pos.max(index + val);
            if index == next || index == last {
                step += 1;
                next = max_pos;
            }
        }

        step
    }
}

#[test]
fn leetcode45_t1() {
    let nums = vec![2, 3, 1, 1, 4];
    let r = Solution::jump(nums);
    println!("{}", r);

    let nums = vec![2, 1];
    let r = Solution::jump(nums);
    println!("{}", r);
}
