struct Solution {}
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut remain = [0, 0];
        let mut action = [0, 0];
        let mut senate = senate.chars().collect::<Vec<char>>();
        let mut temp = vec![];
        loop {
            for c in senate.into_iter() {
                match c {
                    'R' => {
                        if action[1] > 0 {
                            action[1] -= 1;
                        } else {
                            remain[0] += 1;
                            action[0] += 1;
                            temp.push(c);
                        }
                    }
                    'D' => {
                        if action[0] > 0 {
                            action[0] -= 1;
                        } else {
                            remain[1] += 1;
                            action[1] += 1;
                            temp.push(c);
                        }
                    }
                    _ => {}
                }
            }
            if remain[0] == 0 || remain[1] == 0 {
                break;
            }
            senate = temp;
            temp = vec![];
            remain = [0, 0];
        }
        if remain[0] == 0 {
            "Dire".to_string()
        } else {
            "Radiant".to_string()
        }
    }
}

pub fn run() {
    let input = ["RD", "RDD", "RDDRDRRDRDRDDDRDRRDRDRDDDRDR"];

    for senate in input {
        println!("{}", Solution::predict_party_victory(senate.to_string()));
    }
}
