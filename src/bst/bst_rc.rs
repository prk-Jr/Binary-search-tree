use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct BSTNode<T> {
    value: T,
    left: Option<Rc<RefCell<BSTNode<T>>>>,
    right: Option<Rc<RefCell<BSTNode<T>>>>,
}

impl<T> BSTNode<T>
where
    T: PartialEq + PartialOrd,
{
    fn insert(&mut self, val: T) {
        if val < self.value {
            match self.left {
                None => {
                    self.left = Some(Rc::new(RefCell::new(BSTNode {
                        value: val,
                        left: None,
                        right: None,
                    })))
                }
                Some(ref mut node) => node.borrow_mut().insert(val),
            }
        } else {
            match self.right {
                None => {
                    self.right = Some(Rc::new(RefCell::new(BSTNode {
                        value: val,
                        left: None,
                        right: None,
                    })))
                }
                Some(ref mut node) => node.borrow_mut().insert(val),
            }
        }
    }
}

#[test]
fn main() {
    let mut root = BSTNode {
        left: None,
        right: None,
        value: 10,
    };
    root.insert(9);
    root.insert(10);
    root.insert(13);
    root.insert(11);
    root.insert(12);

    println!("{:#?}", root);
}
