use std::vec;

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
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_description(&self) -> String {
        self.description.clone()
    }
    fn get_price(&self) -> f32 {
        self.price
    }
    fn is_vegetarian(&self) -> bool {
        self.vegetarian
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
    let pancake_menu = PancakeHouseMenu::new();
    let pancake_items = pancake_menu.get_menu_items();

    let diner_menu = DinerMenu::new();
    let dinner_items = diner_menu.get_menu_items();

    println!("MENU\n----\nBREAKFAST");
    print_menu_items(pancake_items);
    println!("\nLUNCH");
    print_menu_items(dinner_items);
}
