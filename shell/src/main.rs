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
        
        if inpt.len() == 1 { rtr = 100; }
        else if cmds.iter().any(|&s| s == inpt) { 
            if inpt == cmds[0] { rtr = 99; }
            else if inpt == cmds[1] { rtr = help(); }
            else if inpt == cmds[2] { rtr = test(); }
            else if inpt == cmds[3] { rtr = cc(); } 
            else if inpt == cmds[4] { rtr = 99; }
            else if inpt == cmds[5] { rtr = exit_hls(); }
        }
        else { rtr = 1; }
        
        if rtr == 0 {
            inpt.pop();
            println!("\n > {}\nexecuted successfully\n", inpt);
        }
        else if rtr == 1 {
            inpt.pop();
            println!("\n > {}\ncommand not found\n", inpt);
        }
        else if rtr == 2 {
            inpt.pop();
            println!("\n > {}\nreturned general error\n", inpt);
        }
        else if rtr == 3 {
            inpt.pop();
            println!("\n > {}\naborted\n", inpt);
        }
        else if rtr == 99 { clrs(); }
        else {
            inpt.pop();
            println!("\n > {}\nreturned : {}\n", inpt, rtr);
        }
    }
}

fn clrs() {
    print!("\x1B[2J\x1B[1;1H");
}

fn help() -> i32 {
    println!("HighlightOS Shell

  List of commands:
    . clrs  >>  clear screen
    . help  >>  show list of commands
    . test  >>  test :)
    . cc    >>  show copyright info");
    
    return 0;
}

fn test() -> i32 {
    println!("hello. this is a test command. its life goal is to always return 2.");
    return 2;
}

fn cc() -> i32 {
    println!("Copyright (C) 2024  Adam Perkowski

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see https://www.gnu.org/licenses .");

    return 0;
}

fn exit_hls() -> i32 {
    print!("u sure? [ y/N ] < ");

    let mut inpt2 = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut inpt2).unwrap();
    
    if inpt2.to_lowercase() == "y\n" { println!(); process::exit(1); } // return 0
    else { return 3 }
}
