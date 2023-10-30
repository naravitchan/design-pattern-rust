use std::vec;
trait MenuComponent {
    fn print(&self);
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
}

#[derive(Clone, Debug)]
struct MenuItem {
    name: String,
    description: String,
    vegetarian: bool,
    price: f32,
}

impl MenuItem {
    fn new(name: String, description: String, vegetarian: bool, price: f32) -> Self {
        Self {
            name,
            description,
            vegetarian,
            price,
        }
    }
    fn get_price(&self) -> f32 {
        self.price
    }
    fn is_vegetarian(&self) -> bool {
        self.vegetarian
    }
}

impl MenuComponent for MenuItem {
    fn print(&self) {
        print!("  {}", self.get_name());
        if self.is_vegetarian() {
            print!("(v)");
        }
        println!(", {}", self.get_price());
        println!("    -- {}", self.get_description());
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_description(&self) -> String {
        self.description.clone()
    }
}

struct Menu {
    name: String,
    description: String,
    menu_components: Vec<Box<dyn MenuComponent>>,
}
impl Menu {
    fn new(name: String, description: String) -> Self {
        let menu_components = vec![];
        Self {
            name,
            description,
            menu_components,
        }
    }
    fn add(&mut self, menu_component: Box<dyn MenuComponent>) {
        self.menu_components.push(menu_component);
    }
}

impl MenuComponent for Menu {
    fn print(&self) {
        print!("\n{}", self.get_name());
        println!(", {}", self.get_description());
        println!("---------------------");
        for menu in self.menu_components.iter() {
            menu.print();
        }
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_description(&self) -> String {
        self.description.clone()
    }
}

struct PancakeHouseMenu {
    menu_items: Vec<MenuItem>,
}

impl PancakeHouseMenu {
    fn new() -> Self {
        let mut menu = Self { menu_items: vec![] };
        menu.add_item(
            "K&B's Pancake Breakfast".into(),
            "Pancakes with scrambled eggs and toast".into(),
            true,
            2.99,
        );
        menu.add_item(
            "Regular Pancake Breakfast".into(),
            "Pancakes with fried eggs, sausage".into(),
            false,
            2.99,
        );
        menu.add_item(
            "Blueberry Pancakes".into(),
            "Pancakes made with fresh blueberries".into(),
            true,
            3.49,
        );
        menu.add_item(
            "Waffles".into(),
            "Waffles with your choice of blueberries or strawberries".into(),
            true,
            3.59,
        );
        menu
    }
    fn add_item(&mut self, name: String, description: String, vegetarian: bool, price: f32) {
        let menu_item = MenuItem::new(name, description, vegetarian, price);
        self.menu_items.push(menu_item);
    }
    fn get_menu_items(&self) -> &[MenuItem] {
        &self.menu_items
    }
}

struct DinerMenu {
    number_of_item: usize,
    menu_items: [MenuItem; 6],
}
impl DinerMenu {
    fn new() -> Self {
        let menu_item = MenuItem::new("".into(), "".into(), false, 0.0);
        let mut menu = Self {
            number_of_item: 0,
            menu_items: [
                menu_item.clone(),
                menu_item.clone(),
                menu_item.clone(),
                menu_item.clone(),
                menu_item.clone(),
                menu_item.clone(),
            ],
        };
        menu.add_item(
            "Vegetarian BLT".into(),
            "(Fakin') Bacon with lettuce & tomato on whole wheat".into(),
            true,
            2.99,
        );
        menu.add_item(
            "BLT".into(),
            "Bacon with lettuce & tomato on whole wheat".into(),
            false,
            2.99,
        );
        menu.add_item(
            "Soup of the day".into(),
            "Soup of the day, with a side of potato salad".into(),
            false,
            3.29,
        );
        menu.add_item(
            "Hotdog".into(),
            "A hot dog, with sauerkraut, relish, onions, topped with cheese".into(),
            false,
            3.05,
        );
        menu
    }
    fn add_item(&mut self, name: String, description: String, vegetarian: bool, price: f32) {
        if self.number_of_item >= self.menu_items.len() {
            println!("Sorry, menu is full! Can't add item to menu");
        } else {
            let menu_item = MenuItem::new(name, description, vegetarian, price);
            self.menu_items[self.number_of_item] = menu_item;
            self.number_of_item = self.number_of_item + 1;
        }
    }
    fn get_menu_items(&self) -> &[MenuItem] {
        &self.menu_items[0..self.number_of_item]
    }
}

fn print_menu_items(menu_items: &[MenuItem]) {
    for menu in menu_items.into_iter() {
        println!(
            "{}, {} -- {}",
            menu.get_name(),
            menu.get_price(),
            menu.get_description()
        );
    }
    print!("---\n")
}

fn main() {
    let mut all_menus = Menu::new("ALL MENUS".into(), "All menus combined".into());

    let mut pancake_house_menu = Menu::new("PANCAKE HOUSE MENU".into(), "Breakfast".into());

    let mut desert_menu = Menu::new("DESSERT MENU".into(), "Dessert of course!".into());

    pancake_house_menu.add(Box::new(MenuItem::new(
        "K&B's Pancake Breakfast".into(),
        "Pancakes with scrambled eggs and toast".into(),
        true,
        2.99,
    )));
    pancake_house_menu.add(Box::new(MenuItem::new(
        "Regular Pancake Breakfast".into(),
        "Pancakes with fried eggs, sausage".into(),
        false,
        2.99,
    )));
    pancake_house_menu.add(Box::new(MenuItem::new(
        "Blueberry Pancakes".into(),
        "Pancakes made with fresh blueberries".into(),
        true,
        3.49,
    )));
    pancake_house_menu.add(Box::new(MenuItem::new(
        "Waffles".into(),
        "Waffles with your choice of blueberries or strawberries".into(),
        true,
        3.59,
    )));
    let dinner_menu = Menu::new("DINER MENU".into(), "Diner".into());
    desert_menu.add(Box::new(MenuItem::new(
        "Apple Pie".into(),
        "Apple pie with a flakey crust, topped with vanilla ice cream".into(),
        true,
        1.59,
    )));
    pancake_house_menu.add(Box::new(desert_menu));

    all_menus.add(Box::new(pancake_house_menu));
    all_menus.add(Box::new(dinner_menu));
    all_menus.print();
}
