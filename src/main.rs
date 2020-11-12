use bintree::tree::{Node, SearchOrder};
use bintree::formula_parser::{*};

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

fn fn1(dat: &mut i32) {
    *dat = 100;
    
}

fn fn2() {
    let mut a = Some(Box::new(1));

    let b = a.as_mut();

    let c = b.map(|x| &mut **x).unwrap();

    *c = 10;
    
    println!("{:?}", c);
}

fn fn3() {
    let mut a = FormulaCalculator::new();

    //a.parse("1 + 2.3 * 4 + 5").unwrap();
    //a.parse("1 + 2 * (3 + 15 / (1 + 3)) + 1 / 2").unwrap();
    a.parse("x = 1 + 1").unwrap();

    let ans = a.calc().unwrap();
    println!("ans = {}", ans);

    a.parse("y = 8.0 + 3.0 + x").unwrap();
    let ans = a.calc().unwrap();
    println!("ans = {}", ans);

    a.parse("-1 + -2 * (3 + 2) * x + y").unwrap();
    let ans = a.calc().unwrap();
    println!("ans = {}", ans);

    let v = vec![1, 2, 3, 4];
    let b = &v[2..];

    println!("{:?}", b);
}

fn main() {
    fn3();


/*
    let mut a = 0;
    let b = &mut a;



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
  
    println!("PreOrder");
    root.foreach(&SearchOrder::PreOrder, &mut |x| println!("{}", x));
    println!("InOrder");
    root.foreach(&SearchOrder::InOrder, &mut |x| println!("{}", x));
    println!("OutOrder");
    root.foreach(&SearchOrder::PostOrder, &mut |x| println!("{}", x));

/*
    let mut one = 1;

    let mut plus_one = |x| -> i32 {
        one += 1;
        x + one
    };

    println!("{}", plus_one(10));
    println!("{}", plus_one(10));
    println!("{}", plus_one(10));
    println!("{}", plus_one(10));

    let mut ary: Vec<i32> = Vec::new();

    let mut push2ary = |x| {
        ary.push(x);
    };
    
    push2ary(1);
    push2ary(2);
    push2ary(3);
    push2ary(4);

    println!("{:?}", ary);

    closure_test(push2ary);

    println!("{:?}", ary);*/
    let mut data = 100;

    let mut closure = |x: i32| {
        data += x;
        println!("I'm a closure! {}, {}", x, data);
    };

    call_me_mut(&mut closure);
    call_me_mut(&mut closure);
    call_me(function);
    
    let mut ary: Vec<i32> = Vec::new();
    let mut pusher = |x| ary.push(x);

    call_me_mut(&mut pusher);

    println!("{:?}", ary);

    */
}


fn call_me_mut<F>(f: &mut F) 
where F: FnMut(i32) {
    f(1);
    f(1);
    f(1);
    f(1);
}


fn call_me<F>(f: F) 
where F: Fn(i32) {
    f(1);

}

// Define a wrapper function satisfying the `Fn` bound
fn function(x: i32) {
    println!("I'm a function! {}", x);
}