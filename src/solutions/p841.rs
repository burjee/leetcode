struct Solution {}
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![false; rooms.len()];
        let mut remain = rooms.len();
        Solution::visit(&rooms, &mut visited, &mut remain, 0);
        remain == 0
    }

    pub fn visit(rooms: &Vec<Vec<i32>>, visited: &mut Vec<bool>, remain: &mut usize, i: usize) {
        visited[i] = true;
        *remain -= 1;
        for &room in &rooms[i] {
            let room = room as usize;
            if !visited[room] {
                Solution::visit(rooms, visited, remain, room);
            }
        }
    }
}

pub fn run() {
    let input = [
        vec![vec![1], vec![2], vec![3], vec![]],
        vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]],
    ];

    for rooms in input {
        println!("{}", Solution::can_visit_all_rooms(rooms));
    }
}
