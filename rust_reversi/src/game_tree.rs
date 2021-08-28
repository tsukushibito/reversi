use crate::Action;
use crate::Board;
use std::collections::HashMap;

const C_PUCT: f32 = 1.0;

struct GameNode {
    board: Board,
    depth: u32,
    children: Vec<GameNode>,
    policy: f32,
    weight: f32,
    evaluation_count: u32,
}

impl GameNode {
    pub fn new(board: Board, depth: u32) -> GameNode {
        GameNode {
            board: board,
            depth: depth,
            children: Vec::new(),
            policy: 0.0,
            weight: 0.0,
            evaluation_count: 0,
        }
    }

    fn evaluate(&self) -> f32 {
        if self.board.is_game_over() {
            let (self_count, opponent_count) = if self.depth % 2 == 0 {
                (self.board.black_count(), self.board.white_count())
            } else {
                (self.board.white_count(), self.board.black_count())
            };

            let value = if self_count > opponent_count {
                1.0
            } else if self_count < opponent_count {
                -1.0
            } else {
                0.0
            };

            return value;
        }
        0.0f32
    }

    fn next_child_node(&self) -> &GameNode {
        let total = self
            .children
            .iter()
            .fold(0, |sum, c| sum + c.evaluation_count);

        let values = self.children.iter().map(|c| {
            let v = if c.evaluation_count == 0 {
                0.0
            } else {
                // 相手番の評価なので-1をかける（ネガMAX）
                -c.weight / c.evaluation_count as f32
            };
            v + C_PUCT * c.policy * (total as f32).sqrt() / (1.0 + c.evaluation_count as f32)
        });

        let mut index = 0usize;
        let mut max = 0.0f32;
        for (i, v) in values.enumerate() {
            if v > max {
                max = v;
                index = i;
            }
        }

        &self.children[index]
    }
}

struct GameTree {
    root_node: GameNode,
}
