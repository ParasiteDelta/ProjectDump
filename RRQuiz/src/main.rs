mod res;
mod types_funcs;

use crate::{types_funcs::*, res::*};
use std::process::exit;

fn main() {
    loop {
        println!("{SPACER01}{GREETING01}");
        let usr_sel = get_inp(UserInputType::Number, MENU).uw_c();

        match usr_sel {
            1 => {
                match usr_opt_worksheet_sel() {
                    Ok(_) => println!("{SUCCESS}"),
                    Err(e) => println!("{ERR_FAIL}{e}\n"),
                };
            },
            2 => exit(0),
            _ => println!("{ERR_UNS}"),
        };
    }
}