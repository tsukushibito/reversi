import 'dart:collection';

import 'package:app/features/game/models/reversi.dart' as reversi;
import 'package:flutter/material.dart';

class Board extends StatelessWidget {
  final UnmodifiableListView<int> board;
  final void Function(int, int) onTap;

  const Board({
    super.key,
    required this.board,
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
            reversi.boardSize,
            (row) => Expanded(
              child: Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: List.generate(
                  reversi.boardSize,
                  (col) {
                    var index =
                        reversi.positionToIndex(reversi.Position(row, col));
                    var color = board[index];
                    return Expanded(
                      child: _Cell(
                        row: row,
                        col: col,
                        color: color,
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
  final void Function(int, int) onTap;

  const _Cell({
    super.key,
    required this.row,
    required this.col,
    required this.color,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: () => onTap(row, col),
      child: Container(
        alignment: Alignment.center,
        color: Colors.green,
        margin: const EdgeInsets.all(1.0),
        padding: const EdgeInsets.all(5),
        child: color == reversi.black
            ? Container(
                decoration: const BoxDecoration(
                  color: Colors.black,
                  shape: BoxShape.circle,
                ),
              )
            : color == reversi.white
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
