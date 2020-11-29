
use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct VarData(pub String, pub f64);

impl fmt::Display for VarData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.0, self.1)
    }
}

#[derive(Debug)]
pub struct VarPool {
    pool: HashMap<String, f64>,
    size: usize,
}

impl VarPool {
    pub fn new() -> Self {
        let mut newpool = HashMap::new();
        newpool.insert("ans".to_string(), 0.0);

        VarPool {
            pool: newpool,
            size: 1,
        }
    }

    pub fn insert(&mut self, dat: VarData) {
        self.pool.insert(dat.0, dat.1);
    }

    pub fn get(&self, name: &String) -> Option<f64> {
        self.pool.get(name).map(|f| *f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut p = VarPool::new();

        p.insert(VarData("x".to_string(), 1.0));

        assert_eq!( p.get(&"ans".to_string()), Some(0.0) );
        assert_eq!( p.get(&"x".to_string()), Some(1.0) );
        assert_eq!( p.get(&"y".to_string()), None );
    }

}
