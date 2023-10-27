use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct Node {
    data: i32,
    next: Option<Rc<RefCell<Node>>>,
}
impl Node {
    fn new(data: i32) -> Self {
        Self { data, next: None }
    }

    // fn append(&mut self, data: i32) {
    //     let mut next = self.next;
    //     while next.is_some() {
    //         next = next.unwrap().next;
    //     }
    //     next = Some(Box::new(Node::new(data)))
    // }
}

#[derive(Clone)]
struct NodeList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    curr: Option<Rc<RefCell<Node>>>,
}
impl NodeList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            curr: None,
        }
    }

    fn append(&mut self, data: i32) {
        let node = Rc::new(RefCell::new(Node::new(data)));
        match self.tail.borrow_mut() {
            Some(t) => {
                t.try_borrow_mut().unwrap().next = Some(node.clone());
                self.tail = Some(node.clone());
            }
            None => {
                self.curr = Some(node.clone());
                self.head = Some(node.clone());
                self.tail = Some(node.clone());
            }
        }
    }
}
impl Iterator for NodeList {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.is_some() {
            let node = self
                .curr
                .as_mut()
                .unwrap()
                .try_borrow_mut()
                .unwrap()
                .clone();

            let value = node.data;
            self.curr = node.next.clone();
            return Some(value);
        };

        None
    }
}

fn main() {
    let mut node_list = NodeList::new();
    node_list.append(1);
    node_list.append(2);
    node_list.append(3);

    // let mut node1 = node_list.head.as_mut().unwrap().try_borrow_mut().unwrap();
    // println!("{}", node1.data);

    // let mut node2 = node1.next.as_mut().unwrap().try_borrow_mut().unwrap();
    // println!("{}", node2.data);

    // let node3 = node2.next.as_mut().unwrap().try_borrow_mut().unwrap();
    // println!("{}", node3.data);

    println!("{:?}", node_list.next());
    println!("{:?}", node_list.next());
    println!("{:?}", node_list.next());
}
