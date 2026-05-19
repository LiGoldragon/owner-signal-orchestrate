# owner-signal-persona-orchestrate — architecture

*OwnerSignal contract for privileged `persona-orchestrate` role and
repository administration.*

## 0 · TL;DR

`owner-signal-persona-orchestrate` is the owner-only Signal surface for
mutating orchestration topology. Ordinary role claims, releases,
handoffs, observations, and activity records stay in
`signal-persona-orchestrate`.

This split is code-enforced now and filesystem-permission-enforced
later: callers can compile against the ordinary contract without being
able to express role creation or repository-index refresh orders.

## MUST IMPLEMENT — signal architecture migration

This contract is pending the signal architecture migration named in
`primary/reports/designer/238-signal-architecture-redirection-contract-local-verbs.md`
and implemented by
`primary/reports/designer/239-signal-architecture-migration-plan.md`.
The current owner requests still sit under public `SignalVerb` roots;
that shape is temporary.

Required refactor after `signal-frame` and the updated
`signal_channel!` macro are available:

- replace the `signal-core` dependency with `signal-frame`;
- drop the `Mutate` / `Retract` prefixes from owner request variants;
- expose contract-local operation roots in verb form;
- keep owner-only authority on the owner socket;
- move verb-to-Sema lowering into the `persona-orchestrate` runtime
  executor.

The expected owner operation roots are `Create`, `Retire`, and
`Refresh`. The payloads remain nouns: role creation order, role
retirement order, and repository-index refresh order. The lower Sema
effects remain runtime work, not contract declarations.

**Note to remover:** when the refactor lands, remove this section and
add a `## Migration history — contract-local verbs (2026-05-XX)`
paragraph noting the shape change.

## 1 · Contract Surface

| Request | Signal verb | Meaning |
|---|---|---|
| `CreateRoleOrder` | `Mutate` | Create a dynamic role lane with its harness metadata. |
| `RetireRoleOrder` | `Retract` | Retire a dynamic role from the active registry. |
| `RefreshRepositoryIndexOrder` | `Mutate` | Re-scan local checkouts and refresh the orchestration repository index. |

| Reply | Meaning |
|---|---|
| `RoleCreated` | The daemon created the role record and report-lane paths. |
| `RoleRetired` | The daemon retired the role record. |
| `RoleCreationRejected` | The create order was valid but conflicts with existing state. |
| `RepositoryIndexRefreshed` | The local repository index was refreshed. |
| `OwnerOrchestrateRequestUnimplemented` | The request is part of the owner vocabulary but not implemented by the current runtime. |

## 2 · Shared Nouns

This crate imports role and path nouns from
`signal-persona-orchestrate`:

- `RoleIdentifier`
- `RoleName` compatibility alias
- `HarnessKind`
- `WirePath`

It does not duplicate ordinary claim, release, handoff, activity, or
scope records.

## 3 · Constraints

| Constraint | Witness |
|---|---|
| Topology-changing orders live only in the owner contract. | Ordinary `signal-persona-orchestrate::OrchestrateRequest` has no `CreateRoleOrder`, `RetireRoleOrder`, or `RefreshRepositoryIndexOrder` variants; this crate round-trips all owner variants. |
| Every owner request declares a Signal root verb. | `OwnerOrchestrateRequest::signal_verb()` witnesses `Mutate`, `Retract`, and `Mutate`. |
| Contract code contains no runtime. | Source contains no Kameo, Tokio, sema-engine, redb, filesystem mutation, GitHub, or ghq implementation. |
| Harness assignment is typed, not hidden in a role string. | `CreateRoleOrder` carries `HarnessKind` beside `RoleIdentifier`. |

## 4 · Non-Ownership

- No `persona-orchestrate` daemon.
- No role registry table.
- No claim table.
- No report repository creation.
- No workspace symlink writing.
- No CLI argv parsing.
- No filesystem permission enforcement.

## Code Map

```text
src/lib.rs            owner request/reply records and signal_channel! invocation
tests/round_trip.rs   frame round trips and verb witnesses
```

## See Also

- `../signal-persona-orchestrate/ARCHITECTURE.md`
- `../persona-orchestrate/ARCHITECTURE.md`
- `~/primary/skills/contract-repo.md`
