# Tenancy, Teams, and Organizations

Anubis adopts Bullet Train's multi-tenancy model ("teams should be an MVP feature") and extends it with two structures that sit **beside each other** rather than nesting: an Organization holds Teams, which are groups of people carrying roles, and SubTenants, which are containers the work lives in.

Which way round they are is the whole of the model. A team says *who may act and with which roles*. A sub-tenant says *which container a resource sits in*. Neither is required to know about the other, and the only thing connecting them is which sub-tenants a team reaches.

## Entity model

```
User ─< OrganizationMembership >─ Organization
                                    │    │
                                    │    └─< SubTenant >─ TeamSubTenant ─┐
                                    │             │                      │
User ─< SubTenantMembership >───────┼─────────────┘                      │
                                    │                                    │
User ─< TeamMembership >─ Team ─────┴────────────────────────────────────┘
                            (sub_tenant_scope: organization | explicit)
```

- **User**: a person who can log in. Owns credentials, profile, and preferences. Owns nothing domain-related directly.
- **Organization**: the top-level tenant. The billing and policy umbrella. It owns people and policy, not work.
- **SubTenant**: a container the work lives in. GCP calls it a project, Jira calls it a project, GitHub calls it a repository, Linear calls it a workspace; the shape is the same every time. It belongs to exactly one Organization and holds its own membership. Optional in the strongest sense: an organization may have none, and a resource that belongs to no project belongs to the Organization.
- **Team**: a group of people carrying role keys. It can also own resources of its own, the ones that belong to a group rather than to a project: communications, file artifacts, anything a team keeps.
- **TeamSubTenant**: one grant of one team's reach into one sub-tenant. Read only for a team scoped `explicit`; an organization-scoped team reaches every sub-tenant without a row here.
- **OrganizationMembership**: joins a User to an Organization, carrying org-level roles (org admin, billing), an `access` level, and a `suspended_at` marker.
- **SubTenantMembership**: joins a User to a SubTenant, carrying sub-tenant-level roles and its own `suspended_at`. This is the explicit grant a guest reaches a sub-tenant through, and the row an administrator suspends to cut somebody out of one project without touching their standing anywhere else.
- **TeamMembership**: joins a User to a Team, carrying team-level roles. Domain resources are assigned to TeamMemberships, never directly to Users. This allows assigning work to invited people who have not signed up yet, and keeps assignments intact when a user leaves.
- **Invitation**: created when someone is added to a Team or Organization by email. The emailed 256-bit token (hashed at rest, 14-day expiry) is the credential; whichever signed-in account holds it may claim, and claiming consumes the invitation. Team invitations pre-create the unclaimed TeamMembership, so the membership (id, roles, and any resource assignments) survives the claim intact; organization invitations create the OrganizationMembership at claim time. Re-inviting an email replaces the pending invitation. Inviting requires the admin role on the target, and organization admins may invite to any team in their organization. An admin can revoke a pending invitation, which discards the unclaimed membership with it; a claimed invitation no longer exists, so a claim cannot be taken back.
- **Role**: declared in `roles.yml`, granted through memberships at any of the three levels.

At signup, every user gets a personal Organization containing a default Team ("General"), so solo use requires zero tenancy ceremony. **No sub-tenant is created**: the tier is opt-in, and a project nobody asked for would be a row that shows up in every picker and means nothing. The UI reveals organization complexity only when the user opts into it.

### A worked example

An organization, Jalapeno Labs, with four teams and two projects:

| Team | Scope | Reaches |
|---|---|---|
| Leadership | `organization` | Game One, Game Two, and every project made later |
| Dev | `explicit` | Game One, Game Two |
| QA | `explicit` | Game One |
| Marketing | `explicit` | nothing, so only its own work and the organization's |

Two questions are answered in two places, and keeping them apart is what makes the model work. **Tenancy** decides which projects a team reaches. **`roles.yml`** decides which models and actions a role grants. Marketing not seeing the dev team's designs is the second question, not the first: reaching Game One says nothing about being allowed to read a `Design` in it.

A person belongs to as many teams as they need, and holds the union of every role those teams carry, plus anything granted to them directly at the organization or the project.

## Sub-tenants, and how a team reaches them

The tier is optional: an application that never surfaces it never sees it, and an ownership chain that ends in `Team` keeps working exactly as it did. What the tier adds is a place for applications whose domain is organized around projects, repositories, or workspaces, where mapping that concept onto a Team collapses the structure the product is built around.

