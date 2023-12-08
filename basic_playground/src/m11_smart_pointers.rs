#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    use ethers::core::rand::distributions::uniform::SampleBorrow;

    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_box_smart_pointer() {
        #[derive(Debug)]
        struct Node {
            id: u32,
            next: Option<Box<Node>>,
        }

        let nodes: Box<Node> = Box::new(Node {
            id: 0,
            next: Some(Box::new(Node {
                id: 1,
                next: Some(Box::new(Node { id: 2, next: None })),
            })),
        });

        dbg!(nodes);
    }

    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_reference_counter() {
        let x: Rc<RefCell<i32>> = Rc::new(RefCell::new(50));
        let y: Rc<RefCell<i32>> = Rc::clone(&x);
        let z: Rc<RefCell<i32>> = Rc::clone(&x);

        *x.borrow_mut() = 70;

        dbg!(x.borrow());
        dbg!(1.borrow());

        #[derive(Debug, Clone)]
        struct House {
            address_number: u16,
            street: String,
            furniture: RefCell<Vec<Rc<Furniture>>>,
        }

        #[derive(Debug, Clone)]
        struct Furniture {
            id: String,
            description: String,
            house: Weak<House>,
        }
    }
}
