#[derive(Debug)]
struct BSTNode {
    value: i32,
    left: Option<Box<BSTNode>>,
    right: Option<Box<BSTNode>>,
}

impl BSTNode {
    fn insert(&mut self, val: i32) {
        if val < self.value {
            match self.left {
                None => {
                    self.left = Some(Box::new(BSTNode {
                        value: val,
                        left: None,
                        right: None,
                    }))
                }
                Some(ref mut node) => node.insert(val),
            }
        } else {
            match self.right {
                None => {
                    self.right = Some(Box::new(BSTNode {
                        value: val,
                        left: None,
                        right: None,
                    }))
                }
                Some(ref mut node) => node.insert(val),
            }
        }
    }

    fn find(&mut self, val: i32) -> bool {
        if val == self.value {
            return true;
        }
        if val > self.value {
            match self.left {
                None => false,
                Some(ref mut node) => node.find(val),
            }
        } else {
            match self.right {
                None => false,
                Some(ref mut node) => node.find(val),
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
    root.insert(14);

    println!("{:#?}", root);
}
