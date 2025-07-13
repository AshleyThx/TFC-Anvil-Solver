use crate::action::{Action, COMMON_ACTIONS};

pub fn simple_solve(goal_points: i32, rules: &[&Action; 3]) -> Vec<&'static Action> {
    let mut solution_actions: Vec<&Action> = vec![];

    let mut points_left = goal_points;
    for action in rules {
        points_left -= action.points;
    }

    for n in (5..8).rev() {
        if points_left <= 1 {
            break;
        };
        let act_count: i32 = (points_left as f32 / COMMON_ACTIONS[n].points as f32).floor() as i32;
        points_left -= act_count * COMMON_ACTIONS[n].points;
        for _ in 0..act_count {
            solution_actions.push(&COMMON_ACTIONS[n]);
        }
    }

    if points_left % 2 == 0 {
        {
            let act_count: i32 = points_left / COMMON_ACTIONS[4].points;
            for _ in 0..act_count {
                solution_actions.push(&COMMON_ACTIONS[4]);
            }
        }
    } else {
        match points_left {
            // solving last hits manually
            5 => {
                solution_actions.push(&COMMON_ACTIONS[4]);
                solution_actions.push(&COMMON_ACTIONS[4]);
                solution_actions.push(&COMMON_ACTIONS[5]);
                solution_actions.push(&COMMON_ACTIONS[1]);
            }
            3 => {
                solution_actions.push(&COMMON_ACTIONS[4]);
                solution_actions.push(&COMMON_ACTIONS[5]);
                solution_actions.push(&COMMON_ACTIONS[1]);
            }
            1 => {
                solution_actions.push(&COMMON_ACTIONS[5]);
                solution_actions.push(&COMMON_ACTIONS[1]);
            }
            _ => panic!(),
        }
    }

    solution_actions.sort_by(|a, b| a.points.cmp(&b.points));

    solution_actions
}
