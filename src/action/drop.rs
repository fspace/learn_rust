#[derive(Debug)]
pub struct Dog {
    name: String,
}

impl Dog {
    pub fn new(name: String) -> Dog {
        Dog { name: name }
    }
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!(" [destruction]: Dog [{}] leave!", self.name);
    }
}
