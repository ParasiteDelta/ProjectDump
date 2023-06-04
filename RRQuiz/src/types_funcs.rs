use super::res::*;

use chrono::offset::Local; //I really, REALLY wish that there was something I could use to get the local time in a human-readable format without using Chrono.
use rand::{
    distributions::{Alphanumeric, DistString, Uniform},
    prelude::*,
};
use std::{
    fs::File,
    io::{prelude::*, stdin, LineWriter, Result},
};


/*===TYPES===*/
pub enum Either<A, B, C> {
    Left(A),
    Center(B),
    Right(C),
}

#[derive(Clone, Copy, Default)]
pub enum Operation {
    #[default]
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

pub enum UserInputType {
    Boolean,
    Number,
    Numbers,
}

#[derive(Default)]
pub struct OperationOptions {
    op_type: Operation,
    negatives: bool,
    lowest_num: i32,
    highest_num: i32,
    problems: i32,
}

pub struct Problem {
    constant1: i32,
    sign: char,
    constant2: i32,
    result: i32,
    des: char,
}


/*===IMPLEMENTATIONS===*/
impl<L, C, R> Either<L, C, R> {
    pub fn uw_l(self) -> L
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
    pub fn uw_c(self) -> C
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
    pub fn uw_r(self) -> R
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
    pub fn derive_type(list: Vec<u32>) -> Vec<Operation> {
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
    pub fn string(&self) -> String {
        match &self {
            Operation::Addition => String::from("Addition"),
            Operation::Subtraction => String::from("Subtraction"),
            Operation::Multiplication => String::from("Multiplication"),
            Operation::Division => String::from("Division"),
        }
    }
}

impl OperationOptions {
    pub fn new() -> OperationOptions {
        OperationOptions::default()
    }
    pub fn update(target: &mut OperationOptions) {
        target.problems = get_inp(UserInputType::Number, NUM_OF_PROBLEMS).uw_c();
        target.lowest_num = get_inp(UserInputType::Number, LOWEST_NUM).uw_c();
        target.highest_num = get_inp(UserInputType::Number, HIGHEST_NUM).uw_c();
        target.negatives = get_inp(UserInputType::Boolean, NEGATIVES).uw_l();
    }
}

impl Problem {
    pub fn build(constant1: i32, constant2: i32, op: &Operation) -> Problem {
        let mut res = Problem {constant1, sign: '+', constant2, result: constant1 + constant2, des: 'A'};

        match op {
            Operation::Addition => return res,
            Operation::Subtraction => {
                res.sign = '-';
                res.result = constant1 - constant2;
                res.des = 'S';
            },
            Operation::Multiplication => {
                res.sign = '*';
                res.result = constant1 * constant2;
                res.des = 'M';
            },
            Operation::Division => {
                res.sign = '/';
                res.result = constant1 / constant2;
                res.des = 'D';
            },
        }
    res
    }
    pub fn write(p: Problem, p_num: &i32) -> (String, String) {
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


/*===FUNCTIONS===*/
pub fn get_inp<S: AsRef<str>>(des_output: UserInputType, prompt: S) -> Either<bool, i32, Vec<u32>> {
    let mut usr_inp: String = String::new();
    println!("{}", prompt.as_ref());
    stdin()
        .read_line(&mut usr_inp)
        .expect("{ERR_NOREAD}");

    match des_output {
        UserInputType::Boolean => loop {
            match usr_inp.trim() {
                "y" | "Y" | "yes" | "1" => return Either::Left(true),
                "n" | "N" | "no" | "0" => return Either::Left(false),
                _ => {
                    println!("{ERR_UNS}");
                    continue;
                }
            };
        },
        UserInputType::Number => { Either::Center(usr_inp.trim().parse().unwrap()) },
        UserInputType::Numbers => { Either::Right(usr_inp.trim().chars().filter_map(|a| a.to_digit(10)).collect()) },
    }
}

pub fn usr_opt_worksheet_sel() -> Result<()> {
    let (mut res_cont, mut res_cont_ins): (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());
    let (f_out, f_out_ins) = filegen();
    let (mut f_dist, mut f_ins) = (LineWriter::new(f_out), LineWriter::new(f_out_ins));
    let mut opt_cont = OperationOptions::new();
    let p_types = Operation::derive_type(get_inp(UserInputType::Numbers, WS_PROMPT).uw_r());

    for e in p_types {
        opt_cont.op_type = e;
        println!("\nCurrent: {}", opt_cont.op_type.string());

        OperationOptions::update(&mut opt_cont);

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

pub fn filegen() -> (File, File) {
    let keycode = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
    let tstamp = Local::now().date_naive().format("%m-%d-%Y");
    let (mut f_out, mut f_out_ins): (String, String) = (String::new(), String::new());

    (f_out, f_out_ins) = (
        format!("RRQuiz_Worksheet_{tstamp}_{keycode}.txt"),
        format!("RRQuiz_Worksheet_Ins_{tstamp}_{keycode}.txt"),
    );

    let ofile = File::create(f_out).unwrap();
    let ofile_ins = File::create(f_out_ins).unwrap();

    (ofile, ofile_ins)
}

pub fn maingen(main_options: &OperationOptions) -> (Vec<String>, Vec<String>) {
    let (mut loopnum, mut num_cont, mut ans_cont): (i32, Vec<i32>, Vec<i32>) =
        (0, Vec::new(), Vec::new());
    let (mut res_cont, mut res_cont_ins): (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());

    while loopnum < main_options.problems {
        let (mut rng1, mut rng2) = (thread_rng(), thread_rng());
        let (mut last_c1, mut last_c2): (i32, i32) = (0, 0);

        let main_prob: Problem = Problem::build(
            Uniform::from(main_options.lowest_num..=main_options.highest_num).sample(&mut rng1),
            Uniform::from(main_options.lowest_num..=main_options.highest_num).sample(&mut rng2),
            &main_options.op_type,
        );

        let perc_base = (main_options.highest_num - main_options.lowest_num) / 100;
        let perc: i32 = perc_base * 12;

        match num_cont {
            _ if ((main_prob.constant1 - perc..=main_prob.constant1 + perc)
                .contains(&main_prob.constant2)
                || (main_prob.result <= 0 && !main_options.negatives)) =>
            {
                continue
            }
            _ if loopnum > 1
                && ((main_prob.constant1 - perc..=main_prob.constant1 + perc)
                    .contains(&last_c1)
                    || (main_prob.constant2 - perc..=main_prob.constant2 + perc)
                        .contains(&last_c2)) =>
            {
                continue
            }
            _ if loopnum > 1
                && (num_cont.contains(&main_prob.constant1)
                    || num_cont.contains(&main_prob.constant2)
                    || ans_cont.contains(&main_prob.result)) =>
            {
                continue
            }
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