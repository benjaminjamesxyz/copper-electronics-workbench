import 'dart:math' as math;
import 'package:flutter/material.dart';
import '../models.dart';
import 'painter.dart';
import 'primitives.dart';

class DiagramPanel extends StatefulWidget {
  const DiagramPanel({super.key, required this.calculator, required this.diagram});
  final Calculator calculator;
  final CalculatorDiagram diagram;
  @override
  State<DiagramPanel> createState() => _DiagramPanelState();
}

class _DiagramPanelState extends State<DiagramPanel> {
  final _scroll = ScrollController();
  @override
  void dispose() {
    _scroll.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final diagram = widget.diagram;
    final c = widget.calculator;
    final quantities = <String, String>{
      for (final q in c.inputs) q.key: '${q.label} (${q.unit}); input',
      for (final q in c.outputs) q.key: '${q.label} (${q.unit}); output',
    };
    final legend = <String>{
      for (final label in diagram.marks.whereType<Label>())
        if (label.quantity != null) '${quantities[label.quantity]!} · diagram label: ${label.text}',
    };
    // Keep symbols readable at narrow widths and honor desktop text scaling.
    final scale = math.max(1.0, MediaQuery.textScalerOf(context).scale(14) / 14);
    return Container(
      key: ValueKey('diagram-${c.id}'),
      padding: const EdgeInsets.all(18),
      decoration: BoxDecoration(color: colors.surface, border: Border.all(color: colors.outlineVariant), borderRadius: BorderRadius.circular(8)),
      child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
        Text(diagram.title, style: const TextStyle(fontSize: 18, fontWeight: FontWeight.w600)),
        const SizedBox(height: 4),
        Text('Illustrative model · not to scale', style: TextStyle(fontSize: 12, color: colors.onSurfaceVariant)),
        const SizedBox(height: 8),
        LayoutBuilder(builder: (context, constraints) {
          final needsScroll = constraints.maxWidth < 700 * scale;
          return Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
            if (needsScroll) Padding(padding: const EdgeInsets.only(bottom: 8), child: Text('Scroll diagram horizontally to see all labels.', style: TextStyle(fontSize: 12, color: colors.onSurfaceVariant))),
            Scrollbar(controller: _scroll, thumbVisibility: needsScroll, interactive: true,
              child: SingleChildScrollView(controller: _scroll, scrollDirection: Axis.horizontal,
                child: Padding(padding: EdgeInsets.only(bottom: needsScroll ? 16 : 0),
                  child: Semantics(image: true, label: diagram.title,
                    child: ExcludeSemantics(child: RepaintBoundary(
                      key: ValueKey('diagram-canvas-${c.id}'),
                      child: CustomPaint(size: Size(700 * scale, 280 * scale), painter: DiagramPainter(diagram, colors)),
                    )),
                  ),
                ),
              ),
            ),
          ]);
        }),
        const SizedBox(height: 10),
        SelectableText(diagram.caption, style: const TextStyle(fontSize: 13, height: 1.5)),
        if (legend.isNotEmpty) ...[
          const SizedBox(height: 10),
          Wrap(spacing: 18, runSpacing: 6, children: legend.map((line) => Text(line, style: TextStyle(fontSize: 12, color: colors.onSurfaceVariant))).toList()),
        ],
      ]),
    );
  }
}
