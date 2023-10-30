use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

// #[derive(Debug)]
// enum State {
//     SOLD_OUT,
//     NO_QUARTER,
//     HAS_QUARTER,
//     SOLD,
// }

trait State: Debug {
    fn insert_quarter(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State>;
    fn eject_quarter(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State>;
    fn turn_crank(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State>;
    fn dispense(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State>;
}

#[derive(Clone, Debug)]
struct SoldState {}

impl State for SoldState {
    fn insert_quarter(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State> {
        self
    }
    fn eject_quarter(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State> {
        self
    }
    fn turn_crank(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State> {
        self
    }
    fn dispense(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State> {
        gumball_machine.release_ball();
        if gumball_machine.count > 0 {
            Box::new(NoQuarterState {})
        } else {
            Box::new(SoldOutState {})
        }
    }
}

#[derive(Clone, Debug)]
struct SoldOutState {}

impl State for SoldOutState {
    fn insert_quarter(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State> {
        self
    }
    fn eject_quarter(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State> {
        self
    }
    fn turn_crank(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State> {
        self
    }
    fn dispense(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State> {
        self
    }
}

#[derive(Clone, Debug)]
struct HasQuarterState {}

impl State for HasQuarterState {
    fn insert_quarter(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State> {
        self
    }
    fn eject_quarter(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State> {
        Box::new(NoQuarterState {})
    }
    fn turn_crank(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State> {
        Box::new(SoldState {})
    }
    fn dispense(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State> {
        self
    }
}
#[derive(Clone, Debug)]
struct NoQuarterState {}

impl State for NoQuarterState {
    fn insert_quarter(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State> {
        Box::new(HasQuarterState {})
    }
    fn eject_quarter(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State> {
        self
    }
    fn turn_crank(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State> {
        self
    }
    fn dispense(self: Box<Self>, gumball_machine: &mut GumballMachine) -> Box<dyn State> {
        self
    }
}

struct GumballMachine {
    count: i32,
}
impl GumballMachine {
    fn new(count: i32) -> Self {
        Self { count }
    }
    fn release_ball(&mut self) {
        self.count = self.count - 1;
    }
}

struct GumballApplication {
    machine: GumballMachine,
    state: Box<dyn State>,
}

impl GumballApplication {
    fn new(count: i32) -> Self {
        let state: Box<dyn State> = if count > 0 {
            Box::new(NoQuarterState {})
        } else {
            Box::new(SoldOutState {})
        };
        Self {
            machine: GumballMachine::new(count),
            state,
        }
    }
    // fn insert_quarter(&mut self) {
    //     let state = self.state;
    // // }
    // fn eject_quarter(&mut self) {}
    // fn turn_crank(&mut self) {}
}

fn execute(application: GumballApplication, button: &str) -> GumballApplication {
    let GumballApplication {
        mut machine,
        mut state,
    } = application;
    state = match button {
        "insert" => state.insert_quarter(&mut machine),
        "eject" => state.eject_quarter(&mut machine),
        "crank" => {
            let state = state.turn_crank(&mut machine);
            state.dispense(&mut machine)
        }
        _ => state,
    };
    GumballApplication { machine, state }
}

fn main() {
    // let mut gumball_machine = GumballMachine::new(2);
    let application = GumballApplication::new(2);
    let application = execute(application, "insert");
    println!("{:?} {:?}", application.state, application.machine.count);
    let application = execute(application, "crank");
    println!("{:?} {:?}", application.state, application.machine.count);

    let application = execute(application, "insert");
    println!("{:?} {:?}", application.state, application.machine.count);
    let application = execute(application, "eject");
    println!("{:?} {:?}", application.state, application.machine.count);
    let application = execute(application, "crank");
    println!("{:?} {:?}", application.state, application.machine.count);

    let application = execute(application, "insert");
    println!("{:?} {:?}", application.state, application.machine.count);
    let application = execute(application, "crank");
    println!("{:?} {:?}", application.state, application.machine.count);
}
