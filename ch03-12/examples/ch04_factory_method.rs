trait Pizza {
    fn get_name(&self) -> String;
    fn prepare(&self) {
        println!("prepare {}", self.get_name());
    }
    fn bake(&self) {
        println!("bake {}", self.get_name());
    }
    fn cut(&self) {
        println!("cut {}", self.get_name());
    }
    fn boxx(&self) {
        println!("boxx {}", self.get_name());
    }
}

struct NYStyleCheesePizza {}
impl Pizza for NYStyleCheesePizza {
    fn get_name(&self) -> String {
        "NYStyleCheesePizza".to_string()
    }
}

struct NYStylePepperoniPizza {}
impl Pizza for NYStylePepperoniPizza {
    fn get_name(&self) -> String {
        "NYStylePepperoniPizza".to_string()
    }
}

struct ChicagoStyleCheesePizza {}
impl Pizza for ChicagoStyleCheesePizza {
    fn get_name(&self) -> String {
        "ChicagoStyleCheesePizza".to_string()
    }
}

struct ChicagoStylePepperoniPizza {}
impl Pizza for ChicagoStylePepperoniPizza {
    fn get_name(&self) -> String {
        "ChicagoStylePepperoniPizza".to_string()
    }
}

trait PizzaStore {
    fn create_pizza(&self, name: String) -> Box<dyn Pizza>;
    fn order_pizza(&self, name: String) -> Box<dyn Pizza> {
        let pizza = self.create_pizza(name);
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.boxx();
        pizza
    }
}

struct NYStylePizzaStore {}
impl PizzaStore for NYStylePizzaStore {
    fn create_pizza(&self, name: String) -> Box<dyn Pizza> {
        match name.as_str() {
            "cheese" => Box::new(NYStyleCheesePizza {}),
            _ => Box::new(NYStylePepperoniPizza {}),
        }
    }
}

struct ChicagoStylePizzaStore {}
impl PizzaStore for ChicagoStylePizzaStore {
    fn create_pizza(&self, name: String) -> Box<dyn Pizza> {
        match name.as_str() {
            "cheese" => Box::new(ChicagoStyleCheesePizza {}),
            _ => Box::new(ChicagoStylePepperoniPizza {}),
        }
    }
}

fn main() {
    println!("\n ========");
    let ny_store = NYStylePizzaStore {};
    let pizza = ny_store.order_pizza("cheese".to_string());
    println!("{}", pizza.get_name());

    println!("\n ========");
    let ch_store = ChicagoStylePizzaStore {};
    let pizza: Box<dyn Pizza> = ch_store.order_pizza("zzzz".to_string());
    println!("{}", pizza.get_name());
}
