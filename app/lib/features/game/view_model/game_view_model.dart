import 'dart:async';
import 'dart:collection';

import 'package:app/features/game/domain/reversi.dart' as r;
import 'package:hooks_riverpod/hooks_riverpod.dart';

class GameViewModelState {
  final UnmodifiableListView<int> squares;
  final String blackCount;
  final String whiteCount;
  final String message;
  final List<r.Position> movables;

  GameViewModelState({
    required this.squares,
    required this.blackCount,
    required this.whiteCount,
    required this.message,
    required this.movables,
  });
}

class GameViewModel extends StateNotifier<GameViewModelState> {
  GameViewModel(r.Board board)
      : _board = board,
        super(_createState(board));

  r.Board _board;

  void receivePosition(r.Position position) {
    // 手を進める
    var action = r.Action(_board.turn(), position, false);
    var next = _board.applyAction(action);
    if (next == null) return;
    _board = next;
    state = _createState(next);

    // パスの場合は2秒後に自動で進める
    if (!_board.isGameOver() &&
        _board.getMovablePositions(_board.turn()).isEmpty) {
      Future.delayed(const Duration(seconds: 2), () {
        var passAct = r.Action(_board.turn(), r.Position(0, 0), true);
        var nextnext = _board.applyAction(passAct);
        if (nextnext == null) {
          throw Exception("");
        }

        _board = nextnext;
        state = _createState(nextnext);
      });
    }
  }

  void reset() {
    _board = r.Board.initial();
    state = _createState(_board);
  }

  static GameViewModelState _createState(r.Board board) {
    final UnmodifiableListView<int> squares = board.squares;
    final String blackCount = 'Black: ${board.blackCount()}';
    final String whiteCount = 'White: ${board.whiteCount()}';
    final String message = _createMessage(board);
    final List<r.Position> movables = board.getMovablePositions(board.turn());
    return GameViewModelState(
      squares: squares,
      blackCount: blackCount,
      whiteCount: whiteCount,
      message: message,
      movables: movables,
    );
  }

  static String _createMessage(r.Board board) {
    if (board.isGameOver()) {
      if (board.blackCount() > board.whiteCount()) {
        return 'Black wins!';
      } else if (board.blackCount() < board.whiteCount()) {
        return 'White wins!';
      } else {
        return 'Draw';
      }
    } else {
      if (board.getMovablePositions(board.turn()).isEmpty) {
        return 'Pass';
      } else {
        return 'Turn: ${board.turn() == r.black ? 'Black' : 'White'}';
      }
    }
  }
}
