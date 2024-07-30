struct Solution {}
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable();

        let mut ans = Vec::with_capacity(spells.len());
        for spell in spells {
            let spell = spell as i64;
            let mut mul = success / spell;
            if success % spell > 0 {
                mul += 1;
            }
            let m = potions.partition_point(|&x| (x as i64) < mul);
            ans.push((potions.len() - m) as i32);
        }
        ans
    }
}

pub fn run() {
    let input = [
        (vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7),
        (vec![3, 1, 2], vec![8, 5, 8], 16),
        (vec![1], vec![2, 3, 4], 1),
        (vec![9, 39], vec![35, 40, 22, 37, 29, 22], 320),
    ];

    for (spells, potions, success) in input {
        println!("{:?}", Solution::successful_pairs(spells, potions, success));
    }
}
