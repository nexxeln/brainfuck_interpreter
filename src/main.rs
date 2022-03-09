use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let sub_command: String = match args.get(1) {
        Some(v) => v.to_owned(),
        None => String::from("")
    };

    match sub_command.as_str() {
        "f" => {
            let path = &args[2];
            let file = fs::read_to_string(path);
            match file {
                Ok(code) => brainfuck_interpreter::runtime::Runtime::new(&code).run(),
                Err(_) => println!("Error: Invalid path: {}", path),
            }
        }
        _ => {
            println!("Invalid sub command!\n\nAll valid sub commands:\n\t run [path]")
        },
    }
}