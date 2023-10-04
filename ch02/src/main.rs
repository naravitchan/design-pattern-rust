struct WeatherData {
    temp: i32,
    humid: i32,
    pressure: i32,
}

impl WeatherData {
    fn measurements_changed(&self) {
        let temp = get_temp();
        let humid = get_humid();
        let pressure = get_pressure();

        // update screen1
        // update screen2
        // update screen3
    }
}

fn get_temp() -> i32 {
    10
}

fn get_humid() -> i32 {
    20
}

fn get_pressure() -> i32 {
    30
}

fn main() {
    println!("Hello, world!");
}

// do observer
