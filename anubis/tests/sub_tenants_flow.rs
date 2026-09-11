//! The sub-tenant tier, resolved end to end against a real Postgres database.
//!
//! One story, told in the order somebody would live it: a founder signs up
//! into an organization with no projects at all, creates two, and then four
//! kinds of person try to reach them. Every rule in
//! `anubis::tenancy::resolve_sub_tenant_access` has a step here, and the ones
//! that matter most are the refusals: a guest who was granted one sub-tenant
//! must not see the other, a team granted one must not leak into the other, a
//! suspension must beat every grant including an administrator's, and a
//! sub-tenant the caller cannot reach must be indistinguishable from one that
//! does not exist.
//!
//! Requires `DATABASE_URL`; without it the test logs a skip and passes. CI
//! always provides one.

mod support;

use anubis::guard::{SubTenantMember, TeamMember};
use anubis::http::ApiError;
use anubis::roles::Action;
use anubis::schema::{
    organization_memberships, sub_tenant_memberships, sub_tenants, team_memberships,
    team_sub_tenants, teams, users,
};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde_json::json;
use uuid::Uuid;

use support::{TestDatabase, register, send};

/// `default` reads and `editor` also updates, so a caller's resolved roles are
/// visible in what the probe lets them do rather than only in what it reports.
const ROLES_YML: &str = "
roles:
  default:
    models:
      Probe: [read]
  editor:
    includes: [default]
    models:
      Probe: [read, update]
  billing:
    includes: [default]
    models: {}
  admin:
    includes: [editor, billing]
    models:
      Probe: [manage]
";

/// Reports the caller's resolved standing, which is what every step asserts on.
async fn sub_tenant_probe(member: SubTenantMember) -> Result<impl IntoResponse, ApiError> {
    member.require(Action::Read, "Probe")?;
    Ok(Json(json!({
        "sub_tenant": member.sub_tenant.name,
        "roles": member.access.roles,
        "reach": format!("{:?}", member.access.reach),
        "can_update": member.can(Action::Update, "Probe"),
    })))
}

async fn team_probe(member: TeamMember) -> Result<impl IntoResponse, ApiError> {
    Ok(Json(json!({ "team": member.team.name })))
}

fn sub_tenant_path(sub_tenant: Uuid) -> String {
    format!("/sub-tenants/{sub_tenant}/probe")
}

fn team_path(team: Uuid) -> String {
    format!("/teams/{team}/probe")
}

async fn user_id(connection: &mut AsyncPgConnection, email: &str) -> Uuid {
    users::table
        .filter(users::email.eq(email))
        .select(users::id)
        .first(connection)
        .await
        .expect("the registered account must exist")
}

/// The organization and team registration bootstrapped for a user.
async fn bootstrapped(connection: &mut AsyncPgConnection, user: Uuid) -> (Uuid, Uuid) {
    teams::table
        .inner_join(
            team_memberships::table.on(team_memberships::team_id
                .eq(teams::id)
                .and(team_memberships::user_id.eq(user))),
        )
        .select((teams::organization_id, teams::id))
        .first(connection)
        .await
        .expect("registration bootstraps one team")
}

async fn create_sub_tenant(
    connection: &mut AsyncPgConnection,
    organization: Uuid,
    name: &str,
) -> Uuid {
    diesel::insert_into(sub_tenants::table)
        .values((
            sub_tenants::organization_id.eq(organization),
            sub_tenants::name.eq(name),
        ))
        .returning(sub_tenants::id)
        .get_result(connection)
        .await
        .expect("the sub-tenant must be creatable")
}

async fn create_team(
    connection: &mut AsyncPgConnection,
    organization: Uuid,
    name: &str,
    scope: &str,
) -> Uuid {
    diesel::insert_into(teams::table)
        .values((
            teams::organization_id.eq(organization),
            teams::name.eq(name),
            teams::sub_tenant_scope.eq(scope),
        ))
        .returning(teams::id)
        .get_result(connection)
        .await
        .expect("the team must be creatable")
}

