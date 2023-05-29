struct Solution {}
impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let mut cache = vec![vec![vec![(-1, -1); piles.len() + 1]; piles.len() + 1]; 2];
        Solution::helper(&piles, &mut cache, 0, 0, 1).0
    }

    pub fn helper(
        piles: &Vec<i32>,
        cache: &mut Vec<Vec<Vec<(i32, i32)>>>,
        p: usize,
        i: usize,
        m: usize,
    ) -> (i32, i32) {
        if i + m * 2 >= piles.len() {
            return (piles[i..piles.len()].iter().fold(0, |acc, x| acc + x), 0);
        }
        if cache[p][i][m] != (-1, -1) {
            return cache[p][i][m];
        }

        let (mut n1, mut n2) = (0, i32::MAX);
        let mut s = 0;

        for j in i..piles.len().min(i + m * 2) {
            s += piles[j];
            let (m1, m2) = Solution::helper(piles, cache, p ^ 1, j + 1, m.max(j - i + 1));
            n1 = n1.max(s + m2);
            n2 = n2.min(m1);
        }
        cache[p][i][m] = (n1, n2);
        cache[p][i][m]
    }

    // pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
    //     let mut cache = vec![vec![vec![-1; piles.len() + 1]; piles.len() + 1]; 2];
    //     Solution::helper(&piles, &mut cache, 0, 0, 1)
    // }

    // pub fn helper(
    //     piles: &Vec<i32>,
    //     cache: &mut Vec<Vec<Vec<i32>>>,
    //     p: usize,
    //     i: usize,
    //     m: usize,
    // ) -> i32 {
    //     if i == piles.len() {
    //         return 0;
    //     }
    //     if cache[p][i][m] != -1 {
    //         return cache[p][i][m];
    //     }

    //     let mut n = if p == 0 { 0 } else { i32::MAX };
    //     let mut s = 0;

    //     for j in i..piles.len().min(i + m * 2) {
    //         s += piles[j];
    //         if p == 0 {
    //             n = n.max(s + Solution::helper(piles, cache, 1, j + 1, m.max(j - i + 1)));
    //         } else {
    //             n = n.min(Solution::helper(piles, cache, 0, j + 1, m.max(j - i + 1)));
    //         }
    //     }
    //     cache[p][i][m] = n;
    //     n
    // }
}
pub fn run() {
    let input = [
        vec![2, 7, 7, 100],
        vec![2, 7, 9, 4, 4],
        vec![1, 2, 3, 4, 5, 100],
        vec![0],
    ];

    for piles in input {
        println!("{}", Solution::stone_game_ii(piles));
    }
}
