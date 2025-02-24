use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    env::args().skip(1)
        .fold(HashMap::<String, String>::new(), |mut accumulator, command_line_argument| {
            let parameter = command_line_argument
                .split("=")
                .collect::<Vec<&str>>();

            if let [key, value] = parameter[..] {
                accumulator.insert(String::from(key), String::from(value));
                return accumulator;
            }

            panic!("Wrong argument {}. Argument must be key=value", command_line_argument.clone());
        });
}
