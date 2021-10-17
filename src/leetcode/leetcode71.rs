struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let splited = path.split("/").collect::<Vec<_>>();
        let mut res = vec![];
        splited
            .iter()
            .filter(|&&s| s.len() != 0 && s != "/")
            .for_each(|&s| {
                // handle `..`
                if s == ".." {
                    if res.len() != 0 {
                        res.pop();
                    }
                    return;
                }
                // if not `.`
                if s != "." {
                    res.push(s)
                }
            });
        // have to put `/` at the beginning, but cannot do it through join, so just format it
        // it's also zero cost, no need to crate another string
        format!("/{}", res.join("/"))
    }
}

#[test]
fn leetcode71_t1() {
    let path = "/../".into();
    let r = Solution::simplify_path(path);
    println!("{}", r);
}
