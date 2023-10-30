use std::cell::RefCell;
use std::rc::Rc;

// #[derive(Debug)]
// enum State {
//     SOLD_OUT,
//     NO_QUARTER,
//     HAS_QUARTER,
//     SOLD,
// }

trait State {
    fn insert_quarter(&self, gumball_machine: &mut GumballMachine);
    fn eject_quarter(&self, gumball_machine: &mut GumballMachine);
    fn turn_crank(&self, gumball_machine: &mut GumballMachine);
    fn dispense(&self, gumball_machine: &mut GumballMachine);
}

#[derive(Clone, Debug)]
struct SoldState {}

impl State for SoldState {
    fn insert_quarter(&self, gumball_machine: &mut GumballMachine) {
        // GumballMachine.set_state
    }
    fn eject_quarter(&self, gumball_machine: &mut GumballMachine) {
        // todo
    }
    fn turn_crank(&self, gumball_machine: &mut GumballMachine) {
        // todo
    }
    fn dispense(&self, gumball_machine: &mut GumballMachine) {
        gumball_machine.release_ball();
        if gumball_machine.count > 0 {
            gumball_machine.set_state(gumball_machine.get_no_quarter_state())
        } else {
            gumball_machine.set_state(gumball_machine.get_sold_out_state())
        }
    }
}

#[derive(Clone, Debug)]
struct SoldOutState {}

impl State for SoldOutState {
    fn insert_quarter(&self, gumball_machine: &mut GumballMachine) {
        // GumballMachine.set_state
    }
    fn eject_quarter(&self, gumball_machine: &mut GumballMachine) {
        // todo
    }
    fn turn_crank(&self, gumball_machine: &mut GumballMachine) {
        // todo
    }
    fn dispense(&self, gumball_machine: &mut GumballMachine) {
        // todo
    }
}

#[derive(Clone, Debug)]
struct HasQuarterState {}

impl State for HasQuarterState {
    fn insert_quarter(&self, gumball_machine: &mut GumballMachine) {
        // GumballMachine.set_state
    }
    fn eject_quarter(&self, gumball_machine: &mut GumballMachine) {
        gumball_machine.set_state(gumball_machine.get_no_quarter_state());
    }
    fn turn_crank(&self, gumball_machine: &mut GumballMachine) {
        gumball_machine.set_state(gumball_machine.get_sold_state());
    }
    fn dispense(&self, gumball_machine: &mut GumballMachine) {
        // todo
    }
}
#[derive(Clone, Debug)]
struct NoQuarterState {}

impl State for NoQuarterState {
    fn insert_quarter(&self, gumball_machine: &mut GumballMachine) {
        gumball_machine.set_state(gumball_machine.get_has_quarter_state());
        // GumballMachine.set_state
    }
    fn eject_quarter(&self, gumball_machine: &mut GumballMachine) {
        // todo
    }
    fn turn_crank(&self, gumball_machine: &mut GumballMachine) {
        // todo
    }
    fn dispense(&self, gumball_machine: &mut GumballMachine) {
        // todo
    }
}

struct GumballMachine {
    state: Rc<RefCell<dyn State>>,
    count: i32,
    sold_state: Rc<RefCell<dyn State>>,
    sold_out_state: Rc<RefCell<dyn State>>,
    has_quarter_state: Rc<RefCell<dyn State>>,
    no_quarter_state: Rc<RefCell<dyn State>>,
}
impl GumballMachine {
    fn new(count: i32) -> Self {
        let sold_out_state: Rc<RefCell<dyn State>> = Rc::new(RefCell::new(SoldOutState {}));
        let no_quarter_state: Rc<RefCell<dyn State>> = Rc::new(RefCell::new(NoQuarterState {}));
        let has_quarter_state: Rc<RefCell<dyn State>> = Rc::new(RefCell::new(HasQuarterState {}));
        let sold_state: Rc<RefCell<dyn State>> = Rc::new(RefCell::new(SoldState {}));

        let state: Rc<RefCell<dyn State>> = if count > 0 {
            no_quarter_state.clone()
        } else {
            sold_out_state.clone()
        };

        Self {
            state,
            count,
            sold_state,
            sold_out_state,
            has_quarter_state,
            no_quarter_state,
        }
    }

    fn insert_quarter(&mut self) {
        let state = self.state.clone();
        state.try_borrow().unwrap().insert_quarter(self);
    }

    fn eject_quarter(&mut self) {
        let state = self.state.clone();
        state.try_borrow().unwrap().eject_quarter(self);
    }
    fn turn_crank(&mut self) {
        let state = self.state.clone();
        state.try_borrow().unwrap().turn_crank(self);
        self.state.clone().try_borrow().unwrap().dispense(self);
    }

    fn release_ball(&mut self) {
        self.count = self.count - 1;
    }

    fn set_state(&mut self, state: Rc<RefCell<dyn State>>) {
        self.state = state;
    }

    fn get_has_quarter_state(&self) -> Rc<RefCell<dyn State>> {
        self.has_quarter_state.clone()
    }
    fn get_no_quarter_state(&self) -> Rc<RefCell<dyn State>> {
        self.no_quarter_state.clone()
    }
    fn get_sold_state(&self) -> Rc<RefCell<dyn State>> {
        self.sold_state.clone()
    }
    fn get_sold_out_state(&self) -> Rc<RefCell<dyn State>> {
        self.sold_out_state.clone()
    }
}

fn main() {
    let mut gumball_machine = GumballMachine::new(2);
    gumball_machine.insert_quarter();
    // gumball_machine.eject_quarter();
    gumball_machine.turn_crank();
    println!("{}", gumball_machine.count);

    gumball_machine.insert_quarter();
    // gumball_machine.eject_quarter();
    gumball_machine.turn_crank();
    println!("{}", gumball_machine.count);

    gumball_machine.insert_quarter();
    // gumball_machine.insert_quarter();
    gumball_machine.turn_crank();
    println!("{}", gumball_machine.count);
}
