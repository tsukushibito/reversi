import 'dart:async';

import 'package:app/features/game/domain/reversi.dart';

class GameApplication {
  Board _board = Board.initial();

  final _changeMessageController =
      StreamController<GameApplication>.broadcast();

  Stream<GameApplication> get changeMessage => _changeMessageController.stream;

  Board get board => _board;

  void move(Position position) {
    // 手を進める
    var action = Action(_board.turn(), position, false);
    var next = _board.applyAction(action);
    if (next == null) return;
    _board = next;
    _changeMessageController.sink.add(this);

    // パスの場合は2秒後に自動で進める
    if (!_board.isGameOver() &&
        _board.getMovablePositions(_board.turn()).isEmpty) {
      Future.delayed(const Duration(seconds: 2), () {
        var passAct = Action(_board.turn(), Position(0, 0), true);
        var nextnext = _board.applyAction(passAct);
        if (nextnext == null) {
          throw Exception("");
        }

        _board = nextnext;
        _changeMessageController.sink.add(this);
      });
    }
  }

  void reset() {
    _board = Board.initial();
    _changeMessageController.sink.add(this);
  }
}
