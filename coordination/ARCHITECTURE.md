# Architecture and pipeline

## Product
A local electronics design workbench. Search/filter the complete researched catalog, select a calculator, edit labeled numerical inputs with units, calculate with Rust, inspect all output quantities, readable formulas, model assumptions, source links, and independently worked examples. Levels: basic, intermediate, expert. Desktop targets only: Linux, macOS, Windows. No mobile or browser targets.

## Rust engine
`backend` embeds all `backend/catalog/*.json` at compile time. Formula expressions are parsed once at startup; definitions retain source metadata. The same generic numerical engine implements every catalog feature, including sequential derived outputs and cross-input model constraints. Inputs must exactly match the definition; nonfinite values, invalid ranges, invalid model domains, unknown calculators and malformed requests produce explicit errors. No floating-point failure is coerced into a plausible result.

`meval` supplies parsing and compiled RPN; `checked.rs` evaluates those tokens with the same registered functions and rejects nonfinite intermediate values before later operations conceal them. Each engine reuses a preallocated operand stack. Native conditioning primitives preserve analytic models at short times, near-equal values and extreme scales; derived dimensional outputs are not calculated from already rounded subnormal intermediates where that loses representable precision.

JSON-lines protocol on stdin/stdout:
- `{"command":"catalog"}` → `{"ok":true,"calculations":[...]}`.
- `{"command":"calculate","id":"domain.slug","inputs":{"key":1.0}}` → `{"ok":true,"id":"domain.slug","outputs":{"key":2.0}}` or `{"ok":false,"error":"..."}`.
- One response per line; diagnostics only on stderr. Runtime is fully offline; no network listener or remote service.

## Flutter client
A managed child process launches the Rust executable without a shell. Requests are serialized, errors surfaced, process streams drained, process disposed when the client closes. Packaged builds carry the native executable beside the Flutter binary (inside `Contents/MacOS` on macOS). Development may specify `PCB_CALCULATOR_BACKEND` as an explicit binary path. The frontend derives all forms, outputs, reference text and example controls from the backend catalog; no duplicate numerical implementation.

Desktop composition: searchable domain/level catalog on the left; input workbench centrally; result cards and formula/assumption/source detail in the selected calculator panel. Scrollable and keyboard-operable. Finite scientific notation accepted; units explicit rather than implicit conversions. Edited inputs invalidate displayed results.

Input/selection revisions suppress stale results; a distinct active-request token owns busy-state cleanup, including switching away and back during a pending request. Native integration tests explicitly register synthetic keyboard injection to prevent platform IME updates from overwriting test text; the calculation process and rendered desktop surface remain real.

Each calculator ID resolves to a static explanatory vector model in `frontend/lib/diagrams/catalog.dart`. Thirty-two lane files own 228 diagrams: sixteen original files with eight each, and sixteen expansion files with six or seven each. Shared primitives define connector positions, the painter draws native Flutter vectors, and the panel connects annotations to existing catalog quantity names and units. Captions describe topology, directions and limitations. Curves are illustrative, never substituted for numerical results. A fixed readable canvas scrolls horizontally on narrow windows and scales with desktop text settings; captions, legends and image semantics expose a text alternative. The Rust catalog/protocol does not carry presentation geometry.

## Git pipeline
Root branch `integration/electronics-calculators` is integration-owner-only. Research and architecture are committed before worktree fanout. Sixteen developer branches `feat/<lane>` use isolated worktrees and own only their domain catalog and feature documentation, with one commit per completed calculator. Parent cherry-picks explicit commit lists into integration; no shared branch switching, force pushes, or remote assumptions.

After all development commits integrate, sixteen tester branches `test/<lane>` add independent numerical and boundary regression tests. Sixteen reviewers run in parallel with testers, read integration files, and record evidence-backed findings. Thirty-two simultaneous agents is the runtime ceiling; dependency gates prevent researching/testing unfinished definitions. All agents use root `COMMUNICATION.md` as the shared Markdown bus. Global builds, tests and desktop smoke runs are integration-owner responsibilities, not concurrently executed by workers.

The illustration extension uses sixteen isolated `diagram/<lane>` author worktrees and independent source/pixel review lanes. Parent integrates explicit commits, owns shared rendering and regression checks, captures every actual native diagram, and closes all review findings. `coordination/diagram_reviews.json` retains findings, visual evidence attribution, replacement reviewer dispatch recovery and final lane dispositions.

The second calculation sprint repeats the research → scrum → development → independent assurance gates. Sixteen `feat/expand-<lane>` branches deliver 100 additional atomic numerical-model/diagram/documentation commits. Sixteen `test/expand-<lane>` branches and sixteen reviewers start at the same complete integrated snapshot. The expansion board/backlog reserve distinct physical problems against the actual 128-model baseline. Main records every real-process example, all-228 native traversal, every new GUI example and every new diagram image, then closes source and pixel findings before packaging. See `coordination/EXPANSION_CONTRACT.md` and `coordination/expansion/SPRINT.md`.

## Acceptance gate
All 228 reserved features must be implemented, all independent example vectors must agree within their declared tolerance, all behavioral/boundary tests must pass, all actionable review findings must resolve, and the desktop app must actually launch and be exercised. All 100 expansion features require distinct explanatory diagrams and independent source/pixel acceptance; the original 128 models and diagrams remain preserved. Availability/build results for each desktop OS must be reported honestly; Linux verification does not imply Windows/macOS verification. Calculators are engineering estimates within their documented model limits, not certification or substitutes for simulation/manufacturer specifications.
