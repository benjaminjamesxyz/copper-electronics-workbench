/// Logical coordinates are 700 × 280. Illustrations are explanatory, not plots
/// of computed values. Quantity labels bind to the Rust catalog's exact keys.
enum DiagramInk { wire, input, output, muted }
enum CircuitPart { resistor, capacitor, inductor, voltageSource, currentSource, ground, diode, opamp, nmos, bjt, switchContact, block, circle }

class CalculatorDiagram {
  const CalculatorDiagram({required this.title, required this.caption, required this.marks});
  final String title;
  final String caption;
  final List<DiagramMark> marks;
}

sealed class DiagramMark {
  const DiagramMark();
}

class Wire extends DiagramMark {
  const Wire(this.points, {this.arrow = false, this.dashed = false, this.ink = DiagramInk.wire});
  /// x,y pairs. Corners are literal; crossing wires connect only with a Dot.
  final List<double> points;
  final bool arrow, dashed;
  final DiagramInk ink;
}

class Label extends DiagramMark {
  const Label(this.x, this.y, this.text, {this.quantity, this.ink = DiagramInk.wire, this.size = 14});
  /// Top-left anchor; keep text within the logical canvas and clear of wires.
  final double x, y, size;
  final String text;
  final String? quantity;
  final DiagramInk ink;
}

class Part extends DiagramMark {
  const Part(this.kind, this.x, this.y, {this.width = 60, this.height = 36, this.turns = 0, this.ink = DiagramInk.wire});
  /// Center anchor. Two-terminal horizontal symbols connect at x ± width/2.
  /// turns is clockwise quarter-turns; labels must be separate upright marks.
  /// Diode conducts left (anode) to right (cathode). Ground pin is top center.
  /// Opamp: triangle points right; + at (x-width/2,y-height/4),
  /// − at (x-width/2,y+height/4), output at (x+width/2,y).
  /// NMOS: gate (x-width/2,y), drain (x,y-height/2), source (x,y+height/2).
  /// BJT: base (x-width/2,y), collector (x,y-height/2), emitter
  /// (x,y+height/2), NPN arrow outward. Label terminals when sign matters.
  final CircuitPart kind;
  final double x, y, width, height;
  final int turns;
  final DiagramInk ink;
}

class Dot extends DiagramMark {
  const Dot(this.x, this.y, {this.ink = DiagramInk.wire});
  final double x, y;
  final DiagramInk ink;
}
