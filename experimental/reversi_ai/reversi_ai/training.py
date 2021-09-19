from dataclasses import dataclass
from typing import List
import os
import pickle
import datetime

from reversi_ai import DualNetwork, SelfPlay, SearchRecord


@dataclass
class TrainingParameter:
    model_file_path: str = './models/model.h5'
    evaluation_count: int = 50
    self_play_count: int = 500
    datas_directory: str = './datas/'


class Training:
    def __init__(self, param: TrainingParameter) -> None:
        self._parameter = param

    def train(self):
        if os.path.exists(self._parameter.model_file_path):
            dual_network = DualNetwork.create_from_file(
                self._parameter.model_file_path)
        else:
            dual_network = DualNetwork.create_new()

        datas: List[SearchRecord] = []
        for _ in range(self._parameter.self_play_count):
            self_play = SelfPlay(
                dual_network, self._parameter.evaluation_count, 1.0)

            datas.extend(self_play.run())

        now = datetime.datetime.now()
        os.makedirs(self._parameter.datas_directory,
                    exist_ok=True)  # フォルダがない時は生成
        path = self._parameter.datas_directory + '{:04}{:02}{:02}{:02}{:02}{:02}.dat'.format(
            now.year, now.month, now.day, now.hour, now.minute, now.second)
        with open(path, mode='wb') as f:
            pickle.dump(datas, f)
