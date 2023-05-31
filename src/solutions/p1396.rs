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

enum Call {
    Init,
    CheckIn(i32, String, i32),
    CheckOut(i32, String, i32),
    GetAverageTime(String, String),
}

pub fn run() {
    let input = [
        vec![
            Call::Init,
            Call::CheckIn(45, "Leyton".into(), 3),
            Call::CheckIn(32, "Paradise".into(), 8),
            Call::CheckIn(27, "Leyton".into(), 10),
            Call::CheckOut(45, "Waterloo".into(), 15),
            Call::CheckOut(27, "Waterloo".into(), 20),
            Call::CheckOut(32, "Cambridge".into(), 22),
            Call::GetAverageTime("Paradise".into(), "Cambridge".into()),
            Call::GetAverageTime("Leyton".into(), "Waterloo".into()),
            Call::CheckIn(10, "Leyton".into(), 24),
            Call::GetAverageTime("Leyton".into(), "Waterloo".into()),
            Call::CheckOut(10, "Waterloo".into(), 38),
            Call::GetAverageTime("Leyton".into(), "Waterloo".into()),
        ],
        vec![
            Call::Init,
            Call::CheckIn(10, "Leyton".into(), 3),
            Call::CheckOut(10, "Paradise".into(), 8),
            Call::GetAverageTime("Leyton".into(), "Paradise".into()),
            Call::CheckIn(5, "Leyton".into(), 10),
            Call::CheckOut(5, "Paradise".into(), 16),
            Call::GetAverageTime("Leyton".into(), "Paradise".into()),
            Call::CheckIn(2, "Leyton".into(), 21),
            Call::CheckOut(2, "Paradise".into(), 30),
            Call::GetAverageTime("Leyton".into(), "Paradise".into()),
        ],
    ];

    let mut system = UndergroundSystem::new();
    for calls in input {
        for call in calls {
            match call {
                Call::Init => {
                    system = UndergroundSystem::new();
                    print!("null, ");
                }
                Call::CheckIn(id, station_name, t) => {
                    system.check_in(id, station_name, t);
                    print!("null, ");
                }
                Call::CheckOut(id, station_name, t) => {
                    system.check_out(id, station_name, t);
                    print!("null, ");
                }
                Call::GetAverageTime(start_station, end_station) => {
                    print!("{}, ", system.get_average_time(start_station, end_station));
                }
            }
        }
        println!();
    }
}
