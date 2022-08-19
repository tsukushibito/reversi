import 'dart:collection';
import 'dart:math';

const boardSize = 8;
const empty = 0;
const black = 1;
const white = 2;

class Action {
  final int color;
  final Position position;
  final bool isPass;

  Action(this.color, this.position, this.isPass);
}

class Position {
  final int row;
  final int col;

  Position(this.row, this.col);
}

int positionToIndex(Position pos) {
  return pos.row * boardSize + pos.col;
}

Position indexToPosition(int index) {
  if (index < 0 || index >= boardSize * boardSize) {
    throw ArgumentError("無効なindex");
  }

  final row = index ~/ boardSize;
  final col = index % boardSize;
  return Position(row, col);
}

class Board {
  final List<int> _squares;
  UnmodifiableListView<int> get squares => UnmodifiableListView(_squares);

  final int depth;

  Board({required List<int> squares, required this.depth}) : _squares = squares;

  Board.initial()
      : _squares = _initial(),
        depth = 0;

  static List<int> _initial() {
    var squares = List<int>.filled(boardSize * boardSize, empty);
    squares[positionToIndex(Position(3, 4))] = black;
    squares[positionToIndex(Position(4, 3))] = black;
    squares[positionToIndex(Position(3, 3))] = white;
    squares[positionToIndex(Position(4, 4))] = white;
    return squares;
  }

  int squareCount(int color) {
    return _squares.where((element) => element == color).toList().length;
  }

  Board? applyAction(Action action) {
    if (action.color != turn()) return null;

    if (action.isPass) {
      var movables = getMovablePositions(action.color);
      if (movables.isEmpty) {
        return Board(squares: _squares, depth: depth + 1);
      } else {
        return null;
      }
    } else {
      var index = positionToIndex(action.position);
      if (_squares[index] != empty) {
        return null;
      }

      var movables = getMovablePositions(action.color);
      if (movables
          .where((element) =>
              element.row == action.position.row &&
              element.col == action.position.col)
          .isNotEmpty) {
        var squares = [..._squares];

        squares[index] = action.color;

        for (var dir in _directions) {
          var flip = _getFlipCount(action.color, action.position, dir);
          var posRow = action.position.row;
          var posCol = action.position.col;
          for (var a = 0; a < flip; ++a) {
            posRow += dir.y;
            posCol += dir.x;
            var i = positionToIndex(Position(posRow, posCol));
            squares[i] = action.color;
          }
        }

        return Board(squares: squares, depth: depth + 1);
      } else {
        return null;
      }
    }
  }

  List<Position> getMovablePositions(int color) {
    List<Position> positions = [];
    for (var row = 0; row < boardSize; ++row) {
      for (var col = 0; col < boardSize; ++col) {
        var pos = Position(row, col);
        var index = positionToIndex(pos);
        if (_squares[index] != empty) {
          continue;
        }
        for (var dir in _directions) {
          var flip = _getFlipCount(color, pos, dir);
          if (flip > 0) {
            positions.add(pos);
            break;
          }
        }
      }
    }
    return positions;
  }

  bool isGameOver() {
    return getMovablePositions(black).isEmpty &&
        getMovablePositions(white).isEmpty;
  }

  int blackCount() {
    return squareCount(black);
  }

  int whiteCount() {
    return squareCount(white);
  }

  int emptyCount() {
    return squareCount(empty);
  }

  int turn() {
    if (depth % 2 == 0) {
      return black;
    } else {
      return white;
    }
  }

  int _getFlipCount(int color, Position pos, _Direction dir) {
    var r = pos.row + dir.y;
    var c = pos.col + dir.x;
    while (true) {
      var targetPos = Position(r, c);
      if (!isValidPos(targetPos)) {
        break;
      }
      var index = positionToIndex(targetPos);
      var s = _squares[index];
      if (s == color || s == empty) {
        break;
      }
      r += dir.y;
      c += dir.x;
    }

    var targetPos = Position(r, c);
    if (isValidPos(targetPos) &&
        _squares[positionToIndex(targetPos)] == color) {
      var d = getDistance(pos, targetPos);
      if (d >= 2) {
        return d;
      }
    }

    return 0;
  }
}

class _Direction {
  final int x;
  final int y;

  const _Direction(this.x, this.y);
}

const List<_Direction> _directions = [
  _Direction(-1, -1),
  _Direction(0, -1),
  _Direction(1, -1),
  _Direction(-1, 0),
  _Direction(1, 0),
  _Direction(-1, 1),
  _Direction(0, 1),
  _Direction(1, 1),
];

bool isValidPos(Position pos) {
  return pos.row >= 0 &&
      pos.row < boardSize &&
      pos.col >= 0 &&
      pos.col < boardSize;
}

int getDistance(Position p0, Position p1) {
  return max((p0.row - p1.row).abs(), (p0.col - p1.col).abs());
}
