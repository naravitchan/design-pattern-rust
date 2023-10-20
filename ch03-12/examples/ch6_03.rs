use core::cell::RefCell;
use std::borrow::BorrowMut;
use std::fmt::Debug;
use std::{cell::Ref, rc::Rc};

#[derive(Debug)]
struct Light {
    name: String,
}

impl Light {
    fn new(name: String) -> Self {
        Self { name: name }
    }
    fn on(&self) {
        println!("{} is on", self.name);
    }
    fn off(&self) {
        println!("{} is off", self.name);
    }
}

#[derive(Debug)]
struct GarageDoor {
    name: String,
}
impl GarageDoor {
    fn new(name: String) -> Self {
        Self { name: name }
    }
    fn up(&self) {
        println!("GarageDoor is open");
    }
    fn down(&self) {
        println!("GarageDoor is close");
    }
}

#[derive(Debug)]
struct Stereo {
    name: String,
    volume: i32,
}
impl Stereo {
    fn new(name: String) -> Self {
        Self {
            volume: 0,
            name: name,
        }
    }
    fn on(&self) {
        println!("Stereo is on");
    }
    fn off(&self) {
        println!("Stereo is off");
    }
    fn set_cd(&self) {
        println!("Stereo set cd");
    }
    fn set_volume(&mut self, volume: i32) {
        self.volume = volume;
        println!("Stereo set volume {}", self.volume);
    }
}

trait Command: Debug {
    fn execute(&mut self);
    fn undo(&mut self);
}

#[derive(Debug)]
struct NoCommand {}
impl Command for NoCommand {
    fn execute(&mut self) {}
    fn undo(&mut self) {}
}

#[derive(Debug)]
struct MacroCommand {
    commands: Vec<Rc<RefCell<dyn Command>>>,
}
impl Command for MacroCommand {
    fn execute(&mut self) {
        for command in self.commands.iter() {
            command.try_borrow_mut().unwrap().execute();
        }
    }
    fn undo(&mut self) {
        for command in self.commands.iter().rev() {
            command.try_borrow_mut().unwrap().undo();
        }
    }
}

#[derive(Debug)]
struct LightOnCommand {
    light: Rc<RefCell<Light>>,
}

impl LightOnCommand {
    fn new(light: Rc<RefCell<Light>>) -> Self {
        Self { light }
    }
}

impl Command for LightOnCommand {
    fn execute(&mut self) {
        self.light.try_borrow_mut().unwrap().on();
    }
    fn undo(&mut self) {
        self.light.try_borrow_mut().unwrap().off();
    }
}

#[derive(Debug)]
struct LightOffCommand {
    light: Rc<RefCell<Light>>,
}

impl LightOffCommand {
    fn new(light: Rc<RefCell<Light>>) -> Self {
        Self { light }
    }
}

impl Command for LightOffCommand {
    fn execute(&mut self) {
        self.light.try_borrow_mut().unwrap().off();
    }
    fn undo(&mut self) {
        self.light.try_borrow_mut().unwrap().on();
    }
}

#[derive(Debug)]
struct GarageDoorUpCommand {
    garage_door: GarageDoor,
}

impl GarageDoorUpCommand {
    fn new(garage_door: GarageDoor) -> Self {
        Self { garage_door }
    }
}

impl Command for GarageDoorUpCommand {
    fn execute(&mut self) {
        self.garage_door.up();
    }
    fn undo(&mut self) {
        println!("undo");
    }
}

#[derive(Debug)]
struct GarageDoorDownCommand {
    garage_door: GarageDoor,
}

impl GarageDoorDownCommand {
    fn new(garage_door: GarageDoor) -> Self {
        Self { garage_door }
    }
}

impl Command for GarageDoorDownCommand {
    fn execute(&mut self) {
        self.garage_door.down();
    }
    fn undo(&mut self) {
        println!("undo");
    }
}

#[derive(Debug)]
struct StereoOnWithCDCommand {
    stereo: Stereo,
}

impl StereoOnWithCDCommand {
    fn new(stereo: Stereo) -> Self {
        Self { stereo }
    }
}

impl Command for StereoOnWithCDCommand {
    fn execute(&mut self) {
        self.stereo.on();
        self.stereo.set_cd();
        self.stereo.set_volume(11);
    }
    fn undo(&mut self) {
        println!("undo");
    }
}

