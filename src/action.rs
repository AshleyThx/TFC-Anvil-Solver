// All Anvil actions from TFC
// first 0-3 are red (negative) actions
// last 4-7 are green (positive) actions

pub struct Action {
    pub points: i32,
    pub name: &'static str,
}

pub const COMMON_ACTIONS: [Action; 9] = [
    Action {
        points: -3,
        name: "Light Hit",
    },
    Action {
        points: -6,
        name: "Normal Hit",
    },
    Action {
        points: -9,
        name: "Hard Hit",
    },
    Action {
        points: -15,
        name: "Draw",
    },
    Action {
        points: 2,
        name: "Punch",
    },
    Action {
        points: 7,
        name: "Bend",
    },
    Action {
        points: 13,
        name: "Upset",
    },
    Action {
        points: 16,
        name: "Shrink",
    },
    Action {
        points: 0,
        name: "None",
    },
];
