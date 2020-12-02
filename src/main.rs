use std::io::{stdin, BufRead, BufReader};

use bintree::formula::{*};
use bintree::varpool::{*};

fn main() {

    let stdin = stdin();
    let reader = stdin.lock();

    let mut varpool = VarPool::new();

    for line in reader.lines() {
        let line = line.unwrap();
        
        match line.eval(&varpool) {
            Ok(dat) => {
                println!("=> {}", dat);
                varpool.insert(dat);
            },
            Err( e ) => e.print(),
        }
    }
}

/*

fn fn3() {
    let mut pool = VarPool::new();
    let mut a = FormulaCalculator::new();

    //a.parse("1 + 2.3 * 4 + 5").unwrap();
    //a.parse("1 + 2 * (3 + 15 / (1 + 3)) + 1 / 2").unwrap();
    let formula = "x = 1 + 1";
    a.parse(formula).unwrap();
    println!("{}", formula);

    let ans = a.calc(&pool).unwrap();
    println!("{}", ans);
    pool.insert(ans);

    a.parse("y = 8.0 + 3.0 + x").unwrap();
    let ans = a.calc(&pool).unwrap();
    println!("{}", ans);
    pool.insert(ans);

    a.parse("-1 + -2 * (3 + 2) * x + y").unwrap();
    let ans = a.calc(&pool).unwrap();
    println!("{}", ans);
    pool.insert(ans);

    a.parse("-1 * -2").unwrap();
    if let Err(e) = a.calc(&pool) {
        e.print();
    }
    

    if let Err(e) = a.parse("-1 + (-2 * (3 + 2) * x + y") {
        e.print();
    }

    if let Err(e) = a.parse("-1 + -2 * (3 + 2) * x) + y") {
        e.print();
    }

    if let Err(e) = a.parse("-1 + -2..1 * (3 + 2) * x) + y") {
        e.print();
    }

    if let Err(e) = a.parse("-1 + # -2.1 * (3 + 2) * x) + y") {
        e.print();
    }
    
    if let Err(e) = a.parse("-1 + aa# -2.1 * (3 + 2) * x) + y") {
        e.print();
    }

    if let Err(e) = a.parse("*1 + -2.1 * (3 + 2) * x + y") {
        e.print();
    }

    if let Err(e) = a.parse("1 + *2.1 * (3 + 2) * x + y") {
        e.print();
    }

    if let Err(e) = a.parse("x = 1 * (1 1)") { 
        e.print();
    }

    if let Err(e) = a.parse("-a") { 
        e.print();
    }

    if let Err(e) = a.parse("1 + (+)") { 
        e.print();
    }

    a.parse("1 + 3 / (1 - 1) ").unwrap(); 
    if let Err(e) = a.calc(&pool) { 
        e.print();
    }

    a.parse("1 + 3 / (1 - 1 + a) ").unwrap(); 
    if let Err(e) = a.calc(&pool) { 
        e.print();
    }

    a.parse("1 = 1 + 3 / 2 ").unwrap(); 
    if let Err(e) = a.calc(&pool) { 
        e.print();
    }
}
*/