use std::env;
use std::process;

// use minigrep::Config;

#[derive(Debug)]
struct Foo {
    bar: String,
}

impl Foo {
    fn dosomething<F>(&self, f: F) -> String
    where
        F: FnOnce(String) -> String,
    {
        f("Hey!".to_string())
    }
}
fn factory() -> String {
    let num = 5;

    "as".to_string()
}

fn main() {
    "asdf".to_string();
    let foo = Foo {
        bar: "what is this?".to_string(),
    };
    // closure that prints out bar
    let x = foo.dosomething(|b| {
        println!("{b:?}");
        b
    });

    let f = factory();
    // let args: Vec<String> = env::args().collect();

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing the errors {err:?}");
    //     process::exit(1);
    // });

    // if let Err(e) = minigrep::run(config) {
    //     eprintln!("Application Error: {e:?}");
    //     process::exit(1);
    // }
}
