// use std::sync::atomic::{AtomicUsize, Ordering};

use once_cell::sync::Lazy;
use std::sync::Mutex;

trait Quackable {
    fn quack(&self);
}

struct MallardDuck {}
impl Quackable for MallardDuck {
    fn quack(&self) {
        println!("Quack");
    }
}
struct RubberDuck {}
impl Quackable for RubberDuck {
    fn quack(&self) {
        println!("Squeak");
    }
}

struct QuackCounter {
    number_of_quacks: i32,
    duck: Box<dyn Quackable>,
}
impl QuackCounter {
    fn new(duck: Box<dyn Quackable>) -> Self {
        Self {
            number_of_quacks: 0,
            duck,
        }
    }
    fn quack(&mut self) {
        self.duck.quack();
        self.number_of_quacks = self.number_of_quacks + 1;
        *COUNT.lock().unwrap() += 1;
    }
    fn get_quacks(&self) -> i32 {
        self.number_of_quacks;
        *COUNT.lock().unwrap()
    }
}

static COUNT: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

fn main() {
    let mut counter = QuackCounter::new(Box::new(MallardDuck {}));
    counter.quack();
    counter.quack();
    println!("{}", counter.get_quacks());
    let mut counter2 = QuackCounter::new(Box::new(MallardDuck {}));
    counter2.quack();
    println!("{}", counter2.get_quacks());
}
