trait ObserverTrait {
    fn update(&mut self, temp: i32, humid: i32, pressure: i32);
}

trait DisplayTrait {
    fn display(&self);
}

trait Subject {
    fn registerObserver(&mut self, o: impl ObserverTrait + 'static);
    fn removeObserver(&mut self);
    fn notifyObservers(&self);
}

struct WeatherData {
    observer: Vec<Box<dyn ObserverTrait>>,
    temp: i32,
    humid: i32,
    pressure: i32,
}

impl Subject for WeatherData {
    fn registerObserver(&mut self, o: impl ObserverTrait + 'static) {
        let mut oo = Box::new(o);
        self.observer.push(oo)
    }
    fn removeObserver(&mut self) {
        let _ = self.observer.pop();
    }
    fn notifyObservers(&self) {
        //
        // self.observer
        //     .as_mut()
        //     .downcast_mut::<ObserverTrait>()
        //     .into_iter()
        //     .map(|o| o.update(self.temp, self.humid, self.pressure));

        for o in self.observer {
            o.as_mut().update(self.temp, self.humid, self.pressure)
            // o.update(self.temp, self.humid, self.pressure)
        }
    }
}

impl WeatherData {
    fn new() -> Self {
        Self {
            observer: vec![],
            temp: 0,
            humid: 0,
            pressure: 0,
        }
    }
    fn set_measurements(&mut self, temp: i32, humid: i32, pressure: i32) {
        self.temp = temp;
        self.humid = humid;
        self.pressure = pressure;
        self.measurements_changed();
    }
    fn measurements_changed(&self) {
        self.notifyObservers();
    }
}

struct CurrentConditionsDisplay {
    temp: i32,
    humid: i32,
    // weather_data: WeatherData,
}
impl DisplayTrait for CurrentConditionsDisplay {
    fn display(&self) {
        println!("Current conditions {} {}", self.temp, self.humid);
    }
}
impl ObserverTrait for CurrentConditionsDisplay {
    fn update(&mut self, temp: i32, humid: i32, _pressure: i32) {
        self.temp = temp;
        self.humid = humid;
        self.display()
    }
}
impl CurrentConditionsDisplay {
    fn new() -> Self {
        Self {
            temp: 0,
            humid: 0,
            // weather_data: weather_data,
        }
    }
}
fn main() {
    println!("Hello, world!");
    let mut weather_data = WeatherData::new();
    let d1 = CurrentConditionsDisplay::new();
    weather_data.registerObserver(d1);
    weather_data.set_measurements(10, 20, 30);
}
