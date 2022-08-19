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

  factory GameViewModelState.init() {
    var board = r.Board.initial();
    return GameViewModelState(
        squares: board.squares,
        blackCount: '',
        whiteCount: '',
        message: '',
        movables: []);
  }
}

class GameViewModel extends StateNotifier<GameViewModelState> {
  GameViewModel(r.Board board)
      : _board = board,
        super(GameViewModelState.init()) {
    state = _createState();
  }

  r.Board _board;

  void receivePosition(r.Position position) {
    // 手を進める
    var action = r.Action(_board.turn(), position, false);
    var next = _board.applyAction(action);
    if (next == null) return;
    _board = next;
    state = _createState();

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
        state = _createState();
      });
    }
  }

  void reset() {
    _board = r.Board.initial();
    state = _createState();
  }

  GameViewModelState _createState() {
    final UnmodifiableListView<int> squares = _board.squares;
    final String blackCount = 'Black: ${_board.blackCount()}';
    final String whiteCount = 'White: ${_board.whiteCount()}';
    final String message = _createMessage();
    final List<r.Position> movables = _board.getMovablePositions(_board.turn());
    return GameViewModelState(
      squares: squares,
      blackCount: blackCount,
      whiteCount: whiteCount,
      message: message,
      movables: movables,
    );
  }

  String _createMessage() {
    if (_board.isGameOver()) {
      if (_board.blackCount() > _board.whiteCount()) {
        return 'Black wins!';
      } else if (_board.blackCount() < _board.whiteCount()) {
        return 'White wins!';
      } else {
        return 'Draw';
      }
    } else {
      if (_board.getMovablePositions(_board.turn()).isEmpty) {
        return 'Pass';
      } else {
        return 'Turn: ${_board.turn() == r.black ? 'Black' : 'White'}';
      }
    }
  }
}
