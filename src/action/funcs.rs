fn add_one(x: i32) -> i32 {
    x + 1
}
fn do_twice(f: fn(i32) -> i32, val: i32) -> i32 {
    f(val) + f(val)
}

// type fnT = fn(i32) -> i32;

pub fn action_fn_ptr() {
    // 函数指针都实现了 Fn FnMut FnOnce
    let r = do_twice(add_one, 5);

    println!("r: {}", r);
    //
    let a = wrapper_func(|x|x+1, 1) ;
    println!("a = {}", a);

    let b = wrapper_func(func,1) ;
    println!("b= {}" , b);

    let  c = return_clo() ;
    println!("1+1 = {}", c(1));
    println!("1+1 = {}", (*c)(1)); // 解引用多态
}

fn wrapper_func<T>(t: T, v:i32) -> i32
where T: Fn(i32) -> i32 {
    t(v)
}
fn func(v : i32)-> i32{
    v+ 1
}

// ## 作为返回值
// fn return_clo() -> Fn(i32) -> i32 {
//     |x| x+1
// }
fn return_clo() -> Box< dyn Fn(i32) -> i32 >{
    Box::new(|x| x+1) 
}