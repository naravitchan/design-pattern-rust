use core::cell::RefCell;
use std::rc::{Rc, Weak};

trait Observer {
    fn update(&mut self, temperature: f64, humidity: f64, pressure: f64);
}

trait DisplayElement {
    fn display(&self);
}

trait Subject {
    fn register_observer(&mut self, o: Weak<RefCell<dyn Observer>>);
    fn remove_observer(&mut self, o: Weak<RefCell<dyn Observer>>);
    fn notify_observers(&self);
}

struct WeatherData {
    observers: Vec<Weak<RefCell<dyn Observer>>>,
    temperature: f64,
    humidity: f64,
    pressure: f64,
}

impl WeatherData {
    fn new() -> Self {
        WeatherData {
            observers: Vec::new(),
            temperature: 0.0,
            humidity: 0.0,
            pressure: 0.0,
        }
    }

    fn set_measurements(&mut self, temperature: f64, humidity: f64, pressure: f64) {
        self.temperature = temperature;
        self.humidity = humidity;
        self.pressure = pressure;
        self.measurements_changed();
    }

    fn measurements_changed(&self) {
        self.notify_observers();
    }
}

impl Subject for WeatherData {
    fn register_observer(&mut self, o: Weak<RefCell<dyn Observer>>) {
        self.observers.push(o);
    }
    fn remove_observer(&mut self, o: Weak<RefCell<dyn Observer>>) {
        if let Some(idx) = self.observers.iter().position(|x| Weak::ptr_eq(x, &o)) {
            self.observers.remove(idx);
        }
    }

    fn notify_observers(&self) {
        for observer in self.observers.iter() {
            observer
                .upgrade()
                .unwrap()
                .try_borrow_mut()
                .unwrap()
                .update(self.temperature, self.humidity, self.pressure);
        }
    }
}

struct CurrentConditionsDisplay {
    temperature: f64,
    humidity: f64,
    weather_data: Weak<RefCell<WeatherData>>,
}

impl CurrentConditionsDisplay {
    fn new(weather_data: Weak<RefCell<WeatherData>>) -> Rc<RefCell<dyn Observer>> {
        let display = CurrentConditionsDisplay {
            temperature: 0.0,
            humidity: 0.0,
            weather_data: weather_data.clone(),
        };
        let data: Rc<RefCell<dyn Observer>> = Rc::new(RefCell::new(display));
        let weak = Rc::downgrade(&data);
        weather_data
            .upgrade()
            .unwrap()
            .try_borrow_mut()
            .unwrap()
            .register_observer(weak);
        data
    }
}
impl DisplayElement for CurrentConditionsDisplay {
    fn display(&self) {
        println!(
            "Current conditions: {} F degrees and  {} % humidity",
            self.temperature, self.humidity
        )
    }
}

impl Observer for CurrentConditionsDisplay {
    fn update(&mut self, temperature: f64, humidity: f64, pressure: f64) {
        self.temperature = temperature;
        self.humidity = humidity;
        self.display();
    }
}

struct StatisticsDisplay {
    temperature: f64,
    pressure: f64,
    weather_data: Weak<RefCell<WeatherData>>,
}

impl StatisticsDisplay {
    fn new(weather_data: Weak<RefCell<WeatherData>>) -> Rc<RefCell<dyn Observer>> {
        let display = StatisticsDisplay {
            temperature: 0.0,
            pressure: 0.0,
            weather_data: weather_data.clone(),
        };
        let data: Rc<RefCell<dyn Observer>> = Rc::new(RefCell::new(display));
        let weak = Rc::downgrade(&data);
        weather_data
            .upgrade()
            .unwrap()
            .try_borrow_mut()
            .unwrap()
            .register_observer(weak);
        data
    }
}
impl DisplayElement for StatisticsDisplay {
    fn display(&self) {
        println!(
            "Static: {} F degrees and  {} pressure",
            self.temperature, self.pressure
        )
    }
}

impl Observer for StatisticsDisplay {
    fn update(&mut self, temperature: f64, humidity: f64, pressure: f64) {
        self.temperature = temperature;
        self.pressure = pressure;
        self.display();
    }
}

fn main() {
    let weather_data = WeatherData::new();
    let weather_data = Rc::new(RefCell::new(weather_data));
    let weak = Rc::downgrade(&weather_data);
    let current_conditions_display = CurrentConditionsDisplay::new(weak.clone());
    let static_display = StatisticsDisplay::new(weak.clone());

    weather_data
        .try_borrow_mut()
        .unwrap()
        .set_measurements(80.0, 65.0, 30.4);
    weather_data
        .try_borrow_mut()
        .unwrap()
        .set_measurements(82.0, 70.0, 29.2);

    // weather_data
    //     .try_borrow_mut()
    //     .unwrap()
    //     .remove_observer(Rc::downgrade(&static_display));

    weather_data
        .try_borrow_mut()
        .unwrap()
        .remove_observer(Rc::downgrade(&current_conditions_display));

    weather_data
        .try_borrow_mut()
        .unwrap()
        .set_measurements(78.0, 90.0, 29.2);
}
