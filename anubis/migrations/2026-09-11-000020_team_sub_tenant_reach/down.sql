-- Lossy by nature: a single column can hold one grant, so a team reaching two
-- sub-tenants comes back reaching the first of them by id. The organizations
-- whose default sub-tenant was dropped do not get it back, because nothing
-- records that it was ever there.
ALTER TABLE teams
    ADD COLUMN sub_tenant_id UUID,
    ADD CONSTRAINT teams_sub_tenant_fkey
        FOREIGN KEY (sub_tenant_id, organization_id)
        REFERENCES sub_tenants (id, organization_id) ON DELETE CASCADE;

CREATE INDEX teams_sub_tenant_id_idx ON teams (sub_tenant_id);

UPDATE teams SET sub_tenant_id = (
    SELECT sub_tenant_id FROM team_sub_tenants
    WHERE team_id = teams.id
    ORDER BY sub_tenant_id
    LIMIT 1
)
WHERE sub_tenant_scope = 'explicit';

DROP TABLE team_sub_tenants;

ALTER TABLE teams DROP COLUMN sub_tenant_scope;

ALTER TABLE teams DROP CONSTRAINT teams_id_organization_id_key;
