trait BeatModelTrait {
    fn on(&mut self);
    fn off(&mut self);
    fn set_bpm(&mut self, bpm: i32);
    fn get_bpm(&self) -> i32;
    fn register_observer(&mut self, observer: Box<dyn BPMObserver>);
    fn notify_observers(&self);
}

trait BPMObserver {
    fn update_bpm(&self, bmp: i32);
}

struct BeatModel {
    bpm: i32,
    stop: bool,
    observers: Vec<Box<dyn BPMObserver>>,
}

impl BeatModel {
    fn new() -> Self {
        Self {
            bpm: 90,
            stop: false,
            observers: vec![],
        }
    }
}

impl BeatModelTrait for BeatModel {
    fn on(&mut self) {
        self.bpm = 90;
        self.notify_observers();
        self.stop = false;
    }
    fn off(&mut self) {
        self.stop = true;
    }
    fn set_bpm(&mut self, bpm: i32) {
        self.bpm = bpm;
        self.notify_observers();
    }
    fn get_bpm(&self) -> i32 {
        self.bpm
    }
    fn register_observer(&mut self, observer: Box<dyn BPMObserver>) {
        self.observers.push(observer);
    }
    fn notify_observers(&self) {
        for observer in self.observers.iter() {
            observer.update_bpm(self.bpm);
        }
    }
}

trait ControllerTrait {
    fn start(&self, model: &mut BeatModel);
    fn stop(&self);
    fn increase_bpm(&self, model: &mut BeatModel);
    fn decrease_bpm(&self, model: &mut BeatModel);
    fn set_bpm(&mut self, bpm: i32);
}

struct BeatController {
    // model: BeatModel, //view
}

impl ControllerTrait for BeatController {
    fn start(&self, model: &mut BeatModel) {
        model.on();
    }
    fn stop(&self) {
        // self.model.off();
    }
    fn increase_bpm(&self, model: &mut BeatModel) {
        let bpm = model.get_bpm();
        model.set_bpm(bpm + 1);
    }
    fn decrease_bpm(&self, model: &mut BeatModel) {
        let bpm = model.get_bpm();
        model.set_bpm(bpm - 1);
    }
    fn set_bpm(&mut self, bpm: i32) {
        // self.model.set_bpm(bpm);
    }
}

struct PlayerApplication {
    controller: BeatController,
    model: BeatModel,
}

impl PlayerApplication {
    fn new() -> Self {
        let view: DJView = DJView {};
        let view2: DJView2 = DJView2 {};
        let mut model = BeatModel::new();
        model.register_observer(Box::new(view));
        model.register_observer(Box::new(view2));

        Self {
            controller: BeatController {},
            model,
        }
    }
    fn increase_bpm_button(&mut self) {
        self.controller.increase_bpm(&mut self.model);
    }
    fn decrease_bpm_button(&mut self) {
        self.controller.decrease_bpm(&mut self.model);
    }
    fn start(&mut self) {
        self.controller.start(&mut self.model);
    }
}

struct DJView {}
impl BPMObserver for DJView {
    fn update_bpm(&self, bpm: i32) {
        println!("DJView1 bpm: {}", bpm);
    }
}

struct DJView2 {}
impl BPMObserver for DJView2 {
    fn update_bpm(&self, bpm: i32) {
        println!("DJView2 bpm: {}", bpm);
    }
}

fn main() {
    let mut application = PlayerApplication::new();
    application.start();
    application.increase_bpm_button();
    application.start();
    application.increase_bpm_button();
    application.decrease_bpm_button();
    application.decrease_bpm_button();
}
