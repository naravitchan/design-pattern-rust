trait FlyBehavior {
    fn fly(&self);
}

struct FlyWithWings {}
impl FlyBehavior for FlyWithWings {
    fn fly(&self) {
        println!("can fly");
    }
}

struct FlyNoWay {}
impl FlyBehavior for FlyNoWay {
    fn fly(&self) {
        println!("cannot fly");
    }
}

struct Duck {
    fly_behavior: Box<dyn FlyBehavior>,
}

impl Duck {
    fn perform_fly(&self) {
        self.fly_behavior.fly()
    }

    fn set_fly_behavior(&mut self, a: impl FlyBehavior + 'static) {
        self.fly_behavior = Box::new(a);
    }
}

fn main() {
    println!("Hello, main world!");
    let mut d = Duck {
        fly_behavior: Box::new(FlyWithWings {}),
    };
    d.perform_fly();
    d.set_fly_behavior(FlyNoWay {});
    d.perform_fly()
}
