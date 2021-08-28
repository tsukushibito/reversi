from dataclasses import dataclass
from typing import List
from reversi import GameState


@dataclass
class GameTreeNode:
    value: float = 0
    policies: List[float] = None
    children: List['GameTreeNode'] = None
