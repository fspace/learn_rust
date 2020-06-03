trait A {
    fn print(&self);
}

trait B {
    fn print(&self);
}

#[derive(Debug)]
struct MyType;

impl A for MyType {
    fn print(&self) {
        println!("A trait for MyType");
    }
}
impl B for MyType {
    fn print(&self) {
        println!("B trait for MyType");
    }
}

impl MyType {
    fn print(&self) {
        println!("MyType");
    }
}

// ##
trait Animal {
    fn baby_name() -> String;
}
#[derive(Debug)]
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> std::string::String {
        String::from("puppy")
    }
}

pub fn action_trait_impl() {
    let mt = MyType;
    mt.print(); // 自己的方法

    MyType::print(&mt);
    A::print(&mt);
    B::print(&mt);

    println!("baby_name {}", Dog::baby_name());
    println!("baby_name {}", <Dog as Animal>::baby_name()); // 完全限定语法
}

use std::fmt;
trait Outprint: fmt::Display {
    // 要求实现Display trait

    fn out_print(&self) {
        let output = self.to_string();
        println!("output: {}", output);
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Outprint for Point {}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn action_trait_parent() {}
