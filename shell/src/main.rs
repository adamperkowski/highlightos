// Copyleft 🄯 2024  Adam Perkowski

use std::io;
use std::io::Write;

mod commands;

use commands::COMMAND_LIST;

struct RtrType {
    code: &'static i32,
    info: &'static str,
}

fn main() {
    // (COMMAND_LIST[0].fun)();

    println!("HighlightOS Shell\nversion {}\n", env!("CARGO_PKG_VERSION"));

    loop {
        let mut inpt = String::new();

        print!("hls < ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut inpt).unwrap();

        inpt.pop();

        let mut args: Vec<&str> = inpt.split(' ').collect();

        if let Some(command) = COMMAND_LIST.iter().find(|&cmd| cmd.name == args[0]) {
            args.remove(0);

            let rtr = (command.fun)(args);

            if rtr != 1 {
                if let Some(return_code) = RTR_LIST.iter().find(|&rtr_t| rtr_t.code == &rtr) {
                    println!("\n > {}\n{}\n", inpt, return_code.info);
                } else {
                    println!("\n > {}\nreturned : {}\n", inpt, rtr)
                }
            }
        } else {
            println!("\n > {}\ncommand not found\n", inpt);
        }
    }
}

const RTR_LIST: &[RtrType] = &[
    RtrType {
        code: &0,
        info: "executed successfully",
    },
    RtrType {
        code: &2,
        info: "returned general error",
    },
    RtrType {
        code: &3,
        info: "returned critical error",
    },
    RtrType {
        code: &4,
        info: "returned user error",
    },
];
