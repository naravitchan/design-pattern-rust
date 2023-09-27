trait Duck {
    fn name(&self) -> &'static str;
    // fn get_fly_behavior(&self) -> dyn Fly;

    fn quack(&self) {
        println!("{} say quack!", self.name());
    }
    fn swim(&self) {
        println!("swim!");
    }
    fn display(&self) {
        println!("display");
    }
    fn perform_fly(&self) {
        // self.get_fly_behavior.fly()
        println!("perform fly");
    }
}
trait Fly {
    fn fly(&self);
}
struct FlyWithWings {}
impl Fly for FlyWithWings {
    fn fly(&self) {
        println!("fly");
    }
}

struct FlyNoWay {}
impl Fly for FlyNoWay {
    fn fly(&self) {
        println!("can not fly");
    }
}

struct MallardDuck {
    name: &'static str,
    fly_behavior: Box<dyn Fly>,
}
impl MallardDuck {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            fly_behavior: Box::new(FlyWithWings {}),
        }
    }
}
impl Duck for MallardDuck {
    fn name(&self) -> &'static str {
        self.name
    }
    fn display(&self) {
        println!("display like a MallardDuck");
    }
    // cannot do
    // fn get_fly_behavior(&self) -> impl Fly {
    //     self.fly_behavior
    // }
    fn perform_fly(&self) {
        self.fly_behavior.fly();
    }
}

struct RedHeadDuck {
    name: &'static str,
    fly_behavior: Box<dyn Fly>,
}
impl RedHeadDuck {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            fly_behavior: Box::new(FlyWithWings {}),
        }
    }
}
impl Duck for RedHeadDuck {
    fn name(&self) -> &'static str {
        self.name
    }
    fn display(&self) {
        println!("display like a RedDuck");
    }
    fn perform_fly(&self) {
        self.fly_behavior.fly();
    }
}

struct RubberDuck {
    name: &'static str,
    fly_behavior: Box<dyn Fly>,
}

impl RubberDuck {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            fly_behavior: Box::new(FlyNoWay {}),
        }
    }
}
impl Duck for RubberDuck {
    fn name(&self) -> &'static str {
        self.name
    }
    fn display(&self) {
        println!("display like a RubberDuck");
    }
    fn perform_fly(&self) {
        self.fly_behavior.fly();
    }
}

fn main() {
    println!("Hello, main world!");
    let d1 = MallardDuck::new("d1");
    let d2 = RedHeadDuck::new("d2");
    let d3 = RubberDuck::new("d3");
    d1.display();
    d1.quack();
    d1.perform_fly();
    d2.display();
    d2.quack();
    d2.perform_fly();
    d3.display();
    d3.quack();
    d3.perform_fly();
}
