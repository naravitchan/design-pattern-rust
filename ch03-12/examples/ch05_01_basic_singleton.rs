// use once_cell::sync::Lazy; // 1.3.1
// use std::sync::Mutex;

// https://github.com/lpxxn/rust-design-pattern/blob/master/creational/singleton.rs
// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
struct Factory {
    num: i32,
    // f: Factory,
}

impl Factory {
    fn push(&mut self) {
        self.num = self.num + 1
    }
    fn get_num(&mut self) -> i32 {
        self.num
    }
    fn get_instance() -> Self {
        Self { num: 0 }
    }
}

struct Singleton {
    // num: i32,
    f: Option<Factory>,
}

impl Singleton {
    fn new() -> Self {
        Self { f: None }
    }
    fn get_factory(&mut self) -> &mut Factory {
        if self.f.is_none() {
            self.create_factory();
        }
        self.f.as_mut().unwrap()
    }
    fn create_factory(&mut self) {
        self.f = Some(Factory::get_instance())
    }
}

fn main() {
    let mut singleton = Singleton::new();
    singleton.get_factory().push();
    singleton.get_factory().push();
    singleton.get_factory().push();
    singleton.get_factory().push();
    let num = singleton.get_factory().get_num();
    println!("{}", num)
}
