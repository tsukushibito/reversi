from dataclasses import dataclass
from typing import ClassVar, Dict, List
from math import sqrt
import random

import numpy as np

from reversi import Square, GameState
from reversi.game_state import Action
from reversi_ai import DualNetwork


@dataclass
class MctsTreeNode:
    """ゲーム木ノード

    Attributes:
        children(List[GameTreeNode]): 子ノードリスト
        state(GameState): ゲーム状態
        policy: 方策(このノードが選ばれる確率)
        weight: 累計価値
        evaluated_count: 試行回数
    """
    C_PUCT: ClassVar[float] = 1.0

    state: GameState = None
    children: List['MctsTreeNode'] = None
    actions: List[Action] = None
    policy: float = 0.0
    weight: float = 0.0
    evaluated_count: int = 0

    def new(state: GameState, policy: float = 0) -> 'MctsTreeNode':
        return MctsTreeNode(state=state,
                            children=None,
                            actions=state.valid_actions(),
                            policy=policy,
                            weight=0,
                            evaluated_count=0)


@dataclass
class SearchResult:
    scores: List[float] = None
    actions: List[Action] = None

    def select_action(self) -> Action:
        return np.random.choice(self.actions, p=self.scores)


class Mcts:
    """モンテカルロ木探索クラス
    Attributes:
        _dual_network: デュアルネットワーク
        _node_data_table: ノードデータテーブル ノードをキーとして、そのノードに対応するデータを値とします
    """
    C_PUCT: ClassVar[float] = 1.0

    def __init__(self, dual_network: DualNetwork):
        """[summary]

        Args:
            dual_network (DualNetwork): [description]
        """
        self._dual_network: DualNetwork = dual_network

    def search(self, node: MctsTreeNode, evaluation_count: int, temperature: float) -> SearchResult:
        """探索しスコアとアクションを返す

        Args:
            node (MctsTreeNode): 探索開始ノード
            evaluation_count (int): 評価回数
            temperature (float): ばらつき分布用温度パラメータ

        Returns:
            SearchResult: 探索結果
        """
        for _ in range(evaluation_count):
            self._evaluate_node(node)

        scores = [c.evaluated_count for c in node.children]

        if temperature == 0:
            # 最大値のみ1でそれ以外は0
            index = np.argmax(scores)
            scores = np.zeros(len(scores))
            scores[index] = 1
        else:
            # ボルツマン分布でばらつき付与
            scores = [s ** (1 / temperature) for s in scores]
            scores = [s / sum(scores) for s in scores]

        return SearchResult(scores=scores, actions=node.actions)

    def _evaluate_node(self, node: MctsTreeNode) -> float:
        """ノードを評価

        Args:
            node (GameTreeNode): ノード

        Returns:
            float: 評価値
        """

        value: float = 0.0

        # ゲーム終了時
        if node.state.is_end():
            # 勝敗結果で価値を取得
            black_count: int = node.state.black_count()
            white_count: int = node.state.white_count()
            is_draw: bool = black_count == white_count
            wins: bool = black_count > white_count \
                if node.state.color == Square.BLACK \
                else white_count > black_count
            value: int = 0 if is_draw else \
                1 if wins else \
                -1

        # 子ノードが存在しない時
        elif node.children is None:
            # ニューラルネットワークの推論で方策と価値を取得
            policies, value = self._dual_network.predict(node.state)

            # 子ノードの展開
            next_states = [node.state.next_state(
                action) for action in node.actions]
            node.children = [MctsTreeNode.new(state=next_state, policy=policy)
                             for next_state, policy in zip(next_states, policies)]

        # 子ノードが存在する時
        else:
            # アーク評価値が最大の子ノードの評価で価値を取得
            next_node: MctsTreeNode = self._select_next_node(node.children)
            # 相手番の評価値を得るので-1を掛ける
            value = -self._evaluate_node(next_node)

        # 累計価値と試行回数の更新
        node.weight += value
        node.evaluated_count += 1

        return value

    def _select_next_node(self, nodes: List[MctsTreeNode]) -> MctsTreeNode:
        # アーク評価値の計算
        t = sum([n.evaluated_count for n in nodes])

        shuffled = random.sample(nodes, len(nodes))

        def pucb_value(node: MctsTreeNode):
            lhs = -node.weight / node.evaluated_count \
                if node.evaluated_count != 0 \
                else 0.0
            rhs = Mcts.C_PUCT * node.policy * \
                sqrt(t) / (1 + node.evaluated_count)
            return lhs * rhs

        pucb_values = list(map(pucb_value, shuffled))

        # アーク評価値が最大の子ノードを返す
        return shuffled[np.argmax(pucb_values)]


def expand_node(node: MctsTreeNode, depth: int):
    if node.children is None:
        next_states = [node.state.next_state(
            action) for action in node.actions]
        node.children = [MctsTreeNode.new(
            state=next_state, policy=0) for next_state in next_states]
    depth = depth - 1
    if depth > 0:
        for c in node.children:
            expand_node(c, depth)
