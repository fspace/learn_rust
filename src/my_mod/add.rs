pub fn add(x: i32, y: i32) -> i32 {
    // 一个模块总是可以访问其父级作用域（通过 super::）
    //  即便是是父级作用域的私有变量、私有函数等。 DEBUG 是私有的，但我们可以在 add 模块中使用它
    if super::DEBUG {
        println!("[DEBUG]: add({}, {})", x, y);
    }
    x + y
}
