// Copyleft 🄯 2024  Adam Perkowski

use std::io;
use std::io::Write;
use std::process;

fn main() {
    clrs();

    let cmds = vec!["clrs\n", "help\n", "test\n", "cc\n", "clear\n", "exit\n"];

    println!("HighlightOS Shell\nversion {}\n", env!("CARGO_PKG_VERSION"));

    loop {
        let mut inpt = String::new();
        let mut rtr = 0;

        print!("hls < ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut inpt).unwrap();

        if inpt.len() == 1 {
            rtr = 100;
        } else if cmds.iter().any(|&s| s == inpt) {
            if inpt == cmds[0] {
                rtr = 99;
            } else if inpt == cmds[1] {
                rtr = help();
            } else if inpt == cmds[2] {
                rtr = test();
            } else if inpt == cmds[3] {
                rtr = cc();
            } else if inpt == cmds[4] {
                rtr = 99;
            } else if inpt == cmds[5] {
                rtr = exit_hls();
            }
        } else {
            rtr = 1;
        }

        if rtr == 0 {
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
            clrs();
        } else {
            inpt.pop();
            println!("\n > {}\nreturned : {}\n", inpt, rtr);
        }
    }
}
