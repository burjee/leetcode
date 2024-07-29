pub fn strs_to_string(strs: Vec<&str>) -> Vec<String> {
    strs.into_iter().map(|s| s.to_string()).collect()
}

pub fn vec_strs_to_string(vec_strs: Vec<Vec<&str>>) -> Vec<Vec<String>> {
    vec_strs.into_iter().map(|strs| strs_to_string(strs)).collect()
}
