import unittest
from reversi import *
from reversi_ai import *


class TestTraining(unittest.TestCase):
    def test_training(self):
        param = TrainingParameter(
            model_file_path='./models/model.h5',
            evaluation_count=10,
            self_play_count=2,
            datas_directory='./datas/'
        )

        training = Training(param)
        training.train()
