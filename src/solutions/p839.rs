struct Solution {}
impl Solution {
    pub fn num_similar_groups(mut strs: Vec<String>) -> i32 {
        fn is_similar(s0: &str, s1: &str) -> bool {
            let mut count = 0;
            for (c0, c1) in s0.as_bytes().iter().zip(s1.as_bytes().iter()) {
                if c0 != c1 {
                    count += 1;
                    if count > 2 {
                        return false;
                    }
                }
            }
            true
        }

        let mut temp = vec![];
        let mut groups: Vec<Vec<String>> = vec![vec![strs.pop().unwrap()]];
        let mut oc = 0;
        while oc < groups.len() {
            let mut ic = 0;
            while ic < groups[oc].len() {
                for s in strs {
                    if is_similar(&groups[oc][ic], &s) {
                        groups[oc].push(s);
                    } else {
                        temp.push(s);
                    }
                }
                strs = temp;
                temp = vec![];
                ic += 1;
            }
            oc += 1;
            if oc == groups.len() && !strs.is_empty() {
                groups.push(vec![strs.pop().unwrap()])
            }
        }
        groups.len() as i32
    }
}

pub fn run() {
    let input = [vec!["tars", "rats", "arts", "star"], vec!["omv", "ovm"]];
    for strs in input {
        let strs = strs.into_iter().map(|s| s.to_string()).collect();
        println!("{:?}", Solution::num_similar_groups(strs));
    }
}
