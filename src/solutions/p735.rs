struct Solution {}
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];

        for asteroid in asteroids {
            if ans.is_empty() || asteroid > 0 {
                ans.push(asteroid);
            } else {
                loop {
                    let last = *ans.last().unwrap();
                    if last < 0 {
                        ans.push(asteroid);
                        break;
                    } else if last == asteroid.abs() {
                        ans.pop();
                        break;
                    } else if last > asteroid.abs() {
                        break;
                    }

                    ans.pop();
                    if ans.is_empty() {
                        ans.push(asteroid);
                        break;
                    }
                }
            }
        }

        ans
    }
}

pub fn run() {
    let input = [vec![5, 10, -5], vec![8, -8], vec![10, 2, -5]];

    for asteroids in input {
        println!("{:?}", Solution::asteroid_collision(asteroids));
    }
}
