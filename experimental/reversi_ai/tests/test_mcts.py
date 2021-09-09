import unittest
from reversi import *
from reversi_ai import Mcts, MctsTreeNode, DualNetwork


class TestMcts(unittest.TestCase):
    def test_mcts(self):
        state = GameState(Board.initial())
        node = MctsTreeNode(state)
        dual_network = DualNetwork.create_new()
        mcts = Mcts(dual_network=dual_network)
        mcts._evaluate_node(node)

        self.assertEqual(len(node.children), 4)
