import 'primitives.dart';
import 'dc.dart';
import 'ac.dart';
import 'transients.dart';
import 'diodes.dart';
import 'bjt.dart';
import 'mosfet.dart';
import 'opamps.dart';
import 'filters.dart';
import 'power.dart';
import 'magnetics.dart';
import 'signals.dart';
import 'pcb.dart';
import 'thermal.dart';
import 'data.dart';
import 'noise.dart';
import 'timing.dart';

const calculatorDiagrams = <String, CalculatorDiagram>{
  ...dcDiagrams,
  ...acDiagrams,
  ...transientsDiagrams,
  ...diodesDiagrams,
  ...bjtDiagrams,
  ...mosfetDiagrams,
  ...opampsDiagrams,
  ...filtersDiagrams,
  ...powerDiagrams,
  ...magneticsDiagrams,
  ...signalsDiagrams,
  ...pcbDiagrams,
  ...thermalDiagrams,
  ...dataDiagrams,
  ...noiseDiagrams,
  ...timingDiagrams,
};