/// Grants an explicitly scoped team reach into one sub-tenant.
async fn grant_sub_tenant(
    connection: &mut AsyncPgConnection,
    organization: Uuid,
    team: Uuid,
    sub_tenant: Uuid,
) {
    diesel::insert_into(team_sub_tenants::table)
        .values((
            team_sub_tenants::team_id.eq(team),
            team_sub_tenants::sub_tenant_id.eq(sub_tenant),
            team_sub_tenants::organization_id.eq(organization),
        ))
        .execute(connection)
        .await
        .expect("the grant must be writable");
}

async fn join_team(connection: &mut AsyncPgConnection, team: Uuid, user: Uuid, roles: &[&str]) {
    diesel::insert_into(team_memberships::table)
        .values((
            team_memberships::team_id.eq(team),
            team_memberships::user_id.eq(Some(user)),
            team_memberships::roles.eq(roles
                .iter()
                .map(|role| (*role).to_owned())
                .collect::<Vec<_>>()),
        ))
        .execute(connection)
        .await
        .expect("the team membership must be creatable");
}

async fn join_organization(
    connection: &mut AsyncPgConnection,
    organization: Uuid,
    user: Uuid,
    roles: &[&str],
    access: &str,
) {
    diesel::insert_into(organization_memberships::table)
        .values((
            organization_memberships::organization_id.eq(organization),
            organization_memberships::user_id.eq(user),
            organization_memberships::roles.eq(roles
                .iter()
                .map(|role| (*role).to_owned())
                .collect::<Vec<_>>()),
            organization_memberships::access.eq(access),
        ))
        .execute(connection)
        .await
        .expect("the organization membership must be creatable");
}

async fn join_sub_tenant(
    connection: &mut AsyncPgConnection,
    sub_tenant: Uuid,
    user: Uuid,
    roles: &[&str],
    suspended: bool,
) {
    diesel::insert_into(sub_tenant_memberships::table)
        .values((
            sub_tenant_memberships::sub_tenant_id.eq(sub_tenant),
            sub_tenant_memberships::user_id.eq(user),
            sub_tenant_memberships::roles.eq(roles
                .iter()
                .map(|role| (*role).to_owned())
                .collect::<Vec<_>>()),
            sub_tenant_memberships::suspended_at.eq(suspended.then(Utc::now)),
        ))
        .execute(connection)
        .await
        .expect("the sub-tenant membership must be creatable");
}

async fn suspend_organization_member(
    connection: &mut AsyncPgConnection,
    organization: Uuid,
    user: Uuid,
) {
    diesel::update(
        organization_memberships::table
            .filter(organization_memberships::organization_id.eq(organization))
            .filter(organization_memberships::user_id.eq(user)),
    )
    .set(organization_memberships::suspended_at.eq(Some(Utc::now())))
    .execute(connection)
    .await
    .expect("the membership must be suspendable");
}

async fn suspend_sub_tenant_member(
    connection: &mut AsyncPgConnection,
    sub_tenant: Uuid,
    user: Uuid,
) {
    diesel::update(
        sub_tenant_memberships::table
            .filter(sub_tenant_memberships::sub_tenant_id.eq(sub_tenant))
            .filter(sub_tenant_memberships::user_id.eq(user)),
    )
    .set(sub_tenant_memberships::suspended_at.eq(Some(Utc::now())))
    .execute(connection)
    .await
    .expect("the membership must be suspendable");
}

