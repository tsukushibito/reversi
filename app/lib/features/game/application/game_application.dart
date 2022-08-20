import 'dart:async';
import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'package:app/bridge_generated.dart';
import 'package:app/features/game/domain/reversi.dart' as r;

const base = 'reversi';
final path = Platform.isWindows ? '$base.dll' : 'lib$base.so';
late final dylib = Platform.isIOS
    ? DynamicLibrary.process()
    : Platform.isMacOS
        ? DynamicLibrary.executable()
        : DynamicLibrary.open(path);
late final api = ReversiImpl(dylib);

class GameApplication {
  GameApplication() {
    _changeMessageController.stream.listen((_) => _move());
  }

  r.Board _board = r.Board.initial();

  final _changeMessageController =
      StreamController<GameApplication>.broadcast();

  Stream<GameApplication> get changeMessage => _changeMessageController.stream;

  r.Board get board => _board;

  void move(r.Position position) {
    if (_board.turn() != r.black) return;

    // 手を進める
    var action = r.Action(_board.turn(), position, false);
    var next = _board.applyAction(action);
    if (next == null) return;
    _board = next;
    _changeMessageController.sink.add(this);
  }

  void reset() {
    _board = r.Board.initial();
    _changeMessageController.sink.add(this);
  }

  void _move() async {
    if (_board.isGameOver()) return;

    if (_board.turn() == r.black) {
      // パスの場合は２秒後に自動でパス
      var isPass = !_board.isGameOver() &&
          _board.getMovablePositions(_board.turn()).isEmpty;
      if (isPass) {
        Future.delayed(const Duration(seconds: 1), () {
          var passAct = r.Action(_board.turn(), r.Position(0, 0), true);
          var next = _board.applyAction(passAct);
          if (next == null) {
            throw Exception("");
          }

          _board = next;
          _changeMessageController.sink.add(this);
        });
      }
    } else if (_board.turn() == r.white) {
      // AIのターン
      var squares = Int32List.fromList(_board.squares.toList());
      var result = await api.searchGameTree(
        squares: squares,
        turnDepth: _board.depth,
        color: _board.turn(),
        searchDepth: 9,
      );

      var action = r.Action(
          _board.turn(),
          r.Position(result.action.row, result.action.col),
          result.action.isPass);
      var next = _board.applyAction(action);
      if (next == null) {
        throw Exception("");
      }
      _board = next;
      _changeMessageController.sink.add(this);
    }
  }
}
