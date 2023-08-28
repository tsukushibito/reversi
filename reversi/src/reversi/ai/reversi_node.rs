use crate::{
    board::{BitBoard, Board},
    Action, ActionType, PlayerColor,
};

use super::game_tree_node::{GameTreeNode__, State};

pub struct ReversiState {
    pub board: BitBoard,
    pub color: PlayerColor,
    pub depth: u8,
}

impl State for ReversiState {
    fn apply_action(&self, action: &Action) -> Option<Self> {
        let board = self.board.apply_action(action)?;
        let color = self.color.opponent();
        let depth = self.depth + 1;

        Some(ReversiState {
            board,
            color,
            depth,
        })
    }

    fn leagal_actions(&self) -> Vec<Action> {
        let positions = self.board.get_movable_positions(&self.color);
        if positions.is_empty() {
            vec![Action::new(self.color, ActionType::Pass)]
        } else {
            positions
                .iter()
                .map(|p| Action::new(self.color, ActionType::Move(*p)))
                .collect::<Vec<_>>()
        }
    }
}

pub struct ReversiNode {
    pub state: ReversiState,
    pub value: Option<i32>,
    pub action: Option<Action>,
    pub children: Vec<(ReversiNode, Action)>,
}

impl GameTreeNode__ for ReversiNode {
    type S = ReversiState;

    fn new(state: Self::S) -> Self {
        ReversiNode {
            state,
            value: None,
            action: None,
            children: Default::default(),
        }
    }

    fn state(&self) -> &Self::S {
        &self.state
    }

    fn children(&self) -> &[(Self, Action)] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<(Self, Action)> {
        &mut self.children
    }

    fn value(&self) -> &Option<i32> {
        &self.value
    }

    fn action(&self) -> &Option<Action> {
        &self.action
    }

    fn is_leaf(&self) -> bool {
        self.state.board.is_game_over()
    }

    fn set_value(&mut self, value: i32) {
        self.value = Some(value);
    }

    fn set_action(&mut self, action: Action) {
        self.action = Some(action);
    }
}
