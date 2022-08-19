import 'dart:collection';

import 'package:app/features/game/domain/reversi.dart' as r;
import 'package:flutter/material.dart';

class Board extends StatelessWidget {
  final UnmodifiableListView<int> board;
  final List<r.Position> movables;
  final void Function(int, int) onTap;

  const Board({
    super.key,
    required this.board,
    required this.movables,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return AspectRatio(
      aspectRatio: 1,
      child: Container(
        color: Colors.black,
        padding: const EdgeInsets.all(1),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: List.generate(
            r.boardSize,
            (row) => Expanded(
              child: Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: List.generate(
                  r.boardSize,
                  (col) {
                    var index = r.positionToIndex(r.Position(row, col));
                    var color = board[index];
                    var movable =
                        movables.any((m) => m.row == row && m.col == col);
                    return Expanded(
                      child: _Cell(
                        row: row,
                        col: col,
                        color: color,
                        movable: movable,
                        onTap: onTap,
                      ),
                    );
                  },
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class _Cell extends StatelessWidget {
  final int row;
  final int col;
  final int color;
  final bool movable;
  final void Function(int, int) onTap;

  const _Cell({
    super.key,
    required this.row,
    required this.col,
    required this.color,
    required this.movable,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: () => onTap(row, col),
      child: Container(
        alignment: Alignment.center,
        color: movable ? Colors.greenAccent : Colors.green,
        margin: const EdgeInsets.all(1.0),
        padding: const EdgeInsets.all(5),
        child: color == r.black
            ? Container(
                decoration: const BoxDecoration(
                  color: Colors.black,
                  shape: BoxShape.circle,
                ),
              )
            : color == r.white
                ? Container(
                    decoration: const BoxDecoration(
                      color: Colors.white,
                      shape: BoxShape.circle,
                    ),
                  )
                : const SizedBox.shrink(),
      ),
    );
  }
}
