
#[derive(Debug)]
pub enum SearchOrder {
    PreOrder,   // 行きがけ順　ノード→左→右
    InOrder,    // 通りがけ順　左→ノード→右
    PostOrder,  // 帰りがけ順　左→右→ノード
}

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data: data,
            left: None,
            right: None,
        }
    }

    pub fn left(&self) -> Option<&Node<T>> {
        match &self.left {
            Some(t) => Some(t.as_ref()),
            None => None
        }
    }

    pub fn right(&self) -> Option<&Node<T>> {
        match &self.right {
            Some(t) => Some(t.as_ref()),
            None => None
        }
    }

    pub fn left_mut(&mut self) -> Option<&mut Node<T>> {
        match &mut self.left {
            Some(t) => Some(t.as_mut()),
            None => None
        }
    }

    
    pub fn right_mut(&mut self) -> Option<&mut Node<T>> {
        match &mut self.right {
            Some(t) => Some(t.as_mut()),
            None => None
        }
    }
    /* こういうやり方もある
    pub fn take_left(mut self) -> (Self, Option<Box<Node<T>>>) {
        let child_left = self.left;
        self.left = None;
        (self, child_left)
    }
    */
    /*
    pub fn take_left(&mut self) ->  Option<Box<Node<T>>> {
        let mut child = None;

        std::mem::swap(&mut self.left, &mut child);

        child
    }
    */
    pub fn take_left(&mut self) -> Option<Node<T>> {
        match self.left.take() {
            Some(t) => Some(*t),
            None => None
        }
    }

    pub fn take_right(&mut self) -> Option<Node<T>> {
        match self.right.take() {
            Some(t) => Some(*t),
            None => None
        }
    }

    pub fn replace_left(&mut self, tree: Node<T>) {
        self.left.replace(Box::new(tree));
    }

    pub fn replace_right(&mut self, tree: Node<T>) {
        self.right.replace(Box::new(tree));
    }

    pub fn add_node_left(&mut self, tree: Node<T>) -> Result<&mut Node<T>, String> {
        match self.left {
            None => {
                self.left = Some(Box::new(tree));
                Ok( self.left_mut().unwrap() )
            },
            Some(_) => {
                Err("Left Node is aready Exist!".to_string())
            }
        }
    }

    pub fn add_node_right(&mut self, tree: Node<T>) -> Result<&mut Node<T>, String> {
        match self.right {
            None => {
                self.right = Some(Box::new(tree));
                Ok( self.right_mut().unwrap() )
            },
            Some(_) => {
                Err("Right Node is aready Exist!".to_string())
            }
        }
    }

    pub fn create_left_node(&mut self, value: T) -> Result<&mut Node<T>, String> {
        match self.left {
            None => {
                let node = Node::new(value);
                self.left = Some(Box::new(node));
                Ok( self.left_mut().unwrap() )
            },
            Some(_) => {
                Err("Left Node is aready Exist!".to_string())
            }
        }
    }

    pub fn create_right_node(&mut self, value: T) -> Result<&mut Node<T>, String> {
        match self.right {
            None => {
                let node = Node::new(value);
                self.right = Some(Box::new(node));
                Ok( self.right_mut().unwrap() )
            },
            Some(_) => {
                Err("Right Node is aready Exist!".to_string())
            }
        }
    }

    pub fn foreach<'r, F> (&'r self, order: &SearchOrder, func: &mut F) 
    where F: FnMut(&'r T) {
        match order {
            SearchOrder::PreOrder => {
                func(self.as_ref());
                if self.left.is_some() {
                    self.left().unwrap().foreach(order, func);
                }
                if self.right.is_some() {
                    self.right().unwrap().foreach(order, func);
                }
            },
            SearchOrder::InOrder => {
                if self.left.is_some() {
                    self.left().unwrap().foreach(order, func);
                }
                func(self.as_ref());
                if self.right.is_some() {
                    self.right().unwrap().foreach(order, func);
                }
            },
            SearchOrder::PostOrder => {
                if self.left.is_some() {
                    self.left().unwrap().foreach(order, func);
                }
                if self.right.is_some() {
                    self.right().unwrap().foreach(order, func);
                }
                func(self.as_ref());
            }
        }
    }

    pub fn foreach_mut<'r, F> (&'r mut self, order: &SearchOrder, func: &mut F) 
    where F: FnMut(&mut T) {
        match order {
            SearchOrder::PreOrder => {
                func(self.as_mut());
                if self.left_mut().is_some() {
                    self.left_mut().unwrap().foreach_mut(order, func);
                }
                if self.right_mut().is_some() {
                    self.right_mut().unwrap().foreach_mut(order, func);
                }
            },
            SearchOrder::InOrder => {
                if self.left_mut().is_some() {
                    self.left_mut().unwrap().foreach_mut(order, func);
                }
                func(self.as_mut());
                if self.right_mut().is_some() {
                    self.right_mut().unwrap().foreach_mut(order, func);
                }
            },
            SearchOrder::PostOrder => {
                if self.left_mut().is_some() {
                    self.left_mut().unwrap().foreach_mut(order, func);
                }
                if self.right_mut().is_some() {
                    self.right_mut().unwrap().foreach_mut(order, func);
                }
                func(self.as_mut());
            }
        }
    }

    pub fn iter<'r>(&'r self, order: &SearchOrder) -> NodeIter<T> {
        let mut elems: Vec<&'r T> = Vec::new();

        self.foreach(order, &mut |x| elems.push(x));
        let length = elems.len();

        NodeIter {
            elements: elems,
            len: length,
            idx: 0,
        }
    }
