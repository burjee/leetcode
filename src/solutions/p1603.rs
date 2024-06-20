struct ParkingSystem {
    big: i32,
    medium: i32,
    small: i32,
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem { big, medium, small }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        let space = match car_type {
            1 => &mut self.big,
            2 => &mut self.medium,
            _ => &mut self.small,
        };
        if space == &0 {
            return false;
        }
        *space -= 1;
        true
    }
}

/**
 * Your ParkingSystem object will be instantiated and called as such:
 * let obj = ParkingSystem::new(big, medium, small);
 * let ret_1: bool = obj.add_car(carType);
 */

pub fn run() {
    enum Cmd {
        Init { big: i32, medium: i32, small: i32 },
        Add(i32),
    }

    let input = [
        Cmd::Init {
            big: 1,
            medium: 1,
            small: 0,
        },
        Cmd::Add(1),
        Cmd::Add(2),
        Cmd::Add(3),
        Cmd::Add(1),
    ];

    let mut parking_system = ParkingSystem::new(-1, -1, -1);
    for cmd in input {
        match cmd {
            Cmd::Init { big, medium, small } => {
                parking_system = ParkingSystem::new(big, medium, small);
            }
            Cmd::Add(car_type) => {
                println!("{}", parking_system.add_car(car_type));
            }
        }
    }
}
