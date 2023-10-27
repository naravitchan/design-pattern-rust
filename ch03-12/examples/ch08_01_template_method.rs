struct Coffee;
impl CaffeineBeverage for Coffee {
    fn brew(&self) {
        println!("Dripping Coffee through filter");
    }
    fn add_condiments(&self) {
        println!("Adding Sugar and Milk");
    }
}
struct Tea;
impl CaffeineBeverage for Tea {
    fn brew(&self) {
        println!("Steeping the tea");
    }
    fn add_condiments(&self) {
        println!("Adding Lemon");
    }
}

trait CaffeineBeverage {
    fn prepare_recipe(&self) {
        self.boil_water(); // primitiveOperation
        self.brew(); // concreteOperation
        self.pour_in_cup();
        self.add_condiments();
    }
    fn boil_water(&self) {
        println!("Boiling water");
    }
    fn pour_in_cup(&self) {
        println!("Pouring into cup");
    }
    fn brew(&self);
    fn add_condiments(&self);
}
fn main() {
    let tea = Tea {};
    tea.prepare_recipe();
    let coffee = Coffee {};
    coffee.prepare_recipe();
}
