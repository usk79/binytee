use bintree::tree::{Node, SearchOrder};

#[derive(Debug)]
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

fn main() {

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
 /*   
    println!("PreOrder");
    root.foreach(&SearchOrder::PreOrder, &|x| println!("{}", x));
    println!("InOrder");
    root.foreach(&SearchOrder::InOrder, &|x| println!("{}", x));
    println!("OutOrder");
    root.foreach(&SearchOrder::PostOrder, &|x| println!("{}", x));
*/
    let a = 1;
    let &b = &a;
    let mut c = b;
    c += 1;

    println!("{}", b);
    println!("{}", c);
}