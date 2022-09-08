use std::io;

mod ast;

fn main() {
    mainloop();
}

#[derive(Debug, PartialEq)]
struct AWKFields {
    fields: Vec<String>,
}

fn mainloop() {
    loop {
        let mut line = String::new();
        if io::stdin().read_line(&mut line).expect("Failed to read line.") != 0 {
            let fields = AWKFields {
                fields: line.trim().split(" ").map(|f| f.to_string()).collect(),
            };
            dbg!(&fields);
        } else {
            break;
        }
    }
}
