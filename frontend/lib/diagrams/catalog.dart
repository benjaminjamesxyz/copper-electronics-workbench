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
import 'sensors.dart';
import 'opto.dart';
import 'oscillators.dart';
import 'rf.dart';
import 'batteries.dart';
import 'protection.dart';
import 'converters.dart';
import 'control.dart';
import 'emc.dart';
import 'cables.dart';
import 'precision.dart';
import 'logic.dart';
import 'motors.dart';
import 'passives.dart';
import 'cores.dart';
import 'measurement.dart';

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
  ...sensorsDiagrams,
  ...optoDiagrams,
  ...oscillatorsDiagrams,
  ...rfDiagrams,
  ...batteriesDiagrams,
  ...protectionDiagrams,
  ...convertersDiagrams,
  ...controlDiagrams,
  ...emcDiagrams,
  ...cablesDiagrams,
  ...precisionDiagrams,
  ...logicDiagrams,
  ...motorsDiagrams,
  ...passivesDiagrams,
  ...coresDiagrams,
  ...measurementDiagrams,
};
