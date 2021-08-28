import unittest
from reversi import Board, GameState, Action, Square


class TestGameState(unittest.TestCase):
    def setUp(self) -> None:
        super().setUp()
        self.state = GameState(Board.initial())

    def test_game_state_next_state_returns_None_if_action_is_invalid(self):
        next_state = self.state.next_state(
            Action(color=Square.WHITE, row=0, col=0, is_pass=False))
        self.assertIsNone(next_state)

        next_state = self.state.next_state(
            Action(color=Square.BLACK, row=0, col=0, is_pass=False))
        self.assertIsNone(next_state)

        next_state = self.state.next_state(
            Action(color=Square.BLACK, row=0, col=0, is_pass=True))
        self.assertIsNone(next_state)

    def test_game_state_next_state_returns_next_state_if_action_is_valid(self):
        print(self.state.to_string())

        next_state = self.state.next_state(
            Action(color=Square.BLACK, row=2, col=3))

        print(next_state.to_string())

        for r in range(Board.BOARD_SIZE):
            for c in range(Board.BOARD_SIZE):
                s = Square.BLACK if r == 2 and c == 3 \
                    or r == 3 and c == 3 \
                    or r == 3 and c == 4 \
                    or r == 4 and c == 3 \
                    else Square.WHITE if r == 4 and c == 4 else Square.EMPTY
                self.assertEqual(next_state.board[r, c], s, f"r: {r}, c: {c}")
