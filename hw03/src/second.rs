#[derive(Debug)]
pub struct BST<T: PartialOrd + Copy> {
    root : Link<T>,
}

#[derive(Debug)]
struct Node<T: PartialOrd + Copy> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

trait InsertSearch<T> {
    fn insert(&mut self, elem: T) -> bool;
    fn search(&self, elem: T) ->bool;
}

impl<T: PartialOrd + Copy> BST<T> {
    pub fn new() -> BST<T> {
        BST { root: None }
    }

    pub fn insert(&mut self, elem : T) -> bool {
        self.root.insert(elem)
    }

    pub fn search(&self, elem: T) -> bool {
        self.root.search(elem)
    }
}

impl<T: PartialOrd + Copy> InsertSearch<T> for Link<T> {
    fn insert(&mut self, elem: T) -> bool {
        match *self {
            None => {
                let node = Node {
                    elem: elem,
                    left: None,
                    right: None,
                };
                *self = Some(Box::new(node));
                true
            },
            Some(ref mut node) => {
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

    fn search(&self, elem: T) -> bool {
        match *self {
            None => false,
            Some(ref node) => {
                if node.elem == elem {
                    true
                } else if node.elem > elem {
                    node.left.search(elem)
                } else {
                    node.right.search(elem)
                }
            },
        }
    }
}

impl<T: PartialOrd + Copy> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.root)
    }
}

pub struct IntoIter<T: PartialOrd + Copy>(Link<T>);

impl<T: PartialOrd + Copy> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.0.take() {
            let value = node.elem;
            self.0 = node.right;
            return Some(value);
        }
        None
    }
}

pub struct Iter<'a, T : PartialOrd + Copy + 'a > {
    next: Option<&'a Node<T>>,
}

impl<'a, T : PartialOrd + Copy + 'a> IntoIterator for &'a BST<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter { next: self.root.as_ref().map(|link| &**link) }
    }
}

impl<'a, T : PartialOrd + Copy + 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.right.as_ref().map(|nd| &**nd);
            &node.elem
        })
    }
}


pub struct IterMut<'a, T : PartialOrd + Copy + 'a > {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T : PartialOrd + Copy + 'a> IntoIterator for &'a mut BST<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        IterMut { next: self.root.as_mut().map(|link| &mut **link) }
    }
}

impl<'a, T : PartialOrd + Copy + 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.right.as_mut().map(|nd| &mut **nd);
            &mut node.elem
        })
    }
}




#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn basics() {
        let mut tree = BST::new();
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(1), false);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(3), true);

        assert_eq!(tree.search(3), true);
        assert_eq!(tree.search(2), true);
        assert_eq!(tree.search(1), true);
        assert_eq!(tree.search(0), false);

    }
    #[test]
    fn test_into_iter() {
        let mut tree = BST::new();
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(3), true);

        let mut iter = tree.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        let mut tree = BST::new();
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(3), true);

        {
            let mut iter = (&tree).into_iter();
            assert_eq!(iter.next(), Some(&1));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), None);
        }

        assert_eq!(tree.insert(1), false);
    }


    #[test]
    fn test_iter_mut() {
        let mut tree = BST::new();
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(3), true);
        {
            let mut iter = (&mut tree).into_iter();
            assert_eq!(iter.next(), Some(&mut 1));
            assert_eq!(iter.next(), Some(&mut 2));
            let elem = iter.next();
            assert_eq!(elem, Some(&mut 3));
            if let Some(value) = elem {
                *value = 4;
            }
        }
        assert_eq!(tree.insert(1), false);
        assert_eq!(tree.insert(3), true);
        assert_eq!(tree.insert(4), false);
    }
}
