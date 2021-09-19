import unittest
import time
from reversi import *
from reversi_ai import Mcts, MctsTreeNode, DualNetwork, expand_node


class TestMcts(unittest.TestCase):
    def test_mcts_search(self):
        state = GameState(Board.initial())
        node = MctsTreeNode.new(state)
        dual_network = DualNetwork.create_new()
        mcts = Mcts(dual_network=dual_network)

        result = mcts.search(node, 10, 1)

        self.assertEqual(len(node.children), 4)

    def test_mcts_expand_nodes(self):
        state = GameState(Board.initial())
        node = MctsTreeNode.new(state)

        for depth in [4, 5, 6, 7]:
            start = time.perf_counter()
            expand_node(node, depth)
            elapsed = time.perf_counter() - start
            print('depth: {}, elapsed: {}', depth, elapsed)
