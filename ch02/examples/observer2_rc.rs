use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
// use std::rc::Weak;
// use std::sync::Arc;

trait ObserverTrait {
    fn update(&mut self, temp: i32, humid: i32, pressure: i32);
}

trait DisplayTrait {
    fn display(&self);
}

trait Subject {
    fn registerObserver(&mut self, o: Rc<RefCell<dyn ObserverTrait>>);
    fn removeObserver(&mut self);
    fn notifyObservers(&self);
}

struct WeatherData {
    observer: Vec<Rc<RefCell<dyn ObserverTrait>>>,
    temp: i32,
    humid: i32,
    pressure: i32,
}

impl Subject for WeatherData {
    fn registerObserver(&mut self, o: Rc<RefCell<dyn ObserverTrait>>) {
        self.observer.push(o)
    }
    fn removeObserver(&mut self) {
        let _ = self.observer.pop();
    }
    fn notifyObservers(&self) {
        for o in &self.observer {
            o.try_borrow_mut()
                .unwrap()
                .update(self.temp, self.humid, self.pressure);
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
    let d1: CurrentConditionsDisplay = CurrentConditionsDisplay::new();

    // let c = RefCell::new(d1);
    // // https://doc.rust-lang.org/std/rc/struct.Weak.html
    // let strong = Rc::new(c);
    // let weak_d1 = Rc::downgrade(&strong);
    let d1_rc = Rc::new(RefCell::new(d1));
    weather_data.registerObserver(d1_rc);
    weather_data.set_measurements(10, 20, 30);
}