#[derive(Debug)]
struct StereoOffCommand {
    stereo: Stereo,
}

impl StereoOffCommand {
    fn new(stereo: Stereo) -> Self {
        Self { stereo }
    }
}

impl Command for StereoOffCommand {
    fn execute(&mut self) {
        self.stereo.off();
    }
    fn undo(&mut self) {
        println!("undo");
    }
}

struct RemoteControl {
    on_commands: [Rc<RefCell<dyn Command>>; 7],
    off_commands: [Rc<RefCell<dyn Command>>; 7],
    undo_command: Rc<RefCell<dyn Command>>,
}

impl RemoteControl {
    fn new() -> Self {
        let no_command: Rc<RefCell<dyn Command>> = Rc::new(RefCell::new(NoCommand {}));
        let on_commands: [Rc<RefCell<dyn Command>>; 7] = [
            no_command.clone(),
            no_command.clone(),
            no_command.clone(),
            no_command.clone(),
            no_command.clone(),
            no_command.clone(),
            no_command.clone(),
        ];
        let off_commands: [Rc<RefCell<dyn Command>>; 7] = [
            no_command.clone(),
            no_command.clone(),
            no_command.clone(),
            no_command.clone(),
            no_command.clone(),
            no_command.clone(),
            no_command.clone(),
        ];
        Self {
            on_commands: on_commands,
            off_commands: off_commands,
            undo_command: no_command.clone(),
        }
    }

    fn set_command(
        &mut self,
        slot: usize,
        on_command: Rc<RefCell<dyn Command>>,
        off_command: Rc<RefCell<dyn Command>>,
    ) {
        self.on_commands[slot] = on_command;
        self.off_commands[slot] = off_command;
    }

    fn on_button_was_pushed(&mut self, slot: usize) {
        self.on_commands[slot].try_borrow_mut().unwrap().execute();
        self.undo_command = self.on_commands[slot].clone();
    }

    fn off_button_was_pushed(&mut self, slot: usize) {
        self.off_commands[slot].try_borrow_mut().unwrap().execute();
        self.undo_command = self.off_commands[slot].clone();
    }

    fn undo_button_was_pushed(&mut self) {
        self.undo_command.try_borrow_mut().unwrap().undo();
    }

    fn print_state(&self) {
        println!("------ Remote Control -------");
        for i in 0..7 {
            println!(
                "[Slot {}] {:?} {:?}",
                i, self.on_commands[i], self.off_commands[i]
            );
        }
    }
}

fn main() {
    let mut remote = RemoteControl::new();
    let living_light = Rc::new(RefCell::new(Light::new("Living Room".into())));
    let kitchen_light = Rc::new(RefCell::new(Light::new("Kitchen Room".into())));
    // let garage_door = GarageDoor::new("Garage".into());
    // let living_stereo = Stereo::new("Living Room".into());

    let living_light_on = Rc::new(RefCell::new(LightOnCommand::new(living_light.clone())));
    let living_light_off = Rc::new(RefCell::new(LightOffCommand::new(living_light.clone())));

    let kitchen_light_on = Rc::new(RefCell::new(LightOnCommand::new(kitchen_light.clone())));
    let kitchen_light_off = Rc::new(RefCell::new(LightOffCommand::new(kitchen_light.clone())));

    let party_on: Vec<Rc<RefCell<dyn Command>>> =
        vec![living_light_on.clone(), kitchen_light_on.clone()];
    let party_off: Vec<Rc<RefCell<dyn Command>>> =
        vec![living_light_off.clone(), kitchen_light_off.clone()];
    let party_on_macro = Rc::new(RefCell::new(MacroCommand { commands: party_on }));
    let party_off_macro = Rc::new(RefCell::new(MacroCommand {
        commands: party_off,
    }));

    remote.set_command(0, living_light_on, living_light_off);
    remote.set_command(1, kitchen_light_on, kitchen_light_off);
    remote.set_command(2, party_on_macro, party_off_macro);
    remote.print_state();
    remote.on_button_was_pushed(0);
    remote.off_button_was_pushed(0);
    remote.on_button_was_pushed(1);
    remote.off_button_was_pushed(1);
    remote.on_button_was_pushed(2);
    remote.off_button_was_pushed(2);
    remote.undo_button_was_pushed();
}
