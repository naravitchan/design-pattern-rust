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
            "cheese" => {
                let ingredient_factory = NYPizzaIngredientFactory {};
                let mut pizza = CheesePizza::new(Box::new(ingredient_factory));
                pizza.set_name("NY Style Cheese Pizza".to_string());

                Box::new(pizza)
            }
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

trait PizzaIngredientFactory {
    fn create_dough(&self) -> Box<dyn Dough>;
    fn create_sauce(&self) -> Box<dyn Sauce>;
    fn create_cheese(&self) -> Box<dyn Cheese>;
}

trait Dough {}
struct ThinCrustDough {}
impl Dough for ThinCrustDough {}

trait Sauce {}
struct MarinaraSauce {}
impl Sauce for MarinaraSauce {}

trait Cheese {}
struct ReggianoCheese {}
impl Cheese for ReggianoCheese {}

struct NYPizzaIngredientFactory {}
impl PizzaIngredientFactory for NYPizzaIngredientFactory {
    fn create_dough(&self) -> Box<dyn Dough> {
        Box::new(ThinCrustDough {})
    }
    fn create_sauce(&self) -> Box<dyn Sauce> {
        Box::new(MarinaraSauce {})
    }
    fn create_cheese(&self) -> Box<dyn Cheese> {
        Box::new(ReggianoCheese {})
    }
}

struct CheesePizza {
    ingredient_factory: Box<dyn PizzaIngredientFactory>,
    name: String,
    // dough: Box<dyn In>
}
impl CheesePizza {
    fn new(ingredient_factory: Box<dyn PizzaIngredientFactory>) -> Self {
        CheesePizza {
            ingredient_factory,
            name: "".to_string(),
        }
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
impl Pizza for CheesePizza {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
    fn prepare(&self) {
        println!("prepare dough {}", self.get_name());
        println!("prepare cheese {}", self.get_name());
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
