// Copyright (C) 2024  Adam Perkowski

use std::io;
use std::io::Write;

fn main() {
    clrs();

    let cmds = vec!["clrs\n", "help\n", "test\n", "cc\n"];

    println!("HighlightOS Shell\n");

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

    This program is free software; you can redistribute it and/or
    modify it under the terms of the GNU General Public License
    as published by the Free Software Foundation; either version 2
    of the License, or (at your option) any later version.
    
    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
    
    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.");

    return 0;
}