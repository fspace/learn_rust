pub fn action_unsafe_ptr() {
    let mut num = 5;
    // 声明没问题
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 解引用指针需要包裹在unsafe 块中
    unsafe {
        println!("r1 is : {}", *r1);
        println!("r2 is : {}", *r2);
    }

    let add = 0x12345usize;
    let _r = add as *const i32;
}

unsafe fn dangerous() {
    println!("do something dangerous!");
}
fn foo() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("*r1 = {}", *r1);
        println!("*r2 = {}", *r2);
    }
}
pub fn action_unsafe_func() {
    unsafe {
        dangerous();
    }

    foo();

    // rust call c function

    unsafe {
        println!("abs(-3) : {}", abs(-3));
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "hello, world";
static mut COUNTER: u32 = 0;

fn add_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

pub fn action_mutable_static_var() {
    // 静态变量有固定内存地址
    println!("{}", HELLO_WORLD);

    //可变静态变量可能时 不安全的
    add_counter(3);
    unsafe {
        println!("counters: {}", COUNTER);
    }
}

// ---
unsafe trait Foo {
    fn foo(&self);
}
struct Bar();
unsafe impl Foo for Bar {
    fn foo(&self) {
        println!("foo");
    }
}

pub fn action_unsafe_trait() {
    let a = Bar();
    a.foo();
}
