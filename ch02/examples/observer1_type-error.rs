use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

trait ObserverTrait {
    fn update(&mut self, temp: i32, humid: i32, pressure: i32);
}

trait DisplayTrait {
    fn display(&self);
}

// https://github.com/lpxxn/rust-design-pattern
// https://stackoverflow.com/questions/37572734/how-can-i-implement-the-observer-pattern-in-rust
trait Subject<'a, T: ObserverTrait> {
    fn registerObserver(&mut self, o: &'a T);
    fn removeObserver(&mut self);
    fn notifyObservers(&self);
}

struct WeatherData<'a, T: ObserverTrait> {
    observers: Vec<&'a T>,
    temp: i32,
    humid: i32,
    pressure: i32,
}

impl<'a, T: ObserverTrait + PartialEq> Subject<'a, T> for WeatherData<'a, T> {
    fn registerObserver(&mut self, o: &'a T) {
        self.observers.push(o);
    }
    fn removeObserver(&mut self) {
        let _ = self.observers.pop();
    }
    fn notifyObservers(&self) {
        for item in self.observers.iter() {
            item.update(self.temp, self.humid, self.pressure)
        }
    }
}

impl<'a, T: ObserverTrait + PartialEq> WeatherData<'a, T> {
    fn new() -> Self {
        Self {
            observers: vec![],
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

#[derive(PartialEq)]
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
    weather_data.registerObserver(&d1);
    weather_data.set_measurements(10, 20, 30);
}

// Also, observers must outlive observable subjects. Which may be ok in some use cases, but if you want to be able to dynamically add / remove them, you might prefer a solution with Weak, for instance.
