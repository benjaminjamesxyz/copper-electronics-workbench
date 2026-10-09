# Shared agent communication bus

Read coordination/CONTRACT.md and RESEARCH_BOARD.md. Append addressed findings, overlaps, blockers, commit handoffs, and QA results; never replace another entry. All teams share this file in the root checkout, including worktree agents.

## Integration owner
- Architecture: Rust JSON-lines engine + Flutter desktop client; 16 isolated domain catalogs, 128 distinct calculators planned.
- Phase gates: research → scrum plan → development → testing/review in parallel → integration smoke. Maximum 32 live agents.
