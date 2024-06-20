struct Solution {}
impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let len = questions.len();
        let mut ans = vec![0; len + 1];
        for i in (0..len).rev() {
            let power = questions[i][1] as usize;
            let point = questions[i][0] as i64 + ans.get(i + power + 1).unwrap_or(&0);
            ans[i] = ans[i + 1].max(point);
        }
        ans[0]
    }

    pub fn _most_points(questions: Vec<Vec<i32>>) -> i64 {
        Solution::_helper(&questions, &mut vec![-1; questions.len()], 0)
    }

    pub fn _helper(questions: &Vec<Vec<i32>>, cache: &mut Vec<i64>, i: usize) -> i64 {
        if i >= questions.len() {
            return 0;
        }
        if cache[i] != -1 {
            return cache[i];
        }

        let point = questions[i][0] as i64;
        let power = questions[i][1] as usize;
        let sum1 = point + Solution::_helper(questions, cache, i + power + 1);
        let sum2 = Solution::_helper(questions, cache, i + 1);
        cache[i] = sum1.max(sum2);
        cache[i]
    }
}

pub fn run() {
    let input = [
        vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]],
        vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]],
        vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 3],
            vec![4, 4],
            vec![999, 999],
        ],
    ];

    for questions in input {
        println!("{:?}", Solution::most_points(questions));
    }
}
