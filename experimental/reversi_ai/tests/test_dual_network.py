import enum
import unittest
from reversi import Board

from reversi.game_state import GameState
from reversi_ai import DualNetwork, ResnetBlock
import os
import shutil


class TestDualNetwork(unittest.TestCase):
    def test_dual_network(self):
        model_file = './tests/models/test_model.h5'
        if os.path.exists(model_file):
            if os.path.isdir(model_file):
                shutil.rmtree(model_file)
            else:
                os.remove(model_file)

        new_dn = DualNetwork.create_new()
        new_dn.save_model(model_file)
        self.assertTrue(os.path.exists(model_file))

        game_state = GameState(board=Board.initial())

        policies0, value0 = new_dn.predict(game_state)

        loaded_dn = DualNetwork.create_from_file(model_file)

        policies1, value1 = loaded_dn.predict(game_state)

        self.assertEqual(value0, value1)
        self.assertEqual(len(policies0), len(policies1))
        for p0, p1 in zip(policies0, policies1):
            self.assertEqual(p0, p1)
