trait Duck {
    fn name(&self) -> &'static str;

    fn quack(&self) {
        println!("{} say quack!", self.name());
    }
    fn swim(&self) {
        println!("swim!");
    }
    fn display(&self) {
        println!("display");
    }
    // new feature
    fn fly(&self) {
        // update possition
        println!("fly");
    }
}

struct MallardDuck {
    name: &'static str,
}
impl Duck for MallardDuck {
    fn name(&self) -> &'static str {
        self.name
    }
    fn display(&self) {
        println!("display like a MallardDuck");
    }
}

struct RedHeadDuck {
    name: &'static str,
}

impl Duck for RedHeadDuck {
    fn name(&self) -> &'static str {
        self.name
    }
    fn display(&self) {
        println!("display like a RedDuck");
    }
}

struct RubberDuck {
    name: &'static str,
}

impl Duck for RubberDuck {
    fn name(&self) -> &'static str {
        self.name
    }
    fn display(&self) {
        println!("display like a RubberDuck");
    }

    fn quack(&self) {
        println!("{} say quackkkk!", self.name());
    }
}

fn main() {
    println!("Hello, main world!");
    let d1 = MallardDuck { name: "d1" };
    let d2 = RedHeadDuck { name: "d2" };
    let d3 = RubberDuck { name: "d3" };
    d1.display();
    d1.quack();
    d1.fly();

    d2.display();
    d2.quack();
    d2.fly();

    d3.display();
    d3.quack();
    d3.fly();
}
