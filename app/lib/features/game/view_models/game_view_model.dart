import 'dart:async';
import 'dart:collection';

import 'package:app/features/game/models/reversi.dart' as r;

class GameViewModel {
  GameViewModel(r.Board board) : _board = board;
  GameViewModel.init() : _board = r.Board.initial();

  final r.Board _board;

  final _updateEventController = StreamController<GameViewModel>.broadcast();

  Stream<GameViewModel> get updateEvent => _updateEventController.stream;
  UnmodifiableListView<int> get squares => _board.squares;
  String get blackCount => 'Black: ${_board.blackCount()}';
  String get whiteCount => 'White: ${_board.whiteCount()}';
  String get message => _createMessage();
  List<r.Position> get movables => _board.getMovablePositions(_board.turn());

  void receivePosition(r.Position position) {
    // 手を進める
    var action = r.Action(_board.turn(), position, false);
    var next = _board.applyAction(action);
    if (next == null) return;
    _updateEventController.add(GameViewModel(next));

    // パスの場合は2秒後に自動で進める
    if (!next.isGameOver() && next.getMovablePositions(next.turn()).isEmpty) {
      Future.delayed(const Duration(seconds: 2), () {
        var passAct = r.Action(next.turn(), r.Position(0, 0), true);
        var nextnext = next.applyAction(passAct);
        if (nextnext == null) {
          throw Exception("");
        }
        _updateEventController.add(GameViewModel(nextnext));
      });
    }
  }

  void reset() {
    _updateEventController.sink.add(GameViewModel(r.Board.initial()));
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
