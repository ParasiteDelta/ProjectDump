//TODO
/*
--Create more methods for dynamic usage (multiple worksheets, single sheets, multiple singles for category, etc.)
--Refine generation and output
 */

mod res;
mod types_funcs;

use crate::types_funcs::*;
use std::process::exit;


fn main() {
    let prnt_bar1: String =
        String::from("\n=====================================================\n");
    let prog_greet: String = String::from("\nWelcome To RRQuiz, the Rusty rewrite of RoboQuiz!\n");
    println!("{prnt_bar1}{prog_greet}");
    loop {
        let msel1: String = String::from("1: Create a Worksheet\n");
        let exit: String = String::from("2: Exit RRQuiz\n");
        let usr_choice = get_inp(
            UserInputType::Number,
            format!("\nPlease select an option from below:\n{msel1}{exit}\n"),
        )
        .unwrap_center();

        match usr_choice {
            1 => {
                let res = usr_opt_worksheet_sel();
                match res {
                    Ok(_) => {
                        println!("\nOK: Task Completed Successfully!\n");
                    }
                    Err(_) => {
                        println!("\nERR: Task Failed!\n");
                    }
                }
            }
            2 => break,
            _ => {
                println!("ERR: Invalid option selected! Please try again!");
            }
        }
    }
    println!("\nThank you for using RRQuiz! Goodbye!\n");
    exit(0);
}
