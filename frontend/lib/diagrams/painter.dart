import 'dart:math' as math;
import 'package:flutter/material.dart';
import 'primitives.dart';

/// Draws vector primitives at their documented electrical connector positions.
class DiagramPainter extends CustomPainter {
  DiagramPainter(this.diagram, this.colors);
  final CalculatorDiagram diagram;
  final ColorScheme colors;

  Color color(DiagramInk ink) => switch (ink) {
    DiagramInk.wire => colors.onSurface,
    DiagramInk.input => colors.tertiary,
    DiagramInk.output => colors.primary,
    DiagramInk.muted => colors.onSurfaceVariant,
  };

  @override
  void paint(Canvas canvas, Size size) {
    canvas.save();
    canvas.scale(size.width / 700, size.height / 280);
    for (final mark in diagram.marks) {
      switch (mark) {
        case Wire():
          final paint = _stroke(mark.ink);
          final path = Path()..moveTo(mark.points[0], mark.points[1]);
          for (var i = 2; i < mark.points.length; i += 2) {
            path.lineTo(mark.points[i], mark.points[i + 1]);
          }
          if (mark.dashed) {
            for (final metric in path.computeMetrics()) {
              for (double start = 0; start < metric.length; start += 11) {
                canvas.drawPath(metric.extractPath(start, math.min(start + 6, metric.length)), paint);
              }
            }
          } else {
            canvas.drawPath(path, paint);
          }
          if (mark.arrow) {
            final n = mark.points.length;
            _arrow(canvas, Offset(mark.points[n - 4], mark.points[n - 3]),
                Offset(mark.points[n - 2], mark.points[n - 1]), paint);
          }
        case Label():
          final text = TextPainter(
            text: TextSpan(text: mark.text, style: TextStyle(color: color(mark.ink), fontSize: mark.size, fontWeight: FontWeight.w500)),
            textDirection: TextDirection.ltr,
          )..layout();
          text.paint(canvas, Offset(mark.x, mark.y));
          text.dispose();
        case Dot():
          canvas.drawCircle(Offset(mark.x, mark.y), 3.2, Paint()..color = color(mark.ink));
        case Part():
          canvas.save();
          canvas.translate(mark.x, mark.y);
          canvas.rotate(mark.turns * math.pi / 2);
          _part(canvas, mark);
          canvas.restore();
      }
    }
    canvas.restore();
  }

  Paint _stroke(DiagramInk ink) => Paint()
    ..color = color(ink)
    ..style = PaintingStyle.stroke
    ..strokeWidth = 1.8
    ..strokeCap = StrokeCap.round
    ..strokeJoin = StrokeJoin.round;

  void _arrow(Canvas canvas, Offset start, Offset end, Paint paint) {
    final vector = end - start;
    if (vector.distance == 0) return;
    final unit = vector / vector.distance;
    final normal = Offset(-unit.dy, unit.dx);
    final base = end - unit * 8;
    canvas.drawPath(Path()..moveTo((base + normal * 4).dx, (base + normal * 4).dy)
      ..lineTo(end.dx, end.dy)..lineTo((base - normal * 4).dx, (base - normal * 4).dy), paint);
  }

