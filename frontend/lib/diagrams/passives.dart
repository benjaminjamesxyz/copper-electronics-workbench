import 'primitives.dart';

const passivesDiagrams = <String, CalculatorDiagram>{
  'passives.mlcc_logarithmic_aging': CalculatorDiagram(
    title: 'Ferroelectric relaxation after deaging',
    caption: 'Conceptual semilog aging law, not a live-scaled plot. Heating above '
        'the Curie point resets the clock. Fixed temperature and DC bias; '
        'aging is relative to the one-hour value, not compounded.',
    marks: [
      Wire([70, 50, 70, 205, 520, 205], arrow: true),
      Label(35, 42, 'C'),
      Wire([85, 85, 485, 155], ink: DiagramInk.output),
      Dot(85, 85),
      Dot(485, 155, ink: DiagramInk.output),
      Wire([85, 85, 85, 205], dashed: true, ink: DiagramInk.muted),
      Wire([485, 155, 485, 205], dashed: true, ink: DiagramInk.muted),
      Label(95, 57, 'C₁h', quantity: 'capacitance_one_hour'),
      Label(390, 125, 'C(t)', quantity: 'aged_capacitance', ink: DiagramInk.output),
      Label(78, 215, '1 h'),
      Label(475, 215, 't', quantity: 'age'),
      Label(185, 238, 'Time since deaging (log scale)'),
      Label(185, 160, 'A: % of C₁h / decade', quantity: 'aging_percent_decade'),
      Part(CircuitPart.capacitor, 590, 90, width: 60, height: 36),
      Wire([545, 90, 560, 90]),
      Wire([620, 90, 640, 90]),
      Label(541, 135, 'Ferroelectric'),
      Label(553, 158, 'dielectric'),
    ],
  ),
  'passives.dielectric_absorption_recovery': CalculatorDiagram(
    title: 'Hidden polarization survives a terminal short',
    caption: 'After finite charging and shorting, the open terminal recovers '
        'voltage from retained polarization. One fitted absorption branch; '
        'no leakage or meter loading. Va0 is the hidden voltage at release, '
        'not its later value. Both capacitors begin uncharged.',
    marks: [
      Wire([120, 65, 400, 65, 400, 80]),
      Wire([120, 220, 400, 220, 400, 210]),
      Dot(120, 65),
      Dot(120, 220),
      Dot(230, 65),
      Dot(230, 220),
      Wire([230, 65, 230, 110]),
      Part(CircuitPart.capacitor, 230, 145, width: 70, turns: 1),
      Wire([230, 180, 230, 220]),
      Part(CircuitPart.resistor, 400, 105, width: 50, turns: 1),
      Wire([400, 130, 400, 160]),
      Dot(400, 145),
      Part(CircuitPart.capacitor, 400, 185, width: 50, turns: 1),
      Label(75, 37, 'A (+) open'),
      Label(72, 197, 'B (ref)'),
      Label(35, 132, 'Vrec', quantity: 'recovered_voltage', ink: DiagramInk.output),
      Label(256, 132, 'Cm', quantity: 'main_capacitance'),
      Label(435, 94, 'Ra', quantity: 'absorption_resistance'),
      Label(435, 171, 'Ca', quantity: 'absorption_capacitance'),
      Label(445, 132, 'Va0 at release', quantity: 'memory_voltage', ink: DiagramInk.output),
      Label(35, 245, 'History: charge (Vch, tch) → short (ts) → open (t = 0 … trec)'),
    ],
  ),
};
