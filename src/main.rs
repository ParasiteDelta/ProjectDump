use chrono::offset::Local;
use rand::{
    distributions::{Alphanumeric, DistString, Uniform},
    prelude::*,
};
use std::{
    fs::File,
    io::{prelude::*, stdin, LineWriter, Result},
    process::exit,
};

const NUM_OF_PROBLEMS: &str = "\nPlease enter the number of problems you would like to generate:\n";
const LOWEST_NUM: &str = "\nPlease enter the lowest number you want in the generation range:\n";
const HIGHEST_NUM: &str = "\nPlease enter the highest number you want in the generation range:\n";
const NEGATIVES: &str = "\nWould you like to allow negative numbers for the problem results?\nPlease enter 'y' or 'n':\n";
const WS_PROMPT: &str = "\nPlease enter the operations that you wish to perform.\nSeparate your inputs with commas if selecting multiple, e.g. '1,2,3,4':\n";

enum Either<A, B, C> {
    Left(A),
    Center(B),
    Right(C),
}

#[derive(Clone, Copy, Default)]
enum Operation {
    #[default]
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

enum UserInputType {
    Boolean,
    Number,
    Numbers,
}

#[derive(Default)]
struct OperationOptions {
    op_type: Operation,
    negatives: bool,
    lowest_num: i32,
    highest_num: i32,
    num_of_problems: i32,
}

struct Problem {
    constant1: i32,
    sign: char,
    constant2: i32,
    result: i32,
    des: char,
}

impl<L, C, R> Either<L, C, R> {
    fn unwrap_left(self) -> L
    where
        C: core::fmt::Debug,
        R: core::fmt::Debug,
    {
        match self {
            Either::Left(l) => l,
            Either::Center(c) => {
                panic!("ERR: called 'Either::unwrap_left()' on a 'Center' value: {c:?}")
            }
            Either::Right(r) => {
                panic!("ERR: called 'Either::unwrap_left()' on a 'Right' value: {r:?}")
            }
        }
    }
    fn unwrap_center(self) -> C
    where
        L: core::fmt::Debug,
        R: core::fmt::Debug,
    {
        match self {
            Either::Left(l) => {
                panic!("ERR: called 'Either::unwrap_center()' on a 'Left' value: {l:?}")
            }
            Either::Center(c) => c,
            Either::Right(r) => {
                panic!("ERR: called 'Either::unwrap_center()' on a 'Right' value: {r:?}")
            }
        }
    }
    fn unwrap_right(self) -> R
    where
        L: core::fmt::Debug,
        C: core::fmt::Debug,
    {
        match self {
            Either::Left(l) => {
                panic!("ERR: called 'Either::unwrap_right()' on a 'Left' value: {l:?}")
            }
            Either::Center(c) => {
                panic!("ERR: called 'Either::unwrap_right()' on a 'Center' value: {c:?}")
            }
            Either::Right(r) => r,
        }
    }
}

impl Operation {
    ///derive_type:
    /// Takes an input list of menu selections (typically formatted as a Vec<u32>) and returns an output list of relevant Operations.
    /// While this is technically redundant, it's more formal, manageable, and less of a grammatical faux-paus than directly comparing by input.
    fn derive_type(list: Vec<u32>) -> Vec<Operation> {
        let mut output: Vec<Operation> = Vec::new();

        for i in list {
            match i {
                1 => output.push(Operation::Addition),
                2 => output.push(Operation::Subtraction),
                3 => output.push(Operation::Multiplication),
                4 => output.push(Operation::Division),
                _ => {
                    panic!("\nERR: Unsupported Operation!\n");
                }
            }
        }

        output
    }
    fn string(&self) -> String {
        match &self {
            Operation::Addition => String::from("Addition"),
            Operation::Subtraction => String::from("Subtraction"),
            Operation::Multiplication => String::from("Multiplication"),
            Operation::Division => String::from("Division"),
        }
    }
}

impl OperationOptions {
    fn new() -> OperationOptions {
        OperationOptions::default()
    }
    fn update(target: &mut OperationOptions) {
        target.num_of_problems = get_inp(UserInputType::Number, NUM_OF_PROBLEMS).unwrap_center();
        target.lowest_num = get_inp(UserInputType::Number, LOWEST_NUM).unwrap_center();
        target.highest_num = get_inp(UserInputType::Number, HIGHEST_NUM).unwrap_center();
        target.negatives = get_inp(UserInputType::Boolean, NEGATIVES).unwrap_left();
    }
}

impl Problem {
    fn build(c1: i32, c2: i32, op: &Operation) -> Problem {
        match op {
            Operation::Addition => Problem {
                constant1: c1,
                sign: '+',
                constant2: c2,
                result: c1 + c2,
                des: 'A',
            },
            Operation::Subtraction => Problem {
                constant1: c1,
                sign: '-',
                constant2: c2,
                result: c1 - c2,
                des: 'S',
            },
            Operation::Multiplication => Problem {
                constant1: c1,
                sign: '*',
                constant2: c2,
                result: c1 * c2,
                des: 'M',
            },
            Operation::Division => Problem {
                constant1: c1,
                sign: '/',
                constant2: c2,
                result: c1 / c2,
                des: 'D',
            },
        }
    }
    fn write(p: Problem, p_num: &i32) -> (String, String) {
        let res_prob: String = format!(
            "{0}{1}: {2} {3} {4}\n\n",
            p.des, p_num, p.constant1, p.sign, p.constant2
        );
        let res_prob_ins: String = format!(
            "{0}{1}: {2} {3} {4} = {5}\n\n",
            p.des, p_num, p.constant1, p.sign, p.constant2, p.result
        );
        (res_prob, res_prob_ins)
    }
}

fn get_inp<S: AsRef<str>>(des_output: UserInputType, prompt: S) -> Either<bool, i32, Vec<u32>> {
    let mut usr_inp: String = String::new();
    println!("{}", prompt.as_ref());
    stdin()
        .read_line(&mut usr_inp)
        .expect("ERR: Failed to read line!");

    match des_output {
        UserInputType::Boolean => loop {
            match usr_inp.trim() {
                "y" | "Y" | "yes" | "1" => return Either::Left(true),
                "n" | "N" | "no" | "0" => return Either::Left(false),
                _ => {
                    println!("ERR: Invalid or unrecognized input!");
                    continue;
                }
            };
        },
        UserInputType::Number => {
            let proc = usr_inp.trim().parse().unwrap();

            Either::Center(proc)
        }
        UserInputType::Numbers => {
            let proc = usr_inp.trim();
            let res: Vec<u32> = proc.chars().filter_map(|a| a.to_digit(10)).collect();

            Either::Right(res)
        }
    }
}

//Fairly naive, definitely need to rethink this at some point.
fn usr_opt_worksheet_sel() -> Result<()> {
    let (mut res_cont, mut res_cont_ins): (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());
    let (f_out, f_out_ins) = filegen();
    let (mut f_dist, mut f_ins) = (LineWriter::new(f_out), LineWriter::new(f_out_ins));
    let mut opt_cont = OperationOptions::new();
    let p_types = Operation::derive_type(get_inp(UserInputType::Numbers, WS_PROMPT).unwrap_right());

    for e in p_types {
        opt_cont.op_type = e;
        println!("\nCurrent: {}", e.string());

        match e {
            Operation::Addition => {
                OperationOptions::update(&mut opt_cont);
            }
            Operation::Subtraction => {
                OperationOptions::update(&mut opt_cont);
            }
            Operation::Multiplication => {
                OperationOptions::update(&mut opt_cont);
            }
            Operation::Division => {
                OperationOptions::update(&mut opt_cont);
            }
        }

        let (mut res, mut res_ins) = maingen(&opt_cont);
        res_cont.append(&mut res);
        res_cont_ins.append(&mut res_ins);
    }

    for e in res_cont {
        f_dist.write(e.as_bytes())?;
    }
    for e in res_cont_ins {
        f_ins.write(e.as_bytes())?;
    }

    Ok(())
}

//File generator
fn filegen() -> (File, File) {
    //Used for generating a unique reference to this specific worksheet, useful for creating multiple worksheets.
    let keycode = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
    let tstamp = Local::now().date_naive().format("%m-%d-%Y");
    let (mut f_out, mut f_out_ins): (String, String) = (String::new(), String::new());

    (f_out, f_out_ins) = (
        format!("RRQuiz_Worksheet_{tstamp}_{keycode}.txt"),
        format!("RRQuiz_Worksheet_Ins_{tstamp}_{keycode}.txt"),
    );

    //Create new files based on designations
    let ofile = File::create(f_out).unwrap();
    let ofile_ins = File::create(f_out_ins).unwrap();

    (ofile, ofile_ins)
}

//Primary logic generator
fn maingen(main_options: &OperationOptions) -> (Vec<String>, Vec<String>) {
    //Containers init
    let (mut loopnum, mut num_cont, mut ans_cont): (i32, Vec<i32>, Vec<i32>) =
        (0, Vec::new(), Vec::new());
    let (mut res_cont, mut res_cont_ins): (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());

    while loopnum < main_options.num_of_problems {
        //Gather random number from threads, craft boundaries using specified options
        let (mut rng1, mut rng2) = (thread_rng(), thread_rng());
        let (mut last_c1, mut last_c2): (i32, i32) = (0, 0);

        //Create new problem container
        //Run newly-gathered random numbers against boundaries, place as first and second number candidates
        let main_prob: Problem = Problem::build(
            Uniform::from(main_options.lowest_num..=main_options.highest_num).sample(&mut rng1),
            Uniform::from(main_options.lowest_num..=main_options.highest_num).sample(&mut rng2),
            &main_options.op_type,
        );

        //Trying something a bit more dynamic. Still fairly naive, but hopefully it'll work.
        let perc_base = (main_options.highest_num - main_options.lowest_num) / 100;
        let perc: i32 = perc_base * 12;

        match num_cont {
            _ if (main_prob.constant1 - perc..=main_prob.constant1 + perc)
                .contains(&main_prob.constant2) =>
            {
                continue
            }
            _ if main_prob.result <= 0 && !main_options.negatives => continue,
            _ if loopnum > 1
                && (main_prob.constant1 - perc..=main_prob.constant1 + perc).contains(&last_c1) =>
            {
                continue
            }
            _ if loopnum > 1
                && (main_prob.constant2 - perc..=main_prob.constant2 + perc).contains(&last_c2) =>
            {
                continue
            }
            _ if loopnum > 1 && num_cont.contains(&main_prob.constant1) => continue,
            _ if loopnum > 1 && num_cont.contains(&main_prob.constant2) => continue,
            _ if loopnum > 1 && ans_cont.contains(&main_prob.result) => continue,
            _ => {
                loopnum += 1;
                num_cont.push(main_prob.constant1);
                num_cont.push(main_prob.constant2);
                last_c1 = main_prob.constant1;
                last_c2 = main_prob.constant2;
                ans_cont.push(main_prob.result);
                let (dist, ins) = Problem::write(main_prob, &loopnum);
                res_cont.push(dist);
                res_cont_ins.push(ins);
            }
        }
    }

    (res_cont, res_cont_ins)
}

//Intro - Menu
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
