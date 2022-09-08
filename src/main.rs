use std::io::{self};

mod ast;

fn main() {
    mainloop();
}

#[derive(Debug, PartialEq)]
struct AWKFields {
    fields: Vec<String>,
}

impl AWKFields {
    fn nf(&self) -> usize {
        self.fields.len()
    }
    fn get_field(&self, n: usize) -> Result<String, ()> {
        if n == 0 {
            Ok(self.fields.join(" "))
        } else if (1 <= n) && (n <= self.nf()) {
            Ok(self.fields[n - 1].clone())
        } else {
            Err(())
        }
    }
}

fn mainloop() {
    loop {
        let mut line = String::new();
        if io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line.")
            != 0
        {
            let fields = AWKFields {
                fields: line.trim().split(" ").map(|f| f.to_string()).collect(),
            };
            let nf = fields.nf();
            for f in 0..=nf {
                println!("${}: {}", f, fields.get_field(f).unwrap_or("".to_string()));
            }
        } else {
            break;
        }
    }
}
