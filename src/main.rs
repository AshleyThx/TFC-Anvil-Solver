use colored::Colorize;
use std::io::{self};

mod action;
use action::{Action, COMMON_ACTIONS};

mod solver;
use solver::simple_solve;

fn read_u8() -> u8 {
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    num.trim().parse::<u8>().expect("Number expected")
}

fn read_rule() -> usize {
    let mut num = read_u8() as usize;
    num = if num == 0 {
        COMMON_ACTIONS.len() - 1
    } else {
        num - 1
    };

    if (0..COMMON_ACTIONS.len()).contains(&usize::from(num)) {
        return num;
    } else {
        panic!("Invalid rule number");
    }
}

fn print_all_actions() {
    println!("{}", "Every action for rules: [".bright_yellow());
    for idx in 0..COMMON_ACTIONS.len() {
        let act_num = if idx == 0 {
            COMMON_ACTIONS.len() - 1
        } else {
            idx - 1
        };
        let action = &COMMON_ACTIONS[act_num];
        println!("{}", format!("\t{}. {}", idx, action.name).bright_yellow());
    }
}

fn print_solution(solution: Vec<&Action>, rules: &[&Action; 3]) {
    println!("{}", "This is the solution: [".bright_green());
    let mut act_count = 0;
    for action in solution {
        act_count += 1;
        println!(
            "{}",
            format!("\t[goal] {}. {}", act_count, action.name).bright_green()
        );
    }
    println!("{}", "\t-----".bright_green());
    for action in rules {
        act_count += 1;
        println!(
            "{}",
            format!("\t[rule] {}. {}", act_count, action.name).bright_green()
        );
    }
    println!("{}", "]".bright_green());
}

fn main() {
    let mut rules: [&Action; 3] = [&COMMON_ACTIONS[8], &COMMON_ACTIONS[8], &COMMON_ACTIONS[8]];

    print_all_actions();

    println!("{}", "]\nGoal, Rule1, Rule2, Rule3".bright_yellow());
    let goal_points: i32 = read_u8() as i32;
    for n in 0..3 {
        let action = &COMMON_ACTIONS[read_rule()];
        rules[n] = action
    }

    print_solution(simple_solve(goal_points, &rules), &rules);
}