A Team's `sub_tenant_scope` is the whole of its relationship to the tier, and the two states are GitHub's split between a team with organization-wide repository access and one granted repositories by name. The choice is about what happens to projects created **later**:

- **`organization`** reaches every sub-tenant in the organization, the ones created after the team included. This is the state a team is created in, so a team is never born reaching nothing.
- **`explicit`** reaches exactly the sub-tenants named in `team_sub_tenants`, and never picks up a new one on its own. That set may be empty, which is legitimate: a marketing team whose work is all its own needs no project at all.

The relationship is many-to-many because both facts are ordinary: one team works on several projects, and one project is worked on by several teams. Both of `team_sub_tenants`'s foreign keys are composite over the row's own `organization_id`, so a grant spanning two organizations is unrepresentable in the schema rather than merely refused by the queries that read it.

### Access resolution

`anubis::tenancy::resolve_sub_tenant_access` is the one function that answers who reaches a sub-tenant and with which roles. Nothing re-derives it: two call sites working the rules out for themselves would eventually disagree about who may read something, and a disagreement between two authorization paths is a data leak.

**A suspension is a deny.** It is checked before any grant and outranks every one of them, the organization administrator's bypass included, because a deny an admin bit silently ignored would not be a deny. A suspended organization membership cuts the member out of everything in the organization, the team routes included. A suspended sub-tenant membership cuts them out of **that sub-tenant only**, and leaves their teams alone: the two structures are orthogonal, so cutting somebody out of a project says nothing about the group of people they belong to, whose own work is not in that project.

Otherwise a grant applies when any of these hold, and the caller's resolved roles are the **union** of every grant that applies. Permission is monotone in the role set (`RoleSet::can` asks whether *any* held role grants the action), so the union is exactly the strongest standing the caller holds across the tiers:

| Path | Rule |
|---|---|
| Organization admin | The `admin` role bypasses the tier and reaches every sub-tenant in the organization |
| Full organization member | `access = full` cascades into every sub-tenant |
| Guest | `access = guest` cascades into nothing, and reaches only what was granted by name |
| Sub-tenant membership | An explicit grant, applying to full members and guests alike, since it can only add |
| Team | An `organization`-scoped team reaches every sub-tenant; an `explicit` one reaches the ones it was granted |

Two consequences are worth stating rather than leaving to be discovered. An organization membership marked `guest` that also holds `admin` is a contradiction an administrator can write, and admin wins. And a team membership grants reach whatever the member's organization access says, because putting somebody on a team is itself the explicit act: an administrator confining a guest to one project scopes their team to it rather than leaving it organization-wide.

The `SubTenantMember` guard is the first caller. It resolves the route's `{sub_tenant_id}` and answers `404` for a sub-tenant that does not exist and for one the caller cannot reach, byte-identical, so probing ids reveals nothing.

`anubis::tenancy::list_reachable_sub_tenants` answers the same question for a whole organization, and is what the listing endpoint serves. It applies the same rules through the same function rather than restating them, gathering every grant once instead of once per project: a listing that admitted one project more than the guard does would be a leak, and one that admitted fewer would be a screen nobody can explain.

### Not built yet

