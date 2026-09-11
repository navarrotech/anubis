//! Multi-tenancy for Anubis applications.
//!
//! The model follows Bullet Train's, extended with two structures that sit
//! beside each other rather than nesting. Users join **teams** through team
//! memberships, teams belong to **organizations**, and users also hold
//! organization-level memberships for org-wide roles. The full design lives in
//! the repository's `docs/tenancy.md`.
//!
//! The organization also holds **sub-tenants**: what GCP calls a project and
//! GitHub calls a repository, a container the work lives in. The two are
//! orthogonal, and which way round they are is the whole of the model:
//!
//! - A **team** says who may act and with which roles. It is a group of
//!   people, and it can own resources of its own, the ones that belong to a
//!   group rather than to a project.
//!   [`SubTenantScope`] says which of the organization's sub-tenants it
//!   reaches: every one of them, or exactly the ones granted by name.
//! - A **sub-tenant** says which container a resource sits in. It is optional
//!   in the strongest sense: an organization may have none, and a resource
//!   that belongs to no project belongs to the organization, reachable by
//!   everyone the roles admit.
//!
//! [`resolve_sub_tenant_access`] is the one function that decides who reaches
//! a sub-tenant and with which roles, and [`list_reachable_sub_tenants`]
//! answers the same question for a whole organization through the same rules.
//! Nothing re-derives either.
//!
//! Every user gets a personal organization with a default team at signup, so
//! solo use needs no tenancy ceremony. Applications mount [`router`]
//! (conventionally under `/tenancy`) for the membership overview, invitations,
//! and the management endpoints that create, rename, and dissolve tenants.

mod access;
mod bootstrap;
mod departure;
mod invitation;
mod management;
mod model;
mod routes;

pub(crate) use bootstrap::create_personal_organization;
pub(crate) use departure::settle_departure;

#[doc(inline)]
pub use access::{Reach, SubTenantAccess, list_reachable_sub_tenants, resolve_sub_tenant_access};
#[doc(inline)]
pub use bootstrap::ADMIN_ROLE;
#[doc(inline)]
pub use invitation::{INVITATION_TTL_DAYS, Invitation};
#[doc(inline)]
pub use model::{
    Organization, OrganizationAccess, OrganizationMembership, SubTenant, SubTenantMembership,
    SubTenantScope, Team, TeamMembership, TeamSubTenant,
};
#[doc(inline)]
pub use routes::router;
