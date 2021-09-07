from typing import List, Tuple

import numpy as np
import tensorflow as tf
from tensorflow.keras.regularizers import l2
from tensorflow.keras import backend as K

from reversi import Board, Square, GameState

from reversi_ai.resnet_block import ResnetBlock

import os


class DualNetwork:
    FILTER_COUNT = 128  # 畳み込み層のカーネル数（本家は256）
    RESIDUAL_COUNT = 16  # 残差ブロックの数（本家は19）
    INPUT_SHAPE = (Board.BOARD_SIZE, Board.BOARD_SIZE, 2)  # 入力シェイプ
    POLICY_COUNT = Board.BOARD_SIZE * \
        Board.BOARD_SIZE + 1  # 行動数(配置先(8*8)+パス(1))
    POLICIES_INDEX = 0
    VALUE_INDEX = 1

    @classmethod
    def create_new(cls) -> 'DualNetwork':
        K.clear_session()

        # 入力層
        input = tf.keras.layers.Input(shape=DualNetwork.INPUT_SHAPE)

        # 畳み込み層
        x = tf.keras.layers.Conv2D(DualNetwork.FILTER_COUNT,
                                   3,
                                   padding='same',
                                   use_bias=False,
                                   kernel_initializer='he_normal',
                                   kernel_regularizer=l2(0.0005))(input)
        x = tf.keras.layers.BatchNormalization()(x)
        x = tf.keras.layers.Activation('relu')(x)

        # 残差ブロック
        for _ in range(DualNetwork.RESIDUAL_COUNT):
            x = ResnetBlock(filter_size=DualNetwork.FILTER_COUNT)(x)

        # プーリング層
        x = tf.keras.layers.GlobalAveragePooling2D()(x)

        # ポリシー出力
        p = tf.keras.layers.Dense(DualNetwork.POLICY_COUNT, kernel_regularizer=l2(0.0005),
                                  activation='softmax', name='pi')(x)

        # バリュー出力
        v = tf.keras.layers.Dense(
            1, kernel_regularizer=l2(0.0005))(x)
        v = tf.keras.layers.Activation('tanh', name='v')(v)

        # モデルの作成
        model = tf.keras.Model(inputs=input, outputs=[p, v])

        # モデルのコンパイル
        model.compile(loss=['categorical_crossentropy', 'mse'],
                      optimizer='adam')

        return DualNetwork(model)

    @classmethod
    def create_from_file(cls, model_file_path: str) -> 'DualNetwork':
        if not os.path.exists(model_file_path):
            return cls.create_new()

        K.clear_session()
        model = tf.keras.models.load_model(
            model_file_path, custom_objects={'ResnetBlock': ResnetBlock})

        return DualNetwork(model)

    def __init__(self, model: tf.keras.Model) -> None:
        self.model = model

    def save_model(self, file_path) -> None:
        # モデルの保存
        dir = os.path.dirname(file_path)
        os.makedirs(dir, exist_ok=True)  # フォルダがない時は生成
        self.model.save(file_path)

    def predict(self, state: GameState) -> Tuple[List[float], float]:
        # 推論のための入力データのシェイプの変換
        a, b, c = DualNetwork.INPUT_SHAPE
        blacks = state.black_squares()
        whites = state.white_squares()
        x = np.array([blacks, whites]) \
            if state.color == Square.BLACK \
            else np.array([whites, blacks])
        x = x.reshape(c, a, b).transpose(1, 2, 0).reshape(1, a, b, c)

        # 推論
        # 出力はList[np.ndarray]
        # 0: Policyes, 1: Value
        # ndarrayのシェイプは(batch_size, 65 or 1)
        y = self.model.predict(x, batch_size=1)

        # 方策の取得
        valid_actions = state.valid_actions()
        valid_positions = list(map(lambda a:
                                   Board.BOARD_SIZE * a.row + a.col
                                   if not a.is_pass
                                   else Board.BOARD_SIZE * Board.BOARD_SIZE + 1,
                                   valid_actions))
        policies = y[DualNetwork.POLICIES_INDEX][0][valid_positions]  # 合法手のみ
        policies /= sum(policies) if sum(policies) else 1  # 合計1の確率分布に変換

        # 価値の取得
        value = y[DualNetwork.VALUE_INDEX][0][0]
        return policies, value
