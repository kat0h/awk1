/* 
 * file: main.rs
 * author: kota kato 2020
 * description:
 *   main source code of rusty_awk
 */

// TODO
//
// awk '{print $2 $1}'

use std::io;

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
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("usage: awk [-F fs] [-v var=value] [-f progfile | 'prog'] [file ...]");
        return;
    }

    let parsed_program = ast::patternaction::parse_paction(&args[1]);
    if parsed_program.is_err() {
        println!("Parse Err!!!");
        dbg!(&parsed_program);
        return;
    }
    let program = parsed_program.unwrap();
    dbg!(&program);

    loop {
        let mut line = String::new();
        if io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line.")
            != 0
        {
            let fields = AWKFields {
                fields: line
                    .trim()
                    .split_whitespace()
                    .map(|f| f.to_string())
                    .collect(),
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
