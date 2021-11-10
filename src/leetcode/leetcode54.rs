struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return vec![];
        }

        let n = matrix.len();
        let m = matrix[0].len();

        let mut res: Vec<i32> = Vec::with_capacity(n * m);

        let mut offsetx = n - 1;
        let mut offsety = m - 1;

        let mut startx = 0;
        let mut starty = 0;

        let mid = (n / 2).min(m / 2);

        for _ in 0..mid {
            for y in starty..(starty + offsety) {
                res.push(matrix[startx][y]);
            }

            for x in startx..(startx + offsetx) {
                res.push(matrix[x][starty + offsety]);
            }

            for y in ((starty + 1)..(starty + offsety + 1)).rev() {
                res.push(matrix[startx + offsetx][y]);
            }

            for x in ((startx + 1)..(startx + offsetx + 1)).rev() {
                res.push(matrix[x][starty]);
            }

            startx += 1;
            starty += 1;
            if offsetx >= 2 {
                offsetx -= 2;
            }
            if offsety >= 2 {
                offsety -= 2;
            }
        }

        if n % 2 == 1 && n <= m {
            for y in starty..=(starty + offsety) {
                res.push(matrix[startx][y]);
            }
        } else if m % 2 == 1 && m < n {
            for x in startx..=(startx + offsetx) {
                res.push(matrix[x][starty]);
            }
        }

        res
    }
}

#[test]
fn leetcode54_t1() {
    let matrix = vec![
        vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12], // [1,2,3,4,8,12,11,10,9,5,6,7]
    ];

    let r = Solution::spiral_order(matrix);
    println!("{:?}", r);

    let matrix = vec![
        vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], // [1, 2, 3, 6, 9, 8, 7, 4, 5]
    ];

    let r = Solution::spiral_order(matrix);
    println!("{:?}", r);

    let matrix = vec![
        vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10],  // [1, 2, 3, 4, 5, 10, 9, 8, 7, 6]
    ];

    let r = Solution::spiral_order(matrix);
    println!("{:?}", r);
}