#[tokio::test]
#[expect(
    clippy::too_many_lines,
    reason = "one linear narrative, told in the order somebody would live it"
)]
async fn sub_tenant_access_resolves_across_every_tier() {
    let Some(database) = TestDatabase::create("sub_tenants_flow").await else {
        return;
    };
    let pool = database.pool().await;

    let config = support::Harness::config();
    let roles = anubis::roles::RoleSet::from_yaml(ROLES_YML).expect("roles must parse");
    let (mailer, _outbox) = anubis::mail::Mailer::test();
    let rate_limit = anubis::rate_limit::RateLimiter::new(&config.rate_limit);

    let router = Router::new()
        .nest(
            "/auth",
            anubis::auth::router(pool.clone(), mailer, &config, &rate_limit),
        )
        .route("/sub-tenants/{sub_tenant_id}/probe", get(sub_tenant_probe))
        .route("/teams/{team_id}/probe", get(team_probe))
        .layer(anubis::guard::layer(pool.clone(), roles));

    let run = Uuid::new_v4();
    let founder_email = format!("sub-tenant-founder-{run}@example.com");
    let guest_email = format!("sub-tenant-guest-{run}@example.com");
    let teammate_email = format!("sub-tenant-teammate-{run}@example.com");
    let crew_email = format!("sub-tenant-crew-{run}@example.com");
    let polyglot_email = format!("sub-tenant-polyglot-{run}@example.com");
    let stranger_email = format!("sub-tenant-stranger-{run}@example.com");

    let founder = register(&router, &founder_email).await;
    let guest = register(&router, &guest_email).await;
    let teammate = register(&router, &teammate_email).await;
    let crew = register(&router, &crew_email).await;
    let polyglot = register(&router, &polyglot_email).await;
    let stranger = register(&router, &stranger_email).await;

    let mut connection = pool.get().await.expect("a connection must be available");
    let founder_id = user_id(&mut connection, &founder_email).await;
    let guest_id = user_id(&mut connection, &guest_email).await;
    let teammate_id = user_id(&mut connection, &teammate_email).await;
    let crew_id = user_id(&mut connection, &crew_email).await;
    let polyglot_id = user_id(&mut connection, &polyglot_email).await;
    let stranger_id = user_id(&mut connection, &stranger_email).await;

    // ---------------------------------------------------------------------
    // Signing up creates no project at all: the tier is opt-in, and work that
    // belongs to no project belongs to the organization.
    // ---------------------------------------------------------------------
    let (organization, general) = bootstrapped(&mut connection, founder_id).await;
    let sub_tenants_in_organization: i64 = sub_tenants::table
        .filter(sub_tenants::organization_id.eq(organization))
        .count()
        .get_result(&mut connection)
        .await
        .expect("the count must run");
    assert_eq!(
        sub_tenants_in_organization, 0,
        "registration creates no sub-tenant, so an organization starts with none",
    );
    let general_scope: String = teams::table
        .find(general)
        .select(teams::sub_tenant_scope)
        .first(&mut connection)
        .await
        .expect("the bootstrapped team must exist");
    assert_eq!(
        general_scope, "organization",
        "the bootstrapped team reaches every project, including ones made later",
    );

    let main = create_sub_tenant(&mut connection, organization, "Main").await;
    let atlas = create_sub_tenant(&mut connection, organization, "Atlas").await;

    // ---------------------------------------------------------------------
    // The founder administers the organization, so the tier is transparent.
    // ---------------------------------------------------------------------
    let (status, _headers, body) = send(
        &router,
        "GET",
        &sub_tenant_path(atlas),
        None,
        Some(&founder),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "body: {body}");
    assert_eq!(body["sub_tenant"], json!("Atlas"));
    assert_eq!(body["reach"], json!("Administrator"));
    assert_eq!(body["can_update"], json!(true));

    // ---------------------------------------------------------------------
    // A stranger cannot tell an unreachable sub-tenant from a missing one.
    // ---------------------------------------------------------------------
    let (unreachable_status, _headers, unreachable_body) = send(
        &router,
        "GET",
        &sub_tenant_path(atlas),
        None,
        Some(&stranger),
    )
    .await;
    let ghost = sub_tenant_path(Uuid::new_v4());
    let (missing_status, _headers, missing_body) =
        send(&router, "GET", &ghost, None, Some(&stranger)).await;
    assert_eq!(unreachable_status, StatusCode::NOT_FOUND);
    assert_eq!(missing_status, StatusCode::NOT_FOUND);
    assert_eq!(
        unreachable_body, missing_body,
        "the two answers must be identical, so probing ids reveals nothing",
    );

    // The founder is equally shut out of the stranger's own organization.
    let (stranger_organization, _team) = bootstrapped(&mut connection, stranger_id).await;
    let stranger_main = create_sub_tenant(&mut connection, stranger_organization, "Theirs").await;
    let (status, _headers, _body) = send(
        &router,
        "GET",
        &sub_tenant_path(stranger_main),
        None,
        Some(&founder),
    )
    .await;
    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "tenants do not see each other"
    );

    // A malformed id can never exist, and existence is not revealed.
    let (status, _headers, _body) = send(
        &router,
        "GET",
        "/sub-tenants/not-a-uuid/probe",
        None,
        Some(&founder),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    // The schema itself refuses a grant that spans two organizations, so the
    // resolver's filter is a second lock rather than the only one. Both
    // composite foreign keys read the row's own `organization_id`, so whichever
    // one it is made to agree with, the other refuses it.
    for owner in [organization, stranger_organization] {
        let across_tenants = diesel::insert_into(team_sub_tenants::table)
            .values((
                team_sub_tenants::team_id.eq(general),
                team_sub_tenants::sub_tenant_id.eq(stranger_main),
                team_sub_tenants::organization_id.eq(owner),
            ))
            .execute(&mut connection)
            .await;
        assert!(
            across_tenants.is_err(),
            "a team cannot be granted another organization's sub-tenant",
        );
    }

    // ---------------------------------------------------------------------
    // A guest reaches only what they were granted by name.
    // ---------------------------------------------------------------------
    join_organization(
        &mut connection,
        organization,
        guest_id,
        &["editor"],
        "guest",
    )
    .await;
    for target in [main, atlas] {
        let (status, _headers, _body) =
            send(&router, "GET", &sub_tenant_path(target), None, Some(&guest)).await;
        assert_eq!(
            status,
            StatusCode::NOT_FOUND,
            "a guest organization membership cascades into nothing",
        );
    }

    join_sub_tenant(&mut connection, atlas, guest_id, &["default"], false).await;
    let (status, _headers, body) =
        send(&router, "GET", &sub_tenant_path(atlas), None, Some(&guest)).await;
    assert_eq!(status, StatusCode::OK, "body: {body}");
    assert_eq!(body["reach"], json!("Grant"));
    assert_eq!(
        body["roles"],
        json!(["default"]),
        "the organization's editor role must not cascade to a guest",
    );
    assert_eq!(
        body["can_update"],
        json!(false),
        "so the guest holds only what the grant carries",
    );

    let (status, _headers, _body) =
        send(&router, "GET", &sub_tenant_path(main), None, Some(&guest)).await;
    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "one grant admits the guest to one sub-tenant",
    );

    // ---------------------------------------------------------------------
    // An organization-scoped team reaches every sub-tenant, and an explicit
    // one reaches exactly what it was granted.
    // ---------------------------------------------------------------------
    join_team(&mut connection, general, teammate_id, &["editor"]).await;
    for (target, name) in [(main, "Main"), (atlas, "Atlas")] {
        let (status, _headers, body) = send(
            &router,
            "GET",
            &sub_tenant_path(target),
            None,
            Some(&teammate),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "body: {body}");
        assert_eq!(body["sub_tenant"], json!(name));
        assert_eq!(body["reach"], json!("Team"));
        assert_eq!(
            body["can_update"],
            json!(true),
            "the team's roles are the standing a team member holds",
        );
    }

    let atlas_crew = create_team(&mut connection, organization, "Atlas Crew", "explicit").await;
    join_team(&mut connection, atlas_crew, crew_id, &["editor"]).await;
    let (status, _headers, _body) =
        send(&router, "GET", &sub_tenant_path(atlas), None, Some(&crew)).await;
    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "an explicit team starts out reaching nothing, so the grant is what admits it",
    );

    grant_sub_tenant(&mut connection, organization, atlas_crew, atlas).await;
    let (status, _headers, body) =
        send(&router, "GET", &sub_tenant_path(atlas), None, Some(&crew)).await;
    assert_eq!(status, StatusCode::OK, "body: {body}");
    assert_eq!(body["reach"], json!("Team"));
    let (status, _headers, _body) =
        send(&router, "GET", &sub_tenant_path(main), None, Some(&crew)).await;
    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "a team granted one sub-tenant never leaks into another",
    );

    // Two grants are two reaches: the relationship is many-to-many, which is
    // the whole reason it is a table rather than a column.
    grant_sub_tenant(&mut connection, organization, atlas_crew, main).await;
    for target in [main, atlas] {
        let (status, _headers, body) =
            send(&router, "GET", &sub_tenant_path(target), None, Some(&crew)).await;
        assert_eq!(status, StatusCode::OK, "body: {body}");
    }

    // ---------------------------------------------------------------------
    // Somebody standing at three tiers holds the union of all three.
    // ---------------------------------------------------------------------
    join_organization(
        &mut connection,
        organization,
        polyglot_id,
        &["default"],
        "full",
    )
    .await;
    join_sub_tenant(&mut connection, atlas, polyglot_id, &["editor"], false).await;
    join_team(&mut connection, general, polyglot_id, &["billing"]).await;
    let (status, _headers, body) = send(
        &router,
        "GET",
        &sub_tenant_path(atlas),
        None,
        Some(&polyglot),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "body: {body}");
    assert_eq!(
        body["roles"],
        json!(["billing", "default", "editor"]),
        "the resolved roles are the deduplicated union of every applying grant",
    );
    assert_eq!(
        body["reach"],
        json!("Organization"),
        "and the broadest path is the one reported",
    );

    // Both team members can reach their teams before anything is suspended,
    // which is what makes the refusals below about the suspension.
    for (cookie, team) in [(&teammate, general), (&crew, atlas_crew)] {
        let (status, _headers, body) =
            send(&router, "GET", &team_path(team), None, Some(cookie)).await;
        assert_eq!(status, StatusCode::OK, "body: {body}");
    }

    // ---------------------------------------------------------------------
    // A suspension is a deny, and it outranks every grant.
    // ---------------------------------------------------------------------
    suspend_sub_tenant_member(&mut connection, atlas, guest_id).await;
    let (status, _headers, _body) =
        send(&router, "GET", &sub_tenant_path(atlas), None, Some(&guest)).await;
    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "suspending the grant closes the sub-tenant it opened",
    );

    // A full membership would cascade into every sub-tenant; suspended, it
    // cuts the member out of the organization's work altogether, the team
    // route included.
    join_organization(
        &mut connection,
        organization,
        teammate_id,
        &["editor"],
        "full",
    )
    .await;
    suspend_organization_member(&mut connection, organization, teammate_id).await;
    for target in [main, atlas] {
        let (status, _headers, _body) = send(
            &router,
            "GET",
            &sub_tenant_path(target),
            None,
            Some(&teammate),
        )
        .await;
        assert_eq!(status, StatusCode::NOT_FOUND, "a suspension beats a team");
    }
    let (status, _headers, _body) =
        send(&router, "GET", &team_path(general), None, Some(&teammate)).await;
    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "an organization suspension closes every team in the organization",
    );

    // A sub-tenant suspension closes that sub-tenant and leaves the team
    // alone. The two tiers are orthogonal: cutting somebody out of a project
    // says nothing about the group of people they belong to, whose own work
    // is not in that project.
    join_sub_tenant(&mut connection, atlas, crew_id, &[], true).await;
    let (status, _headers, _body) =
        send(&router, "GET", &sub_tenant_path(atlas), None, Some(&crew)).await;
    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "a sub-tenant suspension closes the sub-tenant it names",
    );
    let (status, _headers, body) =
        send(&router, "GET", &team_path(atlas_crew), None, Some(&crew)).await;
    assert_eq!(
        status,
        StatusCode::OK,
        "and leaves the team, which is a different tier: {body}",
    );
    let (status, _headers, body) =
        send(&router, "GET", &team_path(general), None, Some(&founder)).await;
    assert_eq!(
        status,
        StatusCode::OK,
        "as it leaves every other team too: {body}",
    );

    // Even the administrator is cut, because a deny an admin bit ignores is
    // not a deny.
    suspend_organization_member(&mut connection, organization, founder_id).await;
    let (status, _headers, _body) = send(
        &router,
        "GET",
        &sub_tenant_path(atlas),
        None,
        Some(&founder),
    )
    .await;
    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "suspension outranks the bypass"
    );
    let (status, _headers, _body) =
        send(&router, "GET", &team_path(general), None, Some(&founder)).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
