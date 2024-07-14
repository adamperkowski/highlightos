// Copyleft 🄯 2024  Adam Perkowski

use std::io;
use std::io::Write;

mod commands;

use commands::COMMAND_LIST;

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
            (command.fun)();
        }

        /*if inpt.len() == 1 {
            rtr = 100;
        } else if COMMAND_LIST.iter().any(|cmd| cmd.name == inpt) {
            (COMMAND_LIST[0].fun);
        } else {
            rtr = 1;
        }*/

        /*if rtr == 0 {
            inpt.pop();
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
