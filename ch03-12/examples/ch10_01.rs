#[derive(Debug)]
enum State {
    SOLD_OUT,
    NO_QUARTER,
    HAS_QUARTER,
    SOLD,
}

struct GumballMachine {
    state: State,
    count: i32,
}

impl GumballMachine {
    fn new(count: i32) -> Self {
        let state = if count > 0 {
            State::NO_QUARTER
        } else {
            State::SOLD_OUT
        };

        Self { state, count }
    }

    fn insert_quarter(&mut self) {
        print!("insert_quarter {:?} ", self.state);
        match self.state {
            State::NO_QUARTER => {
                self.state = State::HAS_QUARTER;
            }
            _ => {}
        }
        println!("{:?}", self.state);
    }

    fn eject_quarter(&mut self) {
        print!("eject_quarter {:?} ", self.state);
        match self.state {
            State::HAS_QUARTER => {
                self.state = State::NO_QUARTER;
            }
            _ => {}
        }
        println!("{:?}", self.state);
    }

    fn turn_crank(&mut self) {
        print!("turnCrank {:?} ", self.state);
        match self.state {
            State::HAS_QUARTER => {
                self.state = State::SOLD;
                println!("{:?}", self.state);
                self.dispense();
            }
            _ => {
                println!("{:?}", self.state);
            }
        }
    }

    fn dispense(&mut self) {
        print!("dispense {:?} ", self.state);
        match self.state {
            State::SOLD => {
                self.count = self.count - 1;
                if self.count == 0 {
                    self.state = State::SOLD_OUT;
                } else {
                    self.state = State::NO_QUARTER;
                }
            }
            _ => {}
        }
        println!("{:?}", self.state);
    }
}

fn main() {
    let mut gumball_machine = GumballMachine::new(2);
    gumball_machine.insert_quarter();
    gumball_machine.turn_crank();
    println!("{}", gumball_machine.count);

    gumball_machine.insert_quarter();
    gumball_machine.eject_quarter();
    gumball_machine.turn_crank();
    println!("{}", gumball_machine.count);

    gumball_machine.insert_quarter();
    gumball_machine.insert_quarter();
    gumball_machine.turn_crank();
    println!("{}", gumball_machine.count);
}
