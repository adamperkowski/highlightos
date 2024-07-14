// Copyleft 🄯 2024  Adam Perkowski

use std::io;
use std::io::Write;
use std::process;

pub struct Command {
    pub name: &'static str,
    pub doc: &'static str,
    pub fun: fn() -> i32,
}

fn clrs() -> i32 {
    print!("\x1B[2J\x1B[1;1H");
    return 1;
}

fn help() -> i32 {
    println!(
        "HighlightOS Shell

  List of commands:"
    );

    for cmd in COMMAND_LIST {
        println!(". {}  >>  {}", cmd.name, cmd.doc);
    }

    return 0;
}

fn test() -> i32 {
    println!("hello. this is a test command. it's life goal is to always return 2.");
    return 2;
}

fn cc() -> i32 {
    println!(
        "Copyright (C) 2024  Adam Perkowski

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see https://www.gnu.org/licenses ."
    );

    return 0;
}

fn exit_hls() -> i32 {
    print!("are you sure you want to exit? [ y/N ] < ");

    let mut inpt = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut inpt).unwrap();

    if inpt.to_lowercase() == "y\n" {
        println!();
        process::exit(0);
    }
    // return 0
    else {
        return 3;
    }
}

pub const COMMAND_LIST: &[Command] = &[
    Command {
        name: "clrs",
        doc: "clear screen",
        fun: clrs,
    },
    Command {
        name: "help",
        doc: "show list of commands",
        fun: help,
    },
    Command {
        name: "test",
        doc: "test :)",
        fun: test,
    },
    Command {
        name: "cc",
        doc: "display copyright info",
        fun: cc,
    },
    Command {
        name: "exit",
        doc: "exit the shell :((",
        fun: exit_hls,
    },
];
