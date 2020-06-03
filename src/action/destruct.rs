#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn action_struct_match() {
    let p = Point { x: 1, y: 0 };
    match p {
        Point { x, y: 0 } => println!("x axis"),
        Point { x: 0, y } => println!("y axis"),
        Point { x, y } => println!("others"),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ChangeColor2(Color),
}

#[derive(Debug)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

pub fn action_enum_pattern() {
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("quite");
        }
        Message::Move { x, y } => {
            println!("move , x: {}, y: {}", x, y);
        }
        Message::Write(text) => println!("Write msg:{}", text),
        Message::ChangeColor(r, g, b) => println!("color, r: {}, g: {}, b:{}", r, g, b),
        _ => (),
    }
}
pub fn action_enum_pattern2() {
    let msg = Message::ChangeColor2(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor2(Color::Rgb(r, g, b)) => {
            println!("rgb color, r: {}, g: {}, b:{}", r, g, b);
        }
        Message::ChangeColor2(Color::Hsv(h, s, v)) => {
            println!("hsv color, r: {}, g: {}, b:{}", h, s, v);
        }
        Message::ChangeColor2(_) => (),
        _ => (),
    }
}

pub fn action_struct_tuple_match() {
    let ((a, b), Point { x, y }) = ((1, 2), Point { x: 3, y: 4 });
    println!("a: {} , b:{}, x:{}, y: {}", a, b, x, y);
}

// -----------------
fn foo(_: i32, y: i32) {
    println!("y= {}", y);
}

trait A {
    fn bar(x: i32, y: i32);
}

#[derive(Debug)]
struct B {}

impl A for B {
    fn bar(_: i32, y: i32) {
        println!("y= {}", y);
    }
}

pub fn action_ignore_param() {
    foo(1, 2);

    let numbers = (1, 2, 3, 4);
    match numbers {
        (one, _, three, _) => print!("one: {}, three {}", one, three), // None => expr,
    }

    let _x = 5;
    let _y = 5;

    let s = Some(String::from("hello"));
    // if let Some(_c) = s {
    // 用下划线时并没有发生所有权的移动
    if let Some(_) = s {
        println!("Found a string");
    }
    println!("s : {:?}", s);

    print!("=================\n ");
    let numbers = (1, 2, 3, 4, 5, 6, 7, 8);
    match numbers {
        // 用 .. 忽略某些位置的值  出现的位置必须是无歧义的
        (first, .., last) => {
            println!("first: {}, last: {}", first, last);
        }
    }
}

pub fn action_match_guard() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("<5"),
        Some(x) => println!("x: {}", x),
        None => (),
    }
    //
    {
        let x = 4;
        let y = false;
        match x {
            4 | 5 | 6 if y => println!("1"),
            _ => println!("2"),
        }
    }
}

#[derive(Debug)]
enum Message2 {
    Hello { id: i32 },
}
pub fn action_pattern_at() {
    let msg = Message2::Hello { id: 5 };
    match msg {
        // @运算符允许我们在创建一个存放值变量的同时，测试变了的值是否匹配模式
        Message2::Hello { id: id_val @ 3..=7 } => {
            println!("id_val: {}", id_val);
        }
        Message2::Hello { id: 10..=20 } => {
            println!("large ");
        }
        Message2::Hello { id } => {
            println!("id: {}", id);
        }
    }
}
