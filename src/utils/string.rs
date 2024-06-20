pub fn strs_to_string(strs: Vec<&str>) -> Vec<String> {
    strs.into_iter().map(|s| s.to_string()).collect()
}
