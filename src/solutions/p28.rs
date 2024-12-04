struct Solution {}
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        let mut i = 0;
        let (mut l, mut r);
        while i < haystack.len() {
            if haystack[i] == needle[0] {
                l = i;
                r = 0;
                while l < haystack.len() && haystack[l] == needle[r] {
                    l += 1;
                    r += 1;
                    if r == needle.len() {
                        return i as i32;
                    }
                }
            }
            i += 1;
        }
        -1
    }
}

pub fn run() {
    let input = [("sadbutsad", "sad"), ("leetcode", "leeto"), ("mississippi", "issip")];

    for (haystack, needle) in input {
        println!("{}", Solution::str_str(haystack.to_string(), needle.to_string()));
    }
}
