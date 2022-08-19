import 'dart:async';
import 'dart:collection';

import 'package:app/features/game/models/reversi.dart' as r;

class GameViewModel {
  GameViewModel(r.Board board) : _board = board;
  GameViewModel.init() : _board = r.Board.initial();

  r.Board _board;

  final _updateEventController = StreamController<GameViewModel>.broadcast();

  Stream<GameViewModel> get updateEvent => _updateEventController.stream;
  UnmodifiableListView<int> get squares => _board.squares;
  String get blackCount => _board.blackCount().toString();
  String get whiteCount => _board.whiteCount().toString();
  String get message => _createMessage();
  List<r.Position> get movables => _board.getMovablePositions(_board.turn());

  void receivePosition(r.Position position) {
    var action = r.Action(_board.turn(), position, false);
    var next = _board.applyAction(action);
    if (next == null) return;
    _board = next;
    _updateEventController.add(GameViewModel(_board));

    if (!_board.isGameOver() &&
        _board.getMovablePositions(_board.turn()).isEmpty) {
      Future.delayed(const Duration(seconds: 2), () {
        var passAct = r.Action(_board.turn(), r.Position(0, 0), true);
        var next = _board.applyAction(passAct);
        if (next == null) {
          throw Exception("");
        }
        _board = next;
        _updateEventController.add(GameViewModel(_board));
      });
    }
  }

  void reset() {
    _board = r.Board.initial();
    _updateEventController.sink.add(GameViewModel(_board));
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
