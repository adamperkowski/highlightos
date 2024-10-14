use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::{format, vec};

use hlkernel::println;
use hlkernel::vga_buffer::{Color, STR_COLORS, WRITER};

use crate::CMD_HISTORY;

pub struct Command {
    pub name: &'static str,
    pub args: &'static str,
    pub doc: &'static str,
    pub fun: fn(Vec<&str>) -> i32,
}

fn clrs(_args: Vec<&str>) -> i32 {
    WRITER.lock().clear_screen();
    1
}

fn help(_args: Vec<&str>) -> i32 {
    println!(
        "HighlightOS Shell

  List of available commands:"
    );

    for cmd in COMMAND_LIST {
        println!(". {} {}  >>  {}", cmd.name, cmd.args, cmd.doc);
    }

    0
}

fn test(_args: Vec<&str>) -> i32 {
    println!("hello. this is a test command. it's life goal is to always return 2.");
    2
}

fn cc(_args: Vec<&str>) -> i32 {
    println!(
        "Copyright (C) 2024 Adam Perkowski

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

    0
}

fn document(_args: Vec<&str>) -> i32 {
    if !_args.is_empty() {
        let req_com = &_args[0].replace("\n", "");
        if let Some(command) = COMMAND_LIST.iter().find(|&cmd| cmd.name == req_com) {
            println!("{}  >>  {}", command.name, command.doc);
            0
        } else {
            WRITER
                .lock()
                .print_colored("Command not found.\n".to_string(), Color::LightRed, Color::Black);
            3
        }
    } else {
        WRITER
            .lock()
            .print_colored("No command specified.\n".to_string(), Color::LightRed, Color::Black);
        4
    }
}

fn chcolor(_args: Vec<&str>) -> i32 {
    if _args.len() == 2 {
        let mut new_colors: vec::Vec<Color> = vec![];

        for arg in _args {
            if let Some(color) = STR_COLORS.iter().find(|&col| col.name == arg.replace("\n", "")) {
                new_colors.push(color.color);
            } else {
                WRITER
                    .lock()
                    .print_colored(format!("Color not found: {}\n", arg), Color::LightRed, Color::Black);
                return 4;
            }
        }
        WRITER.lock().change_color(new_colors[0], new_colors[1]);
        0
    } else {
        WRITER.lock().print_colored(
            "Specify both foreground and background color.\nExample usage: chcolor red white\n".to_string(),
            Color::LightRed,
            Color::Black,
        );
        4
    }
}

pub fn cmd_hist(_args: Vec<&str>) -> i32 {
    let cmd_history = CMD_HISTORY.lock();
    for i in &cmd_history.history {
        println!("{}", i);
    }

    0
}

#[cfg(debug_assertions)]
fn crasher(_args: Vec<&str>) -> i32 {
    println!("CRASHING...\n\n");
    panic!("Invoked by THE CRASHER >:)");
}

pub const COMMAND_LIST: &[Command] = &[
    Command {
        name: "clrs",
        args: "",
        doc: "clear the output",
        fun: clrs,
    },
    Command {
        name: "help",
        args: "",
        doc: "show a list of available commands",
        fun: help,
    },
    Command {
        name: "test",
        args: "",
        doc: "test :)",
        fun: test,
    },
    Command {
        name: "cc",
        args: "",
        doc: "display copyright info",
        fun: cc,
    },
    Command {
        name: "getdoc",
        args: "[cmd]",
        doc: "display the documentation of selected command",
        fun: document,
    },
    // Command {
    //     name: "reinit",
    //     args: "",
    //     doc: "re-initialize the kernel",
    //     fun: crate::init_kernel,
    // },
    Command {
        name: "chcolor",
        args: "[fg] [bg]",
        doc: "change text color",
        fun: chcolor,
    },
    Command {
        name: "history",
        args: "",
        doc: "display command history",
        fun: cmd_hist,
    },
    #[cfg(debug_assertions)]
    Command {
        name: "crash_kernel",
        args: "",
        doc: "DEV | cause a kernel panic",
        fun: crasher,
    },
];
