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

trait QuackBehavior {
    fn quack(&self);
}

struct Quack {}
impl QuackBehavior for Quack {
    fn quack(&self) {
        println!("Quack");
    }
}

struct Squeak {}
impl QuackBehavior for Squeak {
    fn quack(&self) {
        println!("Quack squeak");
    }
}

enum QuackType {
    quack(Quack),
    squeak(Squeak),
}

struct Duck {
    fly_behavior: Box<dyn FlyBehavior>,
    quack_behavior: QuackType,
}

impl Duck {
    fn perform_fly(&self) {
        self.fly_behavior.fly()
    }
    fn perform_quack(&self) {
        match &self.quack_behavior {
            QuackType::quack(q) => q.quack(),
            QuackType::squeak(q) => q.quack(),
        }
    }

    fn set_fly_behavior(&mut self, a: impl FlyBehavior + 'static) {
        self.fly_behavior = Box::new(a);
    }

    fn set_quack_behavior(&mut self, a: QuackType) {
        self.quack_behavior = a;
    }
}

fn main() {
    println!("Hello, main world!");
    let mut d = Duck {
        fly_behavior: Box::new(FlyWithWings {}),
        quack_behavior: QuackType::quack(Quack {}),
    };
    d.perform_fly();
    d.perform_quack();
    d.set_fly_behavior(FlyNoWay {});
    d.set_quack_behavior(QuackType::squeak(Squeak {}));
    d.perform_fly();
    d.perform_quack();
}
