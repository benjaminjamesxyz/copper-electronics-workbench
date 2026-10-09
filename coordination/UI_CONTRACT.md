# Desktop workbench design contract

User/job: electronics hardware designers calculating component/network/interconnect quantities from explicit design inputs. First-read object: selected calculation and its model level; primary action: calculate with Rust. Domain objects: calculator, input quantity/unit/range, output quantity/unit, analytical formula, model assumptions, independent example, reference.

Density: compact searchable catalog; balanced input form; quantitative result with stronger weight than metadata. No dashboard metrics, decorative charts, hero cards or generic onboarding. Color separates navigation, editable inputs, computed outputs and invalid states. Monospace formula treatment is reserved for mathematics. Engineering notation accepted with explicit units; no silent unit conversion.

Interaction model: catalog → input form → results → model evidence. Search, domain and experience filters preserve discoverability. Loading, engine-not-found, finite-number validation, model-constraint errors, empty search, selection and busy states have explicit representations. Editing any input must clear previous outputs; switching calculations must not display late results from the previous selection.

Comparable task patterns: LTspice separates schematic parameters from simulation results; TI analog design calculators place parameter inputs beside computed component values; KiCad's calculator organizes engineering tools by domain. These interaction lessons, not copied visual designs, inform the catalog/form/result hierarchy.

Responsive priority: on wide desktops the catalog stays alongside the workbench; at narrow desktop widths it moves above the scrollable workbench. Input columns reduce to one when room is limited. Source URLs and formulas remain selectable. All actions use native Flutter focusable controls.

Finish evidence: actual Linux application launch, independent example interaction at basic/intermediate/expert levels, invalid input, stale-output clearing, empty search, selection and narrow-window inspection. Windows and macOS runners exist but are not verified by Linux evidence.
