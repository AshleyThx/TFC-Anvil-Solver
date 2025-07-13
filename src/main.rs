mod action;
use action::{Action, COMMON_ACTIONS};

use std::io::{self, Read};

mod solver;
use solver::simple_solve;

fn read_u8() -> u8 {
    let mut figure_buf = String::new();
    io::stdin().read_line(&mut figure_buf).unwrap();
    figure_buf.trim().parse::<u8>().expect("Number expected")
}

fn read_rule() -> u8 {
    let figure = read_u8();
    if (1..COMMON_ACTIONS.len()).contains(&usize::from(figure)) {
        return figure;
    } else {
        panic!("Invalid rule number");
    }
}

fn main() {
    let mut rules: [&Action; 3] = [&COMMON_ACTIONS[8], &COMMON_ACTIONS[8], &COMMON_ACTIONS[8]];
    let mut goal_points: i32 = 0;

    println!("Every action for rules: [");
    let mut common_act_count = 0;
    for action in COMMON_ACTIONS {
        common_act_count += 1;
        println!("\t{}. {}", common_act_count, action.name);
    }
    println!("]\nGoal, Rule1, Rule2, Rule3");

    goal_points = read_u8() as i32;

    for n in 0..3 {
        rules[n] = &COMMON_ACTIONS[(read_rule() - 1) as usize];
    }

    let solution = simple_solve(goal_points, &rules);

    println!("This is the solution: [");
    let mut act_count = 0;
    for action in solution {
        act_count += 1;
        println!("\t[goal] {}. {}", act_count, action.name);
    }
    println!("\t-----");
    for action in rules {
        act_count += 1;
        println!("\t[rule] {}. {}", act_count, action.name);
    }
    println!("]");
}
