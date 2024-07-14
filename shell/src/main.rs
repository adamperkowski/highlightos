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
    (COMMAND_LIST[0].fun)();

    println!("HighlightOS Shell\nversion {}\n", env!("CARGO_PKG_VERSION"));

    loop {
        let mut inpt = String::new();

        print!("hls < ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut inpt).unwrap();

        inpt.pop();

        if let Some(command) = COMMAND_LIST.iter().find(|&cmd| cmd.name == inpt) {
            let rtr = (command.fun)();

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

        /*if rtr == 0 {
            println!("\n > {}\nexecuted successfully\n", inpt);
        } else if rtr == 1 {
            inpt.pop();
            println!("\n > {}\ncommand not found\n", inpt);
        } else if rtr == 2 {
            inpt.pop();
            println!("\n > {}\nreturned general error\n", inpt);
        } else if rtr == 3 {
            inpt.pop();
            println!("\n > {}\naborted\n", inpt);
        } else if rtr == 99 {
            (COMMAND_LIST[0].fun);
        } else {
            inpt.pop();
            println!("\n > {}\nreturned : {}\n", inpt, rtr);
        }*/
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
];
