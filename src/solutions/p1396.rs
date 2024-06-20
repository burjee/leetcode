use std::collections::HashMap;

struct UndergroundSystem {
    travel: HashMap<i32, (String, i32)>,
    time: HashMap<(String, String), (i32, i32)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        let travel = HashMap::new();
        let time = HashMap::new();
        UndergroundSystem { travel, time }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.travel.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (start_station, start_t) = self.travel.remove(&id).unwrap();
        let time = self
            .time
            .entry((start_station, station_name))
            .or_insert((0, 0));
        time.0 += t - start_t;
        time.1 += 1;
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let &(total_time, count) = self.time.get(&(start_station, end_station)).unwrap();
        total_time as f64 / count as f64
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */

pub fn run() {
    enum Cmd<'a> {
        Init,
        CheckIn(i32, &'a str, i32),
        CheckOut(i32, &'a str, i32),
        GetAverageTime(&'a str, &'a str),
    }

    let input = [
        vec![
            Cmd::Init,
            Cmd::CheckIn(45, "Leyton", 3),
            Cmd::CheckIn(32, "Paradise", 8),
            Cmd::CheckIn(27, "Leyton", 10),
            Cmd::CheckOut(45, "Waterloo", 15),
            Cmd::CheckOut(27, "Waterloo", 20),
            Cmd::CheckOut(32, "Cambridge", 22),
            Cmd::GetAverageTime("Paradise", "Cambridge"),
            Cmd::GetAverageTime("Leyton", "Waterloo"),
            Cmd::CheckIn(10, "Leyton", 24),
            Cmd::GetAverageTime("Leyton", "Waterloo"),
            Cmd::CheckOut(10, "Waterloo", 38),
            Cmd::GetAverageTime("Leyton", "Waterloo"),
        ],
        vec![
            Cmd::Init,
            Cmd::CheckIn(10, "Leyton", 3),
            Cmd::CheckOut(10, "Paradise", 8),
            Cmd::GetAverageTime("Leyton", "Paradise"),
            Cmd::CheckIn(5, "Leyton", 10),
            Cmd::CheckOut(5, "Paradise", 16),
            Cmd::GetAverageTime("Leyton", "Paradise"),
            Cmd::CheckIn(2, "Leyton", 21),
            Cmd::CheckOut(2, "Paradise", 30),
            Cmd::GetAverageTime("Leyton", "Paradise"),
        ],
    ];

    let mut system = UndergroundSystem::new();
    for commands in input {
        for cmd in commands {
            match cmd {
                Cmd::Init => {
                    system = UndergroundSystem::new();
                    print!("null, ");
                }
                Cmd::CheckIn(id, station_name, t) => {
                    system.check_in(id, station_name.to_string(), t);
                    print!("null, ");
                }
                Cmd::CheckOut(id, station_name, t) => {
                    system.check_out(id, station_name.to_string(), t);
                    print!("null, ");
                }
                Cmd::GetAverageTime(start_station, end_station) => {
                    print!(
                        "{}, ",
                        system.get_average_time(start_station.to_string(), end_station.to_string())
                    );
                }
            }
        }
        println!();
    }
}