/*
    pub fn iter_mut<'r>(&'r self, order: &SearchOrder) -> NodeIterMut<T> {
        let mut elems: Vec<&'r mut T> = Vec::new();

        self.foreach_mut(order, &mut |x| elems.push(x));
        let length = elems.len();

        let mut elems = elems.iter_mut();
        
        NodeIterMut {
            elements: elems,
            len: length,
            idx: 0,
        }
    }
    */
}

impl<T> AsRef<T> for Node<T> {
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T> AsMut<T> for Node<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

pub struct NodeIter<'r, T> {
    elements: Vec<&'r T>,
    len: usize,
    idx: usize,
}

impl<'r, T> Iterator for NodeIter<'r, T> {
    type Item = &'r T;

    fn next(&mut self) -> Option<&'r T> {
        if self.idx < self.len {
            let ret = Some(self.elements[self.idx]);
            self.idx += 1;
            ret
        }
        else {
            None
        }
    }
}

pub struct NodeIterMut<'r, T> {
    elements: std::slice::IterMut<'r, &'r mut T>,
    len: usize,
    idx: usize,
}
/*
impl<'r, T> Iterator for NodeIterMut<'r, T> {
    type Item = &'r mut T;

    fn next(&mut self) -> Option<&'r mut T> {
        if self.idx < self.len {
            let a = self.elements[self.idx];
            let ret = Some(a);
            self.idx += 1;
            ret
        }
        else {
            None
        }
    }
}
*/
#[cfg(test)]
mod tests {

    #[derive(Debug, Eq, PartialEq)]
    struct Ijk {
        i: i32,
        j: i32,
        k: i32,
    }

    impl Ijk {
        fn new(idx: i32) -> Self {
            Self {
                i: 1 * idx,
                j: 2 * idx,
                k: 3 * idx,
            }

        }
    }

    use super::*;

    #[test]
    fn root_only() {
        let root = Node::new(Ijk::new(0));

        assert_eq!(root.as_ref(), &Ijk::new(0));
        assert!(root.left.is_none());
        assert!(root.right.is_none());
    }
    #[test]
    fn add() {
        let mut root = Node::new(Ijk::new(0));
        let left = Node::new(Ijk::new(1));
        let right = Node::new(Ijk::new(2));
        
        root.add_node_left(left).unwrap();
        root.add_node_right(right).unwrap();
        
        let a = root.left().unwrap();

        assert_eq!(a.as_ref(), &Ijk::new(1));

        let b = a.left();
        assert!(b.is_none());

        assert_eq!(root.left().unwrap().as_ref(), &Ijk::new(1));  /* このやり方だとborrowになるので、left()を使う！ */
        assert_eq!(root.right().unwrap().as_ref(), &Ijk::new(2));
    }

