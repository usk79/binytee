// Tree構造体を作る→rootの役目
// node のadd_left, add_rightの引数はT型 add_left_treeはtreeを引数に取るようにする

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

    pub fn add_left(&mut self, tree: Node<T>) -> Result<&mut Node<T>, String> {
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

    pub fn add_right(&mut self, tree: Node<T>) -> Result<&mut Node<T>, String> {
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

    pub fn foreach(&self, order: &SearchOrder, func: &Fn(&T)) {
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
        
        root.add_left(left).unwrap();
        root.add_right(right).unwrap();
        
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

        root.add_left(left1).unwrap().add_left(left2).unwrap();
        root.add_right(right1).unwrap().add_right(right2).unwrap();

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

        root.add_left(left1).unwrap().add_left(left2).unwrap();
        root.add_right(right1).unwrap().add_right(right2).unwrap();

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

        root.add_left(left1).unwrap().add_left(left2).unwrap();
        root.add_right(right1).unwrap().add_right(right2).unwrap();

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

        root.add_left(left1_1).unwrap().add_left(left1_2).unwrap();
        left2_1.add_left(left2_2).unwrap();

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

        root.add_right(right1_1).unwrap().add_right(right1_2).unwrap();
        right2_1.add_right(right2_2).unwrap();

        root.replace_right(right2_1);

        assert_eq!(root.right().unwrap().as_ref(), &Ijk::new(3));
    }

    #[test]
    fn search_test1() {
        let mut root = Node::new(1);
        let mut left = Node::new(2);
        let mut right = Node::new(3);

        left.add_left(Node::new(4)).unwrap();
        left.add_right(Node::new(5)).unwrap();

        right.add_left(Node::new(6)).unwrap();
        right.add_right(Node::new(7)).unwrap();

        root.add_left(left).unwrap();
        root.add_right(right).unwrap();

        let preorder_vec = vec![1, 2, 4, 5, 3, 6, 7];
        let inorder_vec = vec![4, 2, 5, 1, 6, 3, 7];
        let postorder_vec = vec![4, 5, 2, 6, 7, 3, 1];
        let mut result: Vec<&i32> = Vec::new();

        root.foreach(&SearchOrder::InOrder, &|x| result.push(x));

    }
}