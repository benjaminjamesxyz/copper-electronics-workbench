# Architecture and pipeline

## Product
A local electronics design workbench. Search/filter the complete researched catalog, select a calculator, edit labeled numerical inputs with units, calculate with Rust, inspect all output quantities, readable formulas, model assumptions, source links, and independently worked examples. Levels: basic, intermediate, expert. Desktop targets only: Linux, macOS, Windows. No mobile or browser targets.

## Rust engine
`backend` embeds all `backend/catalog/*.json` at compile time. Formula expressions are parsed once at startup; definitions retain source metadata. The same generic numerical engine implements every catalog feature, including sequential derived outputs and cross-input model constraints. Inputs must exactly match the definition; nonfinite values, invalid ranges, invalid model domains, unknown calculators and malformed requests produce explicit errors. No floating-point failure is coerced into a plausible result.

JSON-lines protocol on stdin/stdout:
- `{"command":"catalog"}` → `{"ok":true,"calculations":[...]}`.
- `{"command":"calculate","id":"domain.slug","inputs":{"key":1.0}}` → `{"ok":true,"id":"domain.slug","outputs":{"key":2.0}}` or `{"ok":false,"error":"..."}`.
- One response per line; diagnostics only on stderr. Runtime is fully offline; no network listener or remote service.

## Flutter client
A managed child process launches the Rust executable without a shell. Requests are serialized, errors surfaced, process streams drained, process disposed when the client closes. Packaged builds carry the native executable beside the Flutter binary (inside `Contents/MacOS` on macOS). Development may specify `PCB_CALCULATOR_BACKEND` as an explicit binary path. The frontend derives all forms, outputs, reference text and example controls from the backend catalog; no duplicate numerical implementation.

Desktop composition: searchable domain/level catalog on the left; input workbench centrally; result cards and formula/assumption/source detail in the selected calculator panel. Scrollable and keyboard-operable. Finite scientific notation accepted; units explicit rather than implicit conversions. Edited inputs invalidate displayed results.

## Git pipeline
Root branch `integration/electronics-calculators` is integration-owner-only. Research and architecture are committed before worktree fanout. Sixteen developer branches `feat/<lane>` use isolated worktrees and own only their domain catalog and feature documentation, with one commit per completed calculator. Parent cherry-picks explicit commit lists into integration; no shared branch switching, force pushes, or remote assumptions.

After all development commits integrate, sixteen tester branches `test/<lane>` add independent numerical and boundary regression tests. Sixteen reviewers run in parallel with testers, read integration files, and record evidence-backed findings. Thirty-two simultaneous agents is the runtime ceiling; dependency gates prevent researching/testing unfinished definitions. All agents use root `COMMUNICATION.md` as the shared Markdown bus. Global builds, tests and desktop smoke runs are integration-owner responsibilities, not concurrently executed by workers.

## Acceptance gate
All 128 reserved features are implemented, all independent example vectors agree within their declared tolerance, all behavioral/boundary tests pass, all actionable review findings resolve, and the desktop app is actually launched and exercised. Availability/build results for each desktop OS must be reported honestly; Linux verification does not imply Windows/macOS verification. Calculators are engineering estimates within their documented model limits, not certification or substitutes for simulation/manufacturer specifications.