  void _part(Canvas canvas, Part part) {
    final p = _stroke(part.ink);
    final w = part.width, h = part.height;
    void line(double x1, double y1, double x2, double y2) =>
        canvas.drawLine(Offset(x1, y1), Offset(x2, y2), p);
    void text(String value, double x, double y) {
      final label = TextPainter(text: TextSpan(text: value, style: TextStyle(fontSize: 14, color: p.color)), textDirection: TextDirection.ltr)..layout();
      canvas.save();
      canvas.translate(x, y);
      canvas.rotate(-part.turns * math.pi / 2);
      label.paint(canvas, Offset(-label.width / 2, -label.height / 2));
      canvas.restore();
      label.dispose();
    }
    switch (part.kind) {
      case CircuitPart.resistor:
        final path = Path()..moveTo(-w / 2, 0)..lineTo(-w * .3, 0);
        for (var i = 0; i < 6; i++) {
          path.lineTo(-w * .25 + i * w * .1, (i.isEven ? -1 : 1) * h * .23);
        }
        path.lineTo(w * .3, 0); path.lineTo(w / 2, 0);
        canvas.drawPath(path, p);
      case CircuitPart.capacitor:
        line(-w / 2, 0, -w * .09, 0); line(w * .09, 0, w / 2, 0);
        line(-w * .09, -h / 2, -w * .09, h / 2); line(w * .09, -h / 2, w * .09, h / 2);
      case CircuitPart.inductor:
        final path = Path()..moveTo(-w / 2, 0)..lineTo(-w * .32, 0);
        for (var i = 0; i < 4; i++) {
          final x = -w * .32 + i * w * .16;
          path.cubicTo(x, -h * .55, x + w * .16, -h * .55, x + w * .16, 0);
        }
        path.lineTo(w / 2, 0); canvas.drawPath(path, p);
      case CircuitPart.voltageSource:
      case CircuitPart.currentSource:
        final r = math.min(w, h) / 2;
        canvas.drawCircle(Offset.zero, r, p);
        line(-w / 2, 0, -r, 0); line(r, 0, w / 2, 0);
        if (part.kind == CircuitPart.voltageSource) {
          text('+', -r * .48, 0); text('−', r * .48, 0);
        } else {
          line(-r * .55, 0, r * .55, 0);
          _arrow(canvas, Offset(-r * .55, 0), Offset(r * .55, 0), p);
        }
      case CircuitPart.ground:
        line(0, -h / 2, 0, 0);
        line(-w / 2, 0, w / 2, 0);
        line(-w * .3, h * .22, w * .3, h * .22);
        line(-w * .1, h * .44, w * .1, h * .44);
      case CircuitPart.diode:
        line(-w / 2, 0, -w * .2, 0); line(w * .2, 0, w / 2, 0);
        canvas.drawPath(Path()..moveTo(-w * .2, -h / 2)..lineTo(w * .2, 0)..lineTo(-w * .2, h / 2)..close(), p);
        line(w * .2, -h / 2, w * .2, h / 2);
      case CircuitPart.opamp:
        canvas.drawPath(Path()..moveTo(-w / 2, -h / 2)..lineTo(w / 2, 0)..lineTo(-w / 2, h / 2)..close(), p);
        text('+', -w * .34, -h / 4); text('−', -w * .34, h / 4);
      case CircuitPart.nmos:
        line(-w / 2, 0, -w * .28, 0); line(-w * .28, -h * .3, -w * .28, h * .3);
        line(-w * .12, -h * .3, -w * .12, h * .3);
        line(0, -h / 2, 0, -h * .25); line(0, -h * .25, -w * .12, -h * .25);
        line(0, h / 2, 0, h * .25); line(0, h * .25, -w * .12, h * .25);
      case CircuitPart.bjt:
        line(-w / 2, 0, -w * .18, 0); line(-w * .18, -h * .3, -w * .18, h * .3);
        line(-w * .18, -h * .1, 0, -h * .32); line(0, -h * .32, 0, -h / 2);
        line(-w * .18, h * .1, 0, h * .32); line(0, h * .32, 0, h / 2);
        _arrow(canvas, Offset(-w * .18, h * .1), Offset(0, h * .32), p);
      case CircuitPart.switchContact:
        line(-w / 2, 0, -w * .22, 0); line(-w * .22, 0, w * .2, -h * .35);
        line(w * .22, 0, w / 2, 0);
        canvas.drawCircle(Offset(-w * .22, 0), 2.5, p); canvas.drawCircle(Offset(w * .22, 0), 2.5, p);
      case CircuitPart.block:
        canvas.drawRect(Rect.fromCenter(center: Offset.zero, width: w, height: h), p);
      case CircuitPart.circle:
        canvas.drawOval(Rect.fromCenter(center: Offset.zero, width: w, height: h), p);
    }
  }

  @override
  bool shouldRepaint(DiagramPainter oldDelegate) => oldDelegate.diagram != diagram || oldDelegate.colors != colors;
}