    #[test]
    fn add_twice() {
        let mut root = Node::new(Ijk::new(0));
        let left1 = Node::new(Ijk::new(1));
        let left2 = Node::new(Ijk::new(2));
        let right1 = Node::new(Ijk::new(3));
        let right2 = Node::new(Ijk::new(4));

        root.add_node_left(left1).unwrap().add_node_left(left2).unwrap();
        root.add_node_right(right1).unwrap().add_node_right(right2).unwrap();

        assert_eq!(root.left().unwrap().as_ref(), &Ijk::new(1));
        assert_eq!(root.left().unwrap().left().unwrap().as_ref(), &Ijk::new(2));
        assert!(root.left().unwrap().left().unwrap().left().is_none());

        assert_eq!(root.right().unwrap().as_ref(), &Ijk::new(3));
        assert_eq!(root.right().unwrap().right().unwrap().as_ref(), &Ijk::new(4));
        assert!(root.left().unwrap().left().unwrap().right().is_none());
    }

    #[test]
    fn take_left() {
        let mut root = Node::new(Ijk::new(0));
        let left1 = Node::new(Ijk::new(1));
        let left2 = Node::new(Ijk::new(2));
        let right1 = Node::new(Ijk::new(3));
        let right2 = Node::new(Ijk::new(4));

        root.add_node_left(left1).unwrap().add_node_left(left2).unwrap();
        root.add_node_right(right1).unwrap().add_node_right(right2).unwrap();

        let leftnew = root.take_left();
        
        assert!(root.left().is_none());
        //assert_eq!(leftnew.unwrap().as_ref(), &Ijk::new(1));
        assert_eq!(leftnew.unwrap().left().unwrap().as_ref(), &Ijk::new(2));

    }

    #[test]
    fn take_right() {
        let mut root = Node::new(Ijk::new(0));
        let left1 = Node::new(Ijk::new(1));
        let left2 = Node::new(Ijk::new(2));
        let right1 = Node::new(Ijk::new(3));
        let right2 = Node::new(Ijk::new(4));

        root.add_node_left(left1).unwrap().add_node_left(left2).unwrap();
        root.add_node_right(right1).unwrap().add_node_right(right2).unwrap();

        let rightnew = root.take_right().unwrap();
        
        assert!(root.right().is_none());
        //assert_eq!(leftnew.unwrap().as_ref(), &Ijk::new(1));
        assert_eq!(rightnew.as_ref(), &Ijk::new(3));
        assert_eq!(rightnew.right().unwrap().as_ref(), &Ijk::new(4));

    }

    #[test]
    fn relpace_left() {
        let mut root = Node::new(Ijk::new(0));
        let left1_1 = Node::new(Ijk::new(1));
        let left1_2 = Node::new(Ijk::new(2));
        let mut left2_1 = Node::new(Ijk::new(3));
        let left2_2 = Node::new(Ijk::new(4));

        root.add_node_left(left1_1).unwrap().add_node_left(left1_2).unwrap();
        left2_1.add_node_left(left2_2).unwrap();

        root.replace_left(left2_1);

        assert_eq!(root.left().unwrap().as_ref(), &Ijk::new(3));
    }

    #[test]
    fn relpace_right() {
        let mut root = Node::new(Ijk::new(0));
        let right1_1 = Node::new(Ijk::new(1));
        let right1_2 = Node::new(Ijk::new(2));
        let mut right2_1 = Node::new(Ijk::new(3));
        let right2_2 = Node::new(Ijk::new(4));

        root.add_node_right(right1_1).unwrap().add_node_right(right1_2).unwrap();
        right2_1.add_node_right(right2_2).unwrap();

        root.replace_right(right2_1);

        assert_eq!(root.right().unwrap().as_ref(), &Ijk::new(3));
    }

