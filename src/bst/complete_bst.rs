use std::{cell::RefCell, fmt::Debug, rc::Rc};

type Tree<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    pub value: T,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T> Node<T>
where
    T: PartialEq + PartialOrd + Debug,
{
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    fn add_node(&mut self, value: T) {
        if value < self.value {
            match self.left {
                Some(ref mut left) => left.borrow_mut().add_node(value),
                None => self.left = Some(Rc::new(RefCell::new(Node::new(value)))),
            }
        } else {
            match self.right {
                Some(ref mut right) => right.borrow_mut().add_node(value),
                None => self.right = Some(Rc::new(RefCell::new(Node::new(value)))),
            }
        }
    }

    fn inorder(&mut self) {
        if self.left.is_some() {
            match self.left {
                Some(ref mut node) => {
                    node.borrow_mut().inorder();
                }
                None => {}
            }
        }

        println!("{:#?}", self.value);

        if self.right.is_some() {
            match self.right {
                Some(ref mut node) => {
                    node.borrow_mut().inorder();
                }
                None => {}
            }
        }
    }
    fn preorder(&mut self) {
        println!("{:#?}", self.value);
        if self.left.is_some() {
            match self.left {
                Some(ref mut node) => {
                    node.borrow_mut().preorder();
                }
                None => {}
            }
        }

        if self.right.is_some() {
            match self.right {
                Some(ref mut node) => {
                    node.borrow_mut().preorder();
                }
                None => {}
            }
        }
    }

    fn postorder(&mut self) {
        if self.left.is_some() {
            match self.left {
                Some(ref mut node) => {
                    node.borrow_mut().postorder();
                }
                None => {}
            }
        }

        if self.right.is_some() {
            match self.right {
                Some(ref mut node) => {
                    node.borrow_mut().postorder();
                }
                None => {}
            }
        }
        println!("{:#?}", self.value);
    }

    fn find(&self, value: T) {
        if value == self.value {
            println!("Found: {:?}", value);
            return;
        } else if value < self.value {
            match self.left {
                Some(ref node) => {
                    node.borrow().find(value);
                }
                None => {
                    println!("Not Found: {:?}", value);

                    return;
                }
            }
        } else {
            match self.right {
                Some(ref node) => {
                    node.borrow().find(value);
                }
                None => {
                    println!("Not Found: {:?}", value);

                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Node;
    #[test]
    pub fn test() {
        let mut root = Node::new(100);
        root.add_node(50);
        root.add_node(200);
        root.add_node(10);
        root.add_node(60);
        root.add_node(150);
        root.add_node(300);
        println!("Inorder");
        root.inorder();
        println!("Postorder");
        root.postorder();
        println!("Preorder");

        root.preorder();

        root.find(12);
        root.find(13);
        root.find(10);
        root.find(10);
        root.find(9);
    }
}
