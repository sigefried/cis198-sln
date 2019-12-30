// TODO: everything
//
//
//
use std::mem;
#[derive(Debug)]
struct Node {
    elem: i32,
    left: Link,
    right: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
pub struct BST {
    root: Link,
}


impl BST {
    pub fn new() -> BST {
        BST { root: Link::Empty }
    }

    pub fn insert(&mut self, elem: i32) -> bool {
        self.root.insert(elem)
    }

    pub fn search(&self, elem: i32) -> bool {
        self.root.search(elem)
    }
}

impl Link {

    pub fn insert(&mut self, elem : i32) -> bool {
        match *self {
            Link::Empty => {
                let node = Node {
                    elem: elem,
                    left: Link::Empty,
                    right: Link::Empty
                };
                mem::replace(self, Link::More(Box::new(node)));
                true
            },
            Link::More(ref mut node) => {
                if elem < node.elem {
                    node.left.insert(elem)
                } else if elem > node.elem {
                    node.right.insert(elem)
                } else {
                    false
                }
            }
        }
    }


    pub fn search(&self, elem: i32) -> bool {
        match *self {
            Link::Empty => false,
            Link::More(ref node) => {
                if elem < node.elem {
                    node.left.search(elem)
                } else if elem > node.elem {
                    node.right.search(elem)
                } else {
                    true
                }
            }
        }
    }
}



// #[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn basics() {
        let mut tree = BST::new();

        assert_eq!(tree.search(1), false);
        // Populate list
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(3), true);

        // Check search
        assert_eq!(tree.search(1), true);
        assert_eq!(tree.search(2), true);
        assert_eq!(tree.search(3), true);
        // Check repeatly insertion
        assert_eq!(tree.insert(1), false);
    }
}

