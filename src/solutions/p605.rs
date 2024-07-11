struct Solution {}
impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        if n == 0 {
            return true;
        }

        for i in 0..flowerbed.len() {
            if flowerbed[i] == 0 {
                if i == 0 || flowerbed[i - 1] == 0 {
                    if i == flowerbed.len() - 1 || flowerbed[i + 1] == 0 {
                        flowerbed[i] = 1;
                        n -= 1;
                        if n == 0 {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

pub fn run() {
    let input = [(vec![1, 0, 0, 0, 1], 1), (vec![1, 0, 0, 0, 1], 2)];

    for (flowerbed, n) in input {
        println!("{}", Solution::can_place_flowers(flowerbed, n));
    }
}
