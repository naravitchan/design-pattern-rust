trait Duck {
    fn quack(&self);
    fn fly(&self);
}

struct MallardDuck;
impl Duck for MallardDuck {
    fn quack(&self) {
        println!("Quack!");
    }
    fn fly(&self) {
        println!("I'm flying");
    }
}

trait Turkey {
    fn gobble(&self);
    fn fly(&self);
}

struct WildTurkey;
impl Turkey for WildTurkey {
    fn gobble(&self) {
        println!("Gobble gobble");
    }
    fn fly(&self) {
        println!("I'm flying a short distance")
    }
}

fn test_duck(duck: impl Duck) {
    duck.quack();
    duck.fly();
}

struct TurkeyAdapter {
    turkey: Box<dyn Turkey>,
}

impl TurkeyAdapter {
    fn new(turkey: Box<dyn Turkey>) -> Self {
        Self { turkey }
    }
}
impl Duck for TurkeyAdapter {
    fn quack(&self) {
        self.turkey.gobble();
    }
    fn fly(&self) {
        for _ in 0..5 {
            self.turkey.fly();
        }
    }
}

fn main() {
    test_duck(MallardDuck);
    test_duck(TurkeyAdapter::new(Box::new(WildTurkey)))
}