The schema, the model, both resolution functions, the guard, and the management endpoints ship. Still to come, tracked on [Anubis #93](https://github.com/JalapenoLabs/Anubis/issues/93):

- **The ownership roots.** A scaffolded model chains to a `Team` and nothing else yet. The model this page describes needs three: `Team` as today, `SubTenant` for project work, and `Organization` for work that belongs to everyone the roles admit. Each is a living-template family rather than a flag, for the reason three-level ownership was: a name-for-name transform of a two-table query never produces a differently-rooted one. This is the largest remaining piece and the one that makes the tier usable by an application.
- **`resolve_organization_access`**, the reach question for an organization-owned resource, which is deliberately *not* the `OrganizationMember` guard. Administering a tenant needs a membership in it; reaching a resource it owns needs only a path to it, which a team-only member has. Collapsing the two would let a team admin reach `DELETE /tenancy/organizations/{id}`.
- **Roster and suspension endpoints** for a sub-tenant: enrolling a guest by name, changing their project-level roles, and setting `suspended_at` on either membership. Until they exist, an application writes those columns itself. The endpoint that suspends must take the same organization lock the other membership changes take, so the last-admin invariant covers it.
- The membership overview (`GET /tenancy/memberships`) does not yet group teams by sub-tenant.
- Screens for all of the above.

## Ownership chain

Every scaffolded model declares its parent chain back to a Team, exactly like Bullet Train:

```
anubis scaffold model Goal Project,Team description:text_field
```

The chain drives everything: authorization scoping, nested routes, breadcrumbs, and the parent's show-view table. Tenant isolation is enforced by walking the chain, never by trusting a client-supplied id.

Enforcement is extractor-based. A handler that takes `anubis::guard::TeamMember` (or `OrganizationMember`) gets, before its body runs: authentication (401), the route's `{team_id}` resolved against the caller's membership (404 for non-members, byte-identical to a nonexistent id, so probing reveals nothing), and permission checks via `member.require(Action::Update, "Project")` against the compiled role set (403). Routers provide the needed request extensions with `anubis::guard::layer(pool, roles)`. Scaffolded models resolve their parent chain to the owning team and ride the same primitives.

Selectable associations are scoped through generated `valid_*` methods on the model. `anubis scaffold field Project lead_id:super_select{class_name=TeamMembership}` writes `Project::valid_leads`, and these methods populate select fields and enforce the tenancy boundary on write, so a form can never smuggle in another tenant's record. See [scaffolding.md](scaffolding.md#belongs_to-one-record-one-foreign-key).

**Assign to a membership, never to a user.** That is Bullet Train's advice and it is the framework's: a membership exists from the moment somebody is invited, so a record can be assigned to a teammate who has not signed up yet, and it disappears with them when they leave. The framework owns the roster half of that, because `team_memberships` is a framework table an application crate cannot reach through its own Diesel query graph: `TeamMembership::valid_for_team` returns the team's memberships as `anubis::http::FieldOption` values, ordered by the label a person reads, and `TeamMembership::labels_for` reads the labels of a page of assignments in one query. A label is the member's name, falling back to their account email, then to the address their invitation was sent to.

## Roles and permissions

Roles are declared once, in `config/roles.yml`, with role inheritance (`admin` includes `editor` and `billing`) and per-model action grants (`read`, `create`, `update`, `destroy`, or `manage` as shorthand for all four), modeled on `bullet_train-roles`. The starter ships the baseline vocabulary: `default`, `editor`, `billing`, and `admin`.

A role may also name `scopes`, the tenancy tiers it can be granted at, out of `organization`, `sub_tenant`, and `team`. Omitting it means every tier, which is what a role written before the sub-tenant tier existed keeps meaning; the starter scopes `billing` to `organization`, because subscriptions attach to the organization and the role means nothing anywhere else. Scopes say where a role key attaches, never what it grants, so they do not travel through `includes`. `RoleSet::is_grantable_at` is the backend's question and `isGrantableAt` is the SPA's.

One definition drives both sides of the stack:

1. The backend embeds the file at compile time (`include_str!`) and resolves it at boot through `anubis::roles::RoleSet`, which rejects unknown includes, inheritance cycles, and unknown actions before the server takes traffic. Authorization asks `RoleSet::can(held_roles, action, model)`.
2. `anubis roles generate-ts` emits the TypeScript permissions module (`roles.generated.ts`) the SPA uses to hide or disable controls the current member cannot use. Output is deterministic, and CI regenerates it and fails on drift.

`anubis roles check` validates the file standalone.

## Managing tenants

Tenancy is manageable from the API, not only at signup. The routes mount under `/tenancy` alongside the membership overview and the invitation endpoints:

| Route | Who | Effect |
|---|---|---|
| `POST /tenancy/organizations` | any signed-in user | Create an organization with its default team; the creator administers both |
| `PATCH /tenancy/organizations/{organization_id}` | org admin | Rename the organization |
| `DELETE /tenancy/organizations/{organization_id}` | org admin | Delete the organization, its teams, and their records |
| `POST /tenancy/organizations/{organization_id}/teams` | org admin | Create a team, with the creator as its admin member |
| `DELETE /tenancy/organizations/{organization_id}/teams/{team_id}` | org admin | Delete a team and its records |
| `PUT /tenancy/organizations/{organization_id}/teams/{team_id}/sub-tenants` | org admin | Replace which sub-tenants the team reaches |
| `GET /tenancy/organizations/{organization_id}/sub-tenants` | org member | The sub-tenants the caller reaches, with their roles |
| `POST /tenancy/organizations/{organization_id}/sub-tenants` | org admin | Create a sub-tenant |
| `PATCH /tenancy/organizations/{organization_id}/sub-tenants/{sub_tenant_id}` | org admin | Rename a sub-tenant |
| `DELETE /tenancy/organizations/{organization_id}/sub-tenants/{sub_tenant_id}` | org admin | Delete a sub-tenant and its records |
| `GET /tenancy/organizations/{organization_id}/members` | org member | The organization roster, outstanding invitations included |
| `DELETE /tenancy/organizations/{organization_id}/members/{membership_id}` | org admin | Remove another organization member |
| `POST /tenancy/organizations/{organization_id}/leave` | org member | Leave the organization |
| `DELETE /tenancy/organizations/{organization_id}/invitations/{invitation_id}` | org admin | Revoke any pending invitation in the organization |
| `PATCH /tenancy/teams/{team_id}` | team admin | Rename the team |
| `PATCH /tenancy/teams/{team_id}/members/{membership_id}` | team admin | Replace a member's roles |
| `DELETE /tenancy/teams/{team_id}/members/{membership_id}` | team admin | Remove another member |
| `POST /tenancy/teams/{team_id}/leave` | team member | Leave the team |
| `DELETE /tenancy/teams/{team_id}/invitations/{invitation_id}` | team admin | Revoke a pending team invitation |

Team-scoped routes take the `TeamMember` guard, organization-scoped routes take `OrganizationMember`, and sub-tenant-scoped routes take `SubTenantMember`, so the discipline is the one every ownership chain follows: `401` when signed out, `404` when the caller is not a member (byte-identical to a nonexistent id), and `403` when a member lacks the role. Dissolving a team is an organization act rather than a team act, which is why deletion is nested under the organization: a team's own admins run the team, and the organization decides whether the team exists.

Roles are replaced wholesale rather than patched, so a request states the end state and two admins editing the same member cannot interleave into a set neither asked for. Every requested key is checked against the compiled `roles.yml`; an unknown key is a `400`, and an empty list means the baseline `default` role.

A team's reach is replaced the same way and for the same reason. `PUT .../teams/{team_id}/sub-tenants` takes `{"scope": "organization"}` or `{"scope": "explicit", "sub_tenant_ids": [...]}`; a list beside the organization scope is a `400` rather than a silent discard, since an organization-wide team already reaches every project and a list would be a second answer to a settled question. A project named twice is granted once. Both sub-tenant management and team reach are **organization-level acts**: letting a team's own admin grant it a project would let a team widen its own reach, and naming a project is administering the organization's structure. Deleting a project leaves every team standing and simply reaches one fewer, because a project closing does not dissolve the group of people who worked on it.

A tenant's name is one to a hundred characters and carries no control characters. The length is a display bound; the control characters are refused because the name is rendered into an invitation's subject line, where a line break is at best a display bug and at worst an attempt at a header of the caller's own. An invited email address goes through the same validation registration uses, for the same reason. The mail layer encodes whatever it is given, so both are the second lock rather than the only one.

Inviting sends mail to an address the request names, and re-inviting is allowed, so the endpoint charges the same per-recipient inbox budget a password reset charges: five an hour per address, across every surface, since the application builds one limiter and passes it to both routers. The charge lands after the inviter is shown to administer the target, so no signed-in account can spend a stranger's budget. See [rate limiting](api.md#rate-limiting).

The two rosters answer the two levels. A team roster row is a TeamMembership, so an invited person already holds one and carries the `invitation_id` an admin revokes. An organization roster row is either an OrganizationMembership or an invitation that has not created one yet, which is why its `membership_id` is null exactly when `pending` is true. Team invitations belong to their team's roster rather than to the organization's, so each place is listed once.

Both levels are left the same way: `leave` releases the caller's own membership, and removing somebody else is an admin act with its own route, so a request can never mean both. Trying to remove yourself answers `400` and names the leave route. Leaving an organization releases the organization membership and nothing else; the teams inside it are separate memberships, left team by team, because a person working in one team without standing in its organization is a state the model already has (a team-only invitation produces exactly that).

### Invariants

**A team always keeps at least one claimed admin.** Demoting the last admin, or the last admin leaving, answers `409 Conflict`: the request is well formed and only the current state refuses it, which is exactly what `409` says. Unclaimed memberships never count as admins, because an invitation is not a person.

The rule is enforced after the change rather than before it, inside a transaction that first locks the tenant's own row. Both halves matter. Counting afterwards means one rule covers demotion, removal, and leaving alike, and the transaction rolls the change back when it fires. Locking the tenant means two admins acting at the same instant are ordered rather than each reading the other as the admin who remains and both committing, which is the one way a team could have been left with nobody able to administer it. The lock is on the team or organization row, not on the memberships, because two transactions each locking the other's target deadlock, and a deadlock is a `500` where a `409` belongs.

So removing another member normally cannot break the rule, since the caller is a claimed admin and is not the member being removed, and the one case that can reach the `409` is a caller whose own admin role was taken by a request that committed while theirs was in flight.

**An organization keeps an admin under the same rule**, enforced the same way. The last organization admin cannot leave, and gets the same `409`; every organization membership is claimed, so all of them count. The way out of a personal organization is therefore to promote someone or to delete it, not to walk out of it and leave it unreachable. Account deletion is the one path that cannot refuse, and it promotes instead (below).

**Nothing marks the organization created at signup as special.** Its admin may rename it or delete it like any other. Introducing a "personal" flag purely to forbid one deletion would add a permanent concept to the model in order to protect a state that a user rebuilds with a single `POST /tenancy/organizations`. A user with no organization is a valid, recoverable state; a schema concept nobody else needs is not.

### Cascade semantics

Deletion cascades, deliberately. The framework's migrations declare `ON DELETE CASCADE` from `organizations` to sub-tenants, teams, memberships, and invitations; from `sub_tenants` to their memberships and to the teams scoped to them; and from `teams` to memberships, invitations, and platform applications. The scaffolder's living templates declare the same on the ownership chain: a team-owned table's `team_id` references `teams(id) ON DELETE CASCADE`, and a nested model cascades from its parent. So deleting a team deletes the records that chain to it, and deleting an organization deletes its teams first.

Cascade is the right default here because the ownership chain already means "this record exists inside that team". Restrict would force a caller to empty a team by hand through endpoints that may not exist yet, and orphaning is not an option since a record with no team has no tenant and therefore no reachable authorization. An application that deliberately declares a restricting reference of its own is respected rather than overridden: the delete answers `409 Conflict` telling the caller to remove those records first, instead of failing as a `500`.

### Account deletion

Deleting an account is terminal and must always succeed, so it settles what the user leaves behind instead of refusing. In the same transaction that removes the user, the framework:

1. Deletes the user's organization, sub-tenant, and team memberships.
2. Deletes every organization nobody can reach any more, meaning no organization memberships, no claimed team memberships, and no sub-tenant memberships remain anywhere in it. This is what keeps a personal organization from outliving its only member, and what stops a guest's project being deleted out from under them.
3. Keeps every surviving organization and team administrable. One that still has members but lost its last admin promotes its longest-standing remaining member. If a surviving organization has no organization memberships left at all, and lives on only through its teams or its sub-tenants, the longest-standing member of either gains the organization membership as its admin, teams first.

A surviving team with no members left is kept rather than deleted: it still owns application records, and an organization admin can delete it or invite people back into it. The invariant the interactive endpoints defend by refusing, this path defends by succeeding.

## Billing

Subscriptions attach to the Organization, not the User and not the Team. Plans live in `config/billing.yml`, an organization with no subscription is on the free plan, and Stripe is the system of record for money. See [billing.md](billing.md).

Tenancy meets billing in one place: **the `seats` limit is enforced when an invitation is created.** An invitation that would put the organization over its plan's seats answers `409` with a message naming the plan. A seat is a person counted once, by email address, across organization memberships, claimed team memberships, and invitations still claimable, so somebody in three teams pays for one seat and re-inviting an existing member costs nothing. The check runs inside the transaction that writes the invitation, under the same organization lock the last-admin rule takes, so two admins inviting at the same instant are ordered rather than each seeing room for one more.

Every membership change (invite, claim, revoke, remove, leave, delete a team) also queues the job that tells Stripe the new seat count, but only when a plan sells a per-seat price. An application with no `config/billing.yml` passes `None` to `anubis::tenancy::router` and has neither behavior.