    #[test]
    fn create_node() {
        let mut root = Node::new(1);
        root.create_left_node(2).unwrap();
        root.create_right_node(3).unwrap();

        assert_eq!(root.left().unwrap().as_ref(), &2);
        assert_eq!(root.right().unwrap().as_ref(), &3);
    }

    #[test]
    fn search_test1() {
        let mut root = Node::new(1);
        let mut left = Node::new(2);
        let mut right = Node::new(3);

        left.add_node_left(Node::new(4)).unwrap();
        left.add_node_right(Node::new(5)).unwrap();

        right.add_node_left(Node::new(6)).unwrap();
        right.add_node_right(Node::new(7)).unwrap();

        root.add_node_left(left).unwrap();
        root.add_node_right(right).unwrap();

        let preorder_vec = vec![1, 2, 4, 5, 3, 6, 7];
        let inorder_vec = vec![4, 2, 5, 1, 6, 3, 7];
        let postorder_vec = vec![4, 5, 2, 6, 7, 3, 1];

        let mut result: Vec<i32> = Vec::new();
        let mut pusher = |x: &i32| result.push(*x);

        root.foreach(&SearchOrder::PreOrder, &mut pusher);

        assert_eq!(result, preorder_vec);
  
        let mut result: Vec<i32> = Vec::new();
        let mut pusher = |x: &i32| result.push(*x);

        root.foreach(&SearchOrder::InOrder, &mut pusher);

        assert_eq!(result, inorder_vec);

        let mut result: Vec<i32> = Vec::new();
        let mut pusher = |x: &i32| result.push(*x);

        root.foreach(&SearchOrder::PostOrder, &mut pusher);

        assert_eq!(result, postorder_vec);

        *root.as_mut() = 0;

        println!("{:?}", result);
        println!("{:?}", root);

    }

    fn create_test_tree() -> Node<i32> {
        let mut root = Node::new(1);
        let mut left = Node::new(2);
        let mut right = Node::new(3);

        left.add_node_left(Node::new(4)).unwrap();
        left.add_node_right(Node::new(5)).unwrap();

        right.add_node_left(Node::new(6)).unwrap();
        right.add_node_right(Node::new(7)).unwrap();

        root.add_node_left(left).unwrap();
        root.add_node_right(right).unwrap();

        root
    }

    #[test]
    fn foreach_mutable_test()
    {
        let mut root = create_test_tree();

        root.foreach_mut(&SearchOrder::PreOrder, &mut |x| *x = 0);

        let vec = vec![0, 0, 0, 0, 0, 0, 0];

        let mut result: Vec<i32> = Vec::new();
        let mut pusher = |x: &i32| result.push(*x);

        root.foreach(&SearchOrder::PreOrder, &mut pusher);

        assert_eq!(result, vec);
    }

    #[test]
    fn iterator_test() {
        let mut root = Node::new(1);
        let mut left = Node::new(2);
        let mut right = Node::new(3);

        left.add_node_left(Node::new(4)).unwrap();
        left.add_node_right(Node::new(5)).unwrap();

        right.add_node_left(Node::new(6)).unwrap();
        right.add_node_right(Node::new(7)).unwrap();

        root.add_node_left(left).unwrap();
        root.add_node_right(right).unwrap();

        let iter = root.iter(&SearchOrder::PreOrder);
        let mut sum = 0;
        iter.map(|x| sum += x);
        
        /*
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), None);*/
    }

    #[test]
    fn sandbox() {
        let mut root = Node::new(1);
        let mut left = Node::new(2);
        let mut right = Node::new(3);

        left.add_node_left(Node::new(4)).unwrap();
        left.add_node_right(Node::new(5)).unwrap();

        right.add_node_left(Node::new(6)).unwrap();
        right.add_node_right(Node::new(7)).unwrap();

        root.add_node_left(left).unwrap();
        root.add_node_right(right).unwrap();

        let iter = root.iter(&SearchOrder::PreOrder);
        let mut sum = 0;
        iter.map(|x| sum += x);

        
    }
}