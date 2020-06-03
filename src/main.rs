#![allow(dead_code)]

use seahorse::{App, Command, Context, Flag, FlagType};
use std::env;

use std::process::Command as StcCmd;

mod my_lib;
mod my_mod;

mod action;

#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Mednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    // add code here

    fn is_weekday(&self) -> bool {
        match self {
            &Day::Saturday | &Day::Sunday => return false,
            _ => return true,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new()
        .name(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [args]")
        .action(|c| println!("Hello, {:?}", c.args))
        // add more command here!
        .command(Command::new().name("add").action(|c: &Context| {
            println!("hi  this is add sub command");
        }))
        .command(
            Command::new()
                .name("struct/destruct")
                .action(|c: &Context| {
                    action::action_struct_match();
                }),
        )
        .command(
            Command::new()
                .name("pattern/struct2")
                .usage("pattern/struct2 ")
                .action(|c: &Context| {
                    action::action_struct_tuple_match();
                }),
        )
        .command(Command::new().name("pattern/enum").action(|c: &Context| {
            action::action_enum_pattern();
        }))
        .command(Command::new().name("pattern/enum2").action(|c: &Context| {
            action::action_enum_pattern2();
        }))
        .command(Command::new()
        .name("pattern/ignore-param")
        .usage("... pattern/ignore-param")
        .action(|c: &Context| {
            action::action_ignore_param();
        }))
        .command(Command::new()
        .name("pattern/match-guard")
        .usage("... pattern/match-guard")
        .action(|c: &Context| {
            action::action_match_guard();
        }))
        .command(Command::new()
        .name("pattern/at")
        .usage("... pattern/at")
        .action(|c: &Context| {
            action::action_pattern_at();
        }))
        .command(Command::new()
        .name("unsafe:ptr")
        .usage("... unsafe:ptr")
        .action(|c: &Context| {
            action::action_unsafe_ptr();
        }))
        .command(Command::new()
        .name("unsafe:func")
        .usage("... unsafe:func")
        .action(|c: &Context| {
            action::action_unsafe_func();
        }))
        .command(Command::new()
        .name("unsafe:static")
        .usage("... unsafe:static")
        .action(|c: &Context| {
            action::action_mutable_static_var();
        }))
        .command(Command::new()
        .name("unsafe:trait")
        .usage("... unsafe:trait")
        .action(|c: &Context| {
            action::action_unsafe_trait();
        }))
        .command(Command::new()
        .name("op:overwrite")
        .usage("... unsafe:overwrite")
        .action(|c: &Context| {
            action::action_op_overwrite();
        }))
        .command(Command::new()
        .name("trait:impl")
        .usage("... trait:impl")
        .action(|c: &Context| {
            action::action_trait_impl();
        }))
        // 
        .command(Command::new()
        .name("pattern:newtype")
        .usage("... pattern:newtype")
        .action(|c: &Context| {
            my_mod::patterns::action_newtype() ;
        }))
        // 
        .command(Command::new()
        .name("misc:type-alias")
        .usage("... misc:type-alias")
        .action(|c: &Context| {
            my_mod::misc::action_type_alias();
        }))
        // ...
        // 
        .command(Command::new()
        .name("funcs:fn-ptr")
        .usage("... funcs:fn-ptr")
        .action(|c: &Context| {
            action::funcs::action_fn_ptr();
        }))
        // ...
        ;

    app.run(args);
}

fn foo() {
    let d = Day::Thursday;

    println!("Is day a weekday? {}", d.is_weekday());

    cmd_demo();

    println!("mylib::info: {}", my_lib::info());
    println!("my_mod::add: {}", my_mod::add(1, 2));

    let d1 = action::Dog::new(String::from("milk"));
    {
        let d2 = action::Dog::new(String::from("dahuang"));
        println!("< dog  name='dahong'   />");
        let d3 = action::Dog::new(String::from("dog3"));
        // 手动调用析狗函数
        drop(d3);
    }
    println!("=============================");
}

fn cmd_demo() {
    // let mut cmd = Command::new("python3");
    let mut cmd = StcCmd::new("python3");

    cmd.arg("dcode.py");

    // Execute the command
    match cmd.output() {
        Ok(o) => unsafe {
            println!("out put is : {}", String::from_utf8_unchecked(o.stdout));
        },
        Err(e) => {
            println!("There was an error! {}", e);
        }
    }
}
