-- Teams and sub-tenants are orthogonal organizational infrastructure, so the
-- relationship between them is many-to-many rather than a team belonging to a
-- sub-tenant. A team is a group of people carrying roles; a sub-tenant is a
-- container the work lives in. Which projects a team reaches is one fact, and
-- a single foreign key on `teams` could only carry it for one project.

-- The composite foreign key needs a matching unique key to reference. The
-- primary key already makes `id` unique, so this constraint adds no new
-- restriction; it exists so `(team_id, organization_id)` is referenceable.
ALTER TABLE teams ADD CONSTRAINT teams_id_organization_id_key UNIQUE (id, organization_id);

-- 'organization' reaches every sub-tenant in the organization, the ones
-- created after the team included, which is what a leadership team wants.
-- 'explicit' reaches exactly the sub-tenants named in team_sub_tenants, which
-- may be none: a team whose resources are all its own needs no project at all.
ALTER TABLE teams
    ADD COLUMN sub_tenant_scope TEXT NOT NULL DEFAULT 'organization'
        CHECK (sub_tenant_scope IN ('organization', 'explicit'));

CREATE TABLE team_sub_tenants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id UUID NOT NULL,
    sub_tenant_id UUID NOT NULL,
    -- Carried rather than derived so both composite keys can be referenced.
    -- Each side already agrees with it, so it cannot drift, and a grant across
    -- two organizations is unrepresentable rather than merely unwritten.
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT team_sub_tenants_team_fkey
        FOREIGN KEY (team_id, organization_id)
        REFERENCES teams (id, organization_id) ON DELETE CASCADE,
    CONSTRAINT team_sub_tenants_sub_tenant_fkey
        FOREIGN KEY (sub_tenant_id, organization_id)
        REFERENCES sub_tenants (id, organization_id) ON DELETE CASCADE,
    UNIQUE (team_id, sub_tenant_id)
);

CREATE INDEX team_sub_tenants_sub_tenant_id_idx ON team_sub_tenants (sub_tenant_id);

CREATE TRIGGER set_updated_at BEFORE UPDATE ON team_sub_tenants
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- A team that was scoped to one sub-tenant keeps exactly that reach, now
-- spelled as an explicit grant. A team that was organization-level stays so,
-- which is the column default.
INSERT INTO team_sub_tenants (team_id, sub_tenant_id, organization_id)
SELECT id, sub_tenant_id, organization_id FROM teams WHERE sub_tenant_id IS NOT NULL;

UPDATE teams SET sub_tenant_scope = 'explicit' WHERE sub_tenant_id IS NOT NULL;

ALTER TABLE teams DROP COLUMN sub_tenant_id;

-- The default sub-tenant existed because an ownership chain had to end
-- somewhere and only a sub-tenant could hold project work. Resources may now
-- be owned directly by an organization, so a project nobody asked for is a
-- row that shows up in a picker and means nothing. Drop the untouched ones:
-- anything holding a grant, a member, or a name somebody chose is kept.
DELETE FROM sub_tenants
WHERE name = 'Main'
  AND NOT EXISTS (SELECT 1 FROM team_sub_tenants WHERE sub_tenant_id = sub_tenants.id)
  AND NOT EXISTS (SELECT 1 FROM sub_tenant_memberships WHERE sub_tenant_id = sub_tenants.id);
