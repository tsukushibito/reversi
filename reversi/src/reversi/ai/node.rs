use crate::{
    board::{BitBoard, Board},
    Move, PlayerColor,
};

pub trait Node: Sized {
    fn new(board: BitBoard, color: PlayerColor, move_count: u8, last_move: Move) -> Self;

    fn board(&self) -> &BitBoard;
    fn color(&self) -> &PlayerColor;
    fn move_count(&self) -> &u8;

    fn children(&self) -> &[Self];
    fn children_mut(&mut self) -> &mut Vec<Self>;
    fn set_children(&mut self, children: Vec<Self>);
    fn value(&self) -> &Option<i32>;
    fn value_mut(&mut self) -> &mut Option<i32>;
    fn last_move(&self) -> &Move;

    fn expand(&mut self) {
        let positions = self.board().get_movable_positions(&self.color());
        let children = positions
            .iter()
            .map(|position| {
                let action = Move::new_position(*self.color(), *position);
                let next_board = self.board().apply_move(&action).unwrap();
                Self::new(
                    next_board,
                    self.color().opponent(),
                    self.move_count() + 1,
                    action,
                )
            })
            .collect::<Vec<_>>();
        self.set_children(children);
    }

    fn node_count(&self) -> usize {
        self.children()
            .iter()
            .fold(1, |acc, child| acc + child.node_count())
    }

    fn searched_nodes(&self) -> usize {
        self.children()
            .iter()
            .filter(|child| child.value().is_some())
            .fold(1, |acc, child| acc + child.searched_nodes())
    }

    fn candidate(&self) -> Option<Vec<Move>> {
        if self.children().is_empty() {
            None
        } else {
            let mut children = self
                .children()
                .iter()
                .map(|child| (child.value(), child.last_move()))
                .collect::<Vec<_>>();
            children.sort_by(|a, b| b.0.cmp(a.0));
            Some(
                children
                    .iter()
                    .map(|(_, &action)| action)
                    .collect::<Vec<_>>(),
            )
        }
    }
}
