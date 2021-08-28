import unittest
from reversi import *
from reversi_ai import GameTreeNode


class TestGameTree(unittest.TestCase):
    def test_game_tree_node(self):
        state = GameState(Board.initial())
        node = GameTreeNode(state)
        pass
