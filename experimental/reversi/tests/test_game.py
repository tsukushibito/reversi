import unittest
from reversi import GameState, Action, Square, Game, Player, Move


class TestPlayer(Player):
    def __init__(self):
        self.action_table = [
            Action(color=Square.BLACK, move=Move((4, 5))),
            Action(color=Square.WHITE, move=Move((5, 3))),
            Action(color=Square.BLACK, move=Move((4, 2))),
            Action(color=Square.WHITE, move=Move((3, 5))),
            Action(color=Square.BLACK, move=Move((6, 4))),
            Action(color=Square.WHITE, move=Move((5, 5))),
            Action(color=Square.BLACK, move=Move((4, 6))),
            Action(color=Square.WHITE, move=Move((5, 4))),
            Action(color=Square.BLACK, move=Move((2, 4))),
            Action(color=Square.WHITE, move=Move.PASS),
            Action(color=Square.BLACK, move=Move.PASS),
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
