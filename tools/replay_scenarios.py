#!/usr/bin/env python3
"""Replay every expansion lane receipt's ordered process scenarios."""
import json, subprocess, sys
from pathlib import Path
root = Path(__file__).resolve().parent.parent
lanes = ['sensors','opto','rf','batteries','protection','converters','control','emc','cables','precision','logic','motors','passives','cores','measurement','oscillators']
total=ok=err=mism=0; failures=[]
for lane in lanes:
    p = root/f'coordination/expansion/testing/{lane}.json'
    if not p.exists(): print('missing', p); continue
    receipt = json.load(open(p))
    for sc in receipt.get('parent_json_lines_scenarios', []):
        steps = sc.get('sequence') or [sc]
        for step in steps:
            total += 1
            r = subprocess.run([str(root/'backend/target/release/pcb-calculator')], input=json.dumps(step['request'])+'\n', text=True, capture_output=True)
            resp = json.loads(r.stdout)
            exp_ok = step.get('expected_ok', True)
            if exp_ok != resp.get('ok', False):
                mism += 1; failures.append((lane, step.get('name','?'), 'ok-mismatch', resp.get('error','')[:60])); continue
            if not exp_ok: err += 1; continue
            ok += 1
            abstol = step.get('absolute_tolerances', {})
            for k, v in step.get('expected_outputs', {}).items():
                a = resp['outputs'][k]; tol = step.get('relative_tolerance', 1e-6)
                if abs(a-v) > max(abs(v)*tol, abstol.get(k, 0.0)):
                    mism += 1; failures.append((lane, step.get('name','?'), k, a, v))
print(f'scenarios={total} ok={ok} rejected-as-expected={err} mismatches={mism}')
for f in failures: print(f)
sys.exit(1 if mism else 0)
