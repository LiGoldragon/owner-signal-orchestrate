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

## Migration history — contract-local verbs (2026-05-19)

This contract migrated from `signal-core` public `SignalVerb` wrappers
to `signal-frame` contract-local operation roots.

The public owner request surface is now:

- `Create(CreateRoleOrder)`
- `Retire(RetireRoleOrder)`
- `Refresh(RefreshRepositoryIndexOrder)`

There is no public `Mutate` / `Retract` tag in this contract. The
owner socket remains the authority boundary; `persona-orchestrate`
owns the typed Component Commands (Layer 2) that lower contract
operations to executable form, and projects them to payloadless Sema
class labels (Layer 3) for observation. See
`~/primary/skills/component-triad.md` §"Verbs come in three layers".

## 1 · Contract Surface (Layer 1)

| Operation | Projected Sema class | Meaning |
|---|---|---|
| `Create` | `Mutate` | Create a dynamic role lane with its harness metadata. |
| `Retire` | `Retract` | Retire a dynamic role from the active registry. |
| `Refresh` | `Mutate` | Re-scan local checkouts and refresh the orchestration repository index. |

| Reply | Meaning |
|---|---|
| `RoleCreated` | The daemon created the role record and report-lane paths. |
| `RoleRetired` | The daemon retired the role record. |
| `RoleCreationRejected` | The create order was valid but conflicts with existing state. |
| `RepositoryIndexRefreshed` | The local repository index was refreshed. |
| `PartialApplied` | One or more downstream mutation legs succeeded while one or more sibling legs failed; orchestrate records the divergence instead of rolling back. |
| `OwnerOrchestrateRequestUnimplemented` | The request is part of the owner vocabulary but not implemented by the current runtime. |

## 2 · Shared Nouns

This crate imports role and path nouns from
`signal-persona-orchestrate`:

- `RoleIdentifier`
- `RoleName` compatibility alias
- `HarnessKind`
- `PartialApplied` and its downstream success/failure records
- `WirePath`

It does not duplicate ordinary claim, release, handoff, activity, or
scope records.

## 3 · Constraints

| Constraint | Witness |
|---|---|
| Topology-changing orders live only in the owner contract. | Ordinary `signal-persona-orchestrate::OrchestrateRequest` has no `CreateRoleOrder`, `RetireRoleOrder`, or `RefreshRepositoryIndexOrder` variants; this crate round-trips all owner variants. |
| Every owner request has a contract-local operation root. | `OwnerOrchestrateRequest::operation_kind()` witnesses `Create`, `Retire`, and `Refresh`. |
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
tests/round_trip.rs   frame round trips and contract-local operation witnesses
```

## See Also

- `../signal-persona-orchestrate/ARCHITECTURE.md`
- `../persona-orchestrate/ARCHITECTURE.md`
- `../signal-frame/ARCHITECTURE.md`
- `../signal-sema/ARCHITECTURE.md`
- `~/primary/skills/contract-repo.md`
- `~/primary/skills/component-triad.md` §"Verbs come in three layers".
