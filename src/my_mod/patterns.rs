use std::fmt;
#[derive(Debug)]
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "({})", self.0.join(","))
    }
}

pub fn action_newtype() {
    // new type 模式 用来对治（orphan rule)孤儿规则

    let w = Wrapper(vec![String::from("hello"), String::from("world!")]);
    println!("w= {}", w);
}
