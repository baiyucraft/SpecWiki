# AOCI Design Procedure

1. Read `research/aoci.md` and the latest complete Overview; confirm durable ownership, relations, and constraints remain aligned.
2. Map preserved or changed system semantics to explicit interfaces, ownership, failure, and rollback boundaries.
3. Record confirmed semantics, planned semantic delta, Maintain timing, and database conditions under `AOCI-derived semantic constraints`.
4. When AOCI conflicts with source/tests, retain both evidence sets and treat governance drift as a blocker instead of hand-editing formal cognition.
5. Keep current call/impact evidence in the separate CodeGraph-derived section; never merge both into an untraceable generic tool analysis.
