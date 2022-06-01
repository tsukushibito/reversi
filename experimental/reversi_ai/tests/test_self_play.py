import unittest
from reversi import *
from reversi_ai import *


class TestSelfPlay(unittest.TestCase):
    def test_mcts_search(self):
        dual_network = DualNetwork.create_new()
        self_play = SelfPlay(dual_network, 10, 1)
        datas = self_play.run()
