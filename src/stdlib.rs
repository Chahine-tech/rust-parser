use std::collections::HashMap;

pub fn stdlib() -> HashMap<String, Box<dyn Fn(i64) -> i64>> {
    let mut lib: HashMap<String, Box<dyn Fn(i64) -> i64>> = HashMap::new();
    lib.insert("abs".to_string(), Box::new(|x| if x < 0 { -x } else { x }));
    lib.insert("square".to_string(), Box::new(|x| x * x));
    lib
}
