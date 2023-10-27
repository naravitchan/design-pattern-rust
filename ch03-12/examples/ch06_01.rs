struct Light {}

impl Light {
    fn on(&self) {
        println!("Light is on");
    }
    fn off(&self) {
        println!("Light is off");
    }
}

struct GarageDoor {}
impl GarageDoor {
    fn up(&self) {
        println!("GarageDoor is open");
    }
    fn down(&self) {
        println!("GarageDoor is close");
    }
}

trait Command {
    fn execute(&mut self);
}

struct LightOnCommand {
    light: Light,
}

impl LightOnCommand {
    fn new(light: Light) -> Self {
        Self { light }
    }
}

impl Command for LightOnCommand {
    fn execute(&mut self) {
        self.light.on();
    }
}

struct GarageDoorOpenCommand {
    garage_door: GarageDoor,
}

impl GarageDoorOpenCommand {
    fn new(garage_door: GarageDoor) -> Self {
        Self { garage_door }
    }
}

impl Command for GarageDoorOpenCommand {
    fn execute(&mut self) {
        self.garage_door.up();
    }
}

struct RemoteControl {
    slot: Option<Box<dyn Command>>,
}

impl RemoteControl {
    fn new() -> Self {
        Self { slot: None }
    }
    fn set_command(&mut self, command: Box<dyn Command>) {
        self.slot = Some(command);
    }

    fn button_was_pressed(&mut self) {
        self.slot.as_mut().unwrap().execute();
    }
}

fn main() {
    let mut remote = RemoteControl::new();
    let light = Light {};
    let light_on = LightOnCommand::new(light);

    let garage_door = GarageDoor {};
    let garage_door_open = GarageDoorOpenCommand::new(garage_door);

    remote.set_command(Box::new(light_on));
    remote.button_was_pressed();

    remote.set_command(Box::new(garage_door_open));
    remote.button_was_pressed();
}
