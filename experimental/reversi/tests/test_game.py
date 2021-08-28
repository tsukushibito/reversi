import unittest
from reversi import GameState, Action, Square, Game, Player


class TestPlayer(Player):
    def __init__(self):
        self.action_table = [
            Action(Square.BLACK, 4, 5),
            Action(Square.WHITE, 5, 3),
            Action(Square.BLACK, 4, 2),
            Action(Square.WHITE, 3, 5),
            Action(Square.BLACK, 6, 4),
            Action(Square.WHITE, 5, 5),
            Action(Square.BLACK, 4, 6),
            Action(Square.WHITE, 5, 4),
            Action(Square.BLACK, 2, 4),
            Action(color=Square.WHITE, is_pass=True),
            Action(color=Square.BLACK, is_pass=True),
        ]

    def take_action(self, state: GameState) -> Action:
        print(state.to_string())
        return self.action_table[state.depth]


class TestGame(unittest.TestCase):
    def setUp(self) -> None:
        super().setUp()

    def test_game_run(self):
        player = TestPlayer()
        game: Game = Game(player, player)

        result = game.run()

        self.assertEqual(result.black, 13)
