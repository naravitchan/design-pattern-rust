struct Coffee;
impl CaffeineBeverage for Coffee {
    fn brew(&self) {
        println!("Dripping Coffee through filter");
    }
    fn add_condiments(&self) {
        println!("Adding Sugar and Milk");
    }
    fn customer_wants_condiments(&self) -> bool {
        false
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
        if self.customer_wants_condiments()
        // hook
        {
            self.add_condiments();
        }
    }
    fn boil_water(&self) {
        println!("Boiling water");
    }
    fn pour_in_cup(&self) {
        println!("Pouring into cup");
    }
    fn brew(&self);
    fn add_condiments(&self);
    fn customer_wants_condiments(&self) -> bool {
        true
    }
}
fn main() {
    let tea = Tea {};
    tea.prepare_recipe();
    let coffee = Coffee {};
    coffee.prepare_recipe();
}
