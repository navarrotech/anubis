//! Creating organizations and teams, at signup and on demand.
//!
//! Registration bootstraps a personal organization so solo use needs no
//! tenancy ceremony; the management endpoints create further organizations
//! and teams through the same functions, so every organization in the system
//! looks alike and every creator ends up an admin of what they created.

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::auth::User;
use crate::schema::{
    organization_memberships, organizations, sub_tenants, team_memberships, teams,
};
use crate::tenancy::model::{
    NewOrganization, NewOrganizationMembership, NewSubTenant, NewTeam, NewTeamMembership,
    Organization, SubTenant, SubTenantScope, Team,
};

/// Role key granted to the creator of an organization or team.
///
/// The roles.yml compiler formalizes role definitions; until then this is the
/// one well-known key.
pub const ADMIN_ROLE: &str = "admin";

/// Role key granted when a caller asks for no roles at all.
///
/// The starter's `config/roles.yml` ships it as the read-only baseline; an
/// application that removes the key makes role-less requests fail validation,
/// which is the honest outcome.
pub(crate) const DEFAULT_ROLE: &str = "default";

/// Name of the team every new organization starts with.
const DEFAULT_TEAM_NAME: &str = "General";

/// Returns `true` when the held role keys include [`ADMIN_ROLE`].
pub(crate) fn holds_admin(roles: &[String]) -> bool {
    roles.iter().any(|role| role == ADMIN_ROLE)
}

/// Creates the personal organization, default team, and admin memberships.
///
/// Every user gets this at signup so solo use has zero tenancy ceremony: the
/// organization is named after the email's local part and holds one team. The
/// caller runs this inside the same transaction that inserts the user, so a
/// half-bootstrapped account can never exist.
pub(crate) async fn create_personal_organization(
    connection: &mut AsyncPgConnection,
    user: &User,
) -> Result<(Organization, Team), diesel::result::Error> {
    // Registration validated the email, so the local part is non-empty.
    let name = user.email.split('@').next().unwrap_or("Personal");
    create_organization(connection, user.id, name).await
}

/// Creates an organization with its default team.
///
/// The personal organization at signup and an organization created later are
/// the same thing; nothing marks one as special. Run it in a transaction so an
/// organization without its team, or without its admin, can never exist.
///
/// No sub-tenant is created. The tier is optional, and a resource belonging to
/// no project belongs to the organization, so a project nobody asked for would
/// be a row that shows up in every picker and means nothing. The default team
/// is organization-scoped, so it reaches every project created later.
pub(crate) async fn create_organization(
    connection: &mut AsyncPgConnection,
    owner: Uuid,
    name: &str,
) -> Result<(Organization, Team), diesel::result::Error> {
    let admin_roles = vec![ADMIN_ROLE.to_owned()];

    let organization: Organization = diesel::insert_into(organizations::table)
        .values(NewOrganization { name })
        .returning(Organization::as_returning())
        .get_result(connection)
        .await?;

    diesel::insert_into(organization_memberships::table)
        .values(NewOrganizationMembership {
            organization_id: organization.id,
            user_id: owner,
            roles: &admin_roles,
        })
        .execute(connection)
        .await?;

    let team = create_team(
        connection,
        organization.id,
        DEFAULT_TEAM_NAME,
        SubTenantScope::Organization,
        owner,
    )
    .await?;

    Ok((organization, team))
}

/// Creates a sub-tenant in an organization.
///
/// Nobody is enrolled in it: an organization admin bypasses the tier and a
/// full member cascades into it, so a membership row here would be a second
/// copy of a fact the resolver already reads. Guests are enrolled by name,
/// which is a deliberate act rather than a side effect of creation.
pub(crate) async fn create_sub_tenant(
    connection: &mut AsyncPgConnection,
    organization_id: Uuid,
    name: &str,
) -> Result<SubTenant, diesel::result::Error> {
    diesel::insert_into(sub_tenants::table)
        .values(NewSubTenant {
            organization_id,
            name,
        })
        .returning(SubTenant::as_returning())
        .get_result(connection)
        .await
}

/// Creates a team in an organization, with `owner` as its admin member.
///
/// `scope` decides which of the organization's sub-tenants the team reaches;
/// an [`Explicit`](SubTenantScope::Explicit) team starts out reaching none,
/// and is granted them one at a time afterwards.
pub(crate) async fn create_team(
    connection: &mut AsyncPgConnection,
    organization_id: Uuid,
    name: &str,
    scope: SubTenantScope,
    owner: Uuid,
) -> Result<Team, diesel::result::Error> {
    let admin_roles = vec![ADMIN_ROLE.to_owned()];

    let team: Team = diesel::insert_into(teams::table)
        .values(NewTeam {
            organization_id,
            name,
            sub_tenant_scope: scope,
        })
        .returning(Team::as_returning())
        .get_result(connection)
        .await?;

    diesel::insert_into(team_memberships::table)
        .values(NewTeamMembership {
            team_id: team.id,
            user_id: Some(owner),
            roles: &admin_roles,
        })
        .execute(connection)
        .await?;

    Ok(team)
}
