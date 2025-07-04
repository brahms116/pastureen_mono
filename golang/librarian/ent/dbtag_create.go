// Code generated by ent, DO NOT EDIT.

package ent

import (
	"context"
	"errors"
	"fmt"

	"entgo.io/ent/dialect"
	"entgo.io/ent/dialect/sql"
	"entgo.io/ent/dialect/sql/sqlgraph"
	"entgo.io/ent/schema/field"
	"github.com/brahms116/pastureen_mono/golang/librarian/ent/dblink"
	"github.com/brahms116/pastureen_mono/golang/librarian/ent/dbtag"
)

// DbTagCreate is the builder for creating a DbTag entity.
type DbTagCreate struct {
	config
	mutation *DbTagMutation
	hooks    []Hook
	conflict []sql.ConflictOption
}

// SetID sets the "id" field.
func (dtc *DbTagCreate) SetID(s string) *DbTagCreate {
	dtc.mutation.SetID(s)
	return dtc
}

// AddLinkIDs adds the "links" edge to the DbLink entity by IDs.
func (dtc *DbTagCreate) AddLinkIDs(ids ...string) *DbTagCreate {
	dtc.mutation.AddLinkIDs(ids...)
	return dtc
}

// AddLinks adds the "links" edges to the DbLink entity.
func (dtc *DbTagCreate) AddLinks(d ...*DbLink) *DbTagCreate {
	ids := make([]string, len(d))
	for i := range d {
		ids[i] = d[i].ID
	}
	return dtc.AddLinkIDs(ids...)
}

// Mutation returns the DbTagMutation object of the builder.
func (dtc *DbTagCreate) Mutation() *DbTagMutation {
	return dtc.mutation
}

// Save creates the DbTag in the database.
func (dtc *DbTagCreate) Save(ctx context.Context) (*DbTag, error) {
	return withHooks(ctx, dtc.sqlSave, dtc.mutation, dtc.hooks)
}

// SaveX calls Save and panics if Save returns an error.
func (dtc *DbTagCreate) SaveX(ctx context.Context) *DbTag {
	v, err := dtc.Save(ctx)
	if err != nil {
		panic(err)
	}
	return v
}

// Exec executes the query.
func (dtc *DbTagCreate) Exec(ctx context.Context) error {
	_, err := dtc.Save(ctx)
	return err
}

// ExecX is like Exec, but panics if an error occurs.
func (dtc *DbTagCreate) ExecX(ctx context.Context) {
	if err := dtc.Exec(ctx); err != nil {
		panic(err)
	}
}

// check runs all checks and user-defined validators on the builder.
func (dtc *DbTagCreate) check() error {
	return nil
}

func (dtc *DbTagCreate) sqlSave(ctx context.Context) (*DbTag, error) {
	if err := dtc.check(); err != nil {
		return nil, err
	}
	_node, _spec := dtc.createSpec()
	if err := sqlgraph.CreateNode(ctx, dtc.driver, _spec); err != nil {
		if sqlgraph.IsConstraintError(err) {
			err = &ConstraintError{msg: err.Error(), wrap: err}
		}
		return nil, err
	}
	if _spec.ID.Value != nil {
		if id, ok := _spec.ID.Value.(string); ok {
			_node.ID = id
		} else {
			return nil, fmt.Errorf("unexpected DbTag.ID type: %T", _spec.ID.Value)
		}
	}
	dtc.mutation.id = &_node.ID
	dtc.mutation.done = true
	return _node, nil
}

func (dtc *DbTagCreate) createSpec() (*DbTag, *sqlgraph.CreateSpec) {
	var (
		_node = &DbTag{config: dtc.config}
		_spec = sqlgraph.NewCreateSpec(dbtag.Table, sqlgraph.NewFieldSpec(dbtag.FieldID, field.TypeString))
	)
	_spec.OnConflict = dtc.conflict
	if id, ok := dtc.mutation.ID(); ok {
		_node.ID = id
		_spec.ID.Value = id
	}
	if nodes := dtc.mutation.LinksIDs(); len(nodes) > 0 {
		edge := &sqlgraph.EdgeSpec{
			Rel:     sqlgraph.M2M,
			Inverse: true,
			Table:   dbtag.LinksTable,
			Columns: dbtag.LinksPrimaryKey,
			Bidi:    false,
			Target: &sqlgraph.EdgeTarget{
				IDSpec: sqlgraph.NewFieldSpec(dblink.FieldID, field.TypeString),
			},
		}
		for _, k := range nodes {
			edge.Target.Nodes = append(edge.Target.Nodes, k)
		}
		_spec.Edges = append(_spec.Edges, edge)
	}
	return _node, _spec
}

// OnConflict allows configuring the `ON CONFLICT` / `ON DUPLICATE KEY` clause
// of the `INSERT` statement. For example:
//
//	client.DbTag.Create().
//		OnConflict(
//			// Update the row with the new values
//			// the was proposed for insertion.
//			sql.ResolveWithNewValues(),
//		).
//		Exec(ctx)
func (dtc *DbTagCreate) OnConflict(opts ...sql.ConflictOption) *DbTagUpsertOne {
	dtc.conflict = opts
	return &DbTagUpsertOne{
		create: dtc,
	}
}

// OnConflictColumns calls `OnConflict` and configures the columns
// as conflict target. Using this option is equivalent to using:
//
//	client.DbTag.Create().
//		OnConflict(sql.ConflictColumns(columns...)).
//		Exec(ctx)
func (dtc *DbTagCreate) OnConflictColumns(columns ...string) *DbTagUpsertOne {
	dtc.conflict = append(dtc.conflict, sql.ConflictColumns(columns...))
	return &DbTagUpsertOne{
		create: dtc,
	}
}

type (
	// DbTagUpsertOne is the builder for "upsert"-ing
	//  one DbTag node.
	DbTagUpsertOne struct {
		create *DbTagCreate
	}

	// DbTagUpsert is the "OnConflict" setter.
	DbTagUpsert struct {
		*sql.UpdateSet
	}
)

// UpdateNewValues updates the mutable fields using the new values that were set on create except the ID field.
// Using this option is equivalent to using:
//
//	client.DbTag.Create().
//		OnConflict(
//			sql.ResolveWithNewValues(),
//			sql.ResolveWith(func(u *sql.UpdateSet) {
//				u.SetIgnore(dbtag.FieldID)
//			}),
//		).
//		Exec(ctx)
func (u *DbTagUpsertOne) UpdateNewValues() *DbTagUpsertOne {
	u.create.conflict = append(u.create.conflict, sql.ResolveWithNewValues())
	u.create.conflict = append(u.create.conflict, sql.ResolveWith(func(s *sql.UpdateSet) {
		if _, exists := u.create.mutation.ID(); exists {
			s.SetIgnore(dbtag.FieldID)
		}
	}))
	return u
}

// Ignore sets each column to itself in case of conflict.
// Using this option is equivalent to using:
//
//	client.DbTag.Create().
//	    OnConflict(sql.ResolveWithIgnore()).
//	    Exec(ctx)
func (u *DbTagUpsertOne) Ignore() *DbTagUpsertOne {
	u.create.conflict = append(u.create.conflict, sql.ResolveWithIgnore())
	return u
}

// DoNothing configures the conflict_action to `DO NOTHING`.
// Supported only by SQLite and PostgreSQL.
func (u *DbTagUpsertOne) DoNothing() *DbTagUpsertOne {
	u.create.conflict = append(u.create.conflict, sql.DoNothing())
	return u
}

// Update allows overriding fields `UPDATE` values. See the DbTagCreate.OnConflict
// documentation for more info.
func (u *DbTagUpsertOne) Update(set func(*DbTagUpsert)) *DbTagUpsertOne {
	u.create.conflict = append(u.create.conflict, sql.ResolveWith(func(update *sql.UpdateSet) {
		set(&DbTagUpsert{UpdateSet: update})
	}))
	return u
}

// Exec executes the query.
func (u *DbTagUpsertOne) Exec(ctx context.Context) error {
	if len(u.create.conflict) == 0 {
		return errors.New("ent: missing options for DbTagCreate.OnConflict")
	}
	return u.create.Exec(ctx)
}

// ExecX is like Exec, but panics if an error occurs.
func (u *DbTagUpsertOne) ExecX(ctx context.Context) {
	if err := u.create.Exec(ctx); err != nil {
		panic(err)
	}
}

// Exec executes the UPSERT query and returns the inserted/updated ID.
func (u *DbTagUpsertOne) ID(ctx context.Context) (id string, err error) {
	if u.create.driver.Dialect() == dialect.MySQL {
		// In case of "ON CONFLICT", there is no way to get back non-numeric ID
		// fields from the database since MySQL does not support the RETURNING clause.
		return id, errors.New("ent: DbTagUpsertOne.ID is not supported by MySQL driver. Use DbTagUpsertOne.Exec instead")
	}
	node, err := u.create.Save(ctx)
	if err != nil {
		return id, err
	}
	return node.ID, nil
}

// IDX is like ID, but panics if an error occurs.
func (u *DbTagUpsertOne) IDX(ctx context.Context) string {
	id, err := u.ID(ctx)
	if err != nil {
		panic(err)
	}
	return id
}

// DbTagCreateBulk is the builder for creating many DbTag entities in bulk.
type DbTagCreateBulk struct {
	config
	err      error
	builders []*DbTagCreate
	conflict []sql.ConflictOption
}

// Save creates the DbTag entities in the database.
func (dtcb *DbTagCreateBulk) Save(ctx context.Context) ([]*DbTag, error) {
	if dtcb.err != nil {
		return nil, dtcb.err
	}
	specs := make([]*sqlgraph.CreateSpec, len(dtcb.builders))
	nodes := make([]*DbTag, len(dtcb.builders))
	mutators := make([]Mutator, len(dtcb.builders))
	for i := range dtcb.builders {
		func(i int, root context.Context) {
			builder := dtcb.builders[i]
			var mut Mutator = MutateFunc(func(ctx context.Context, m Mutation) (Value, error) {
				mutation, ok := m.(*DbTagMutation)
				if !ok {
					return nil, fmt.Errorf("unexpected mutation type %T", m)
				}
				if err := builder.check(); err != nil {
					return nil, err
				}
				builder.mutation = mutation
				var err error
				nodes[i], specs[i] = builder.createSpec()
				if i < len(mutators)-1 {
					_, err = mutators[i+1].Mutate(root, dtcb.builders[i+1].mutation)
				} else {
					spec := &sqlgraph.BatchCreateSpec{Nodes: specs}
					spec.OnConflict = dtcb.conflict
					// Invoke the actual operation on the latest mutation in the chain.
					if err = sqlgraph.BatchCreate(ctx, dtcb.driver, spec); err != nil {
						if sqlgraph.IsConstraintError(err) {
							err = &ConstraintError{msg: err.Error(), wrap: err}
						}
					}
				}
				if err != nil {
					return nil, err
				}
				mutation.id = &nodes[i].ID
				mutation.done = true
				return nodes[i], nil
			})
			for i := len(builder.hooks) - 1; i >= 0; i-- {
				mut = builder.hooks[i](mut)
			}
			mutators[i] = mut
		}(i, ctx)
	}
	if len(mutators) > 0 {
		if _, err := mutators[0].Mutate(ctx, dtcb.builders[0].mutation); err != nil {
			return nil, err
		}
	}
	return nodes, nil
}

// SaveX is like Save, but panics if an error occurs.
func (dtcb *DbTagCreateBulk) SaveX(ctx context.Context) []*DbTag {
	v, err := dtcb.Save(ctx)
	if err != nil {
		panic(err)
	}
	return v
}

// Exec executes the query.
func (dtcb *DbTagCreateBulk) Exec(ctx context.Context) error {
	_, err := dtcb.Save(ctx)
	return err
}

// ExecX is like Exec, but panics if an error occurs.
func (dtcb *DbTagCreateBulk) ExecX(ctx context.Context) {
	if err := dtcb.Exec(ctx); err != nil {
		panic(err)
	}
}

// OnConflict allows configuring the `ON CONFLICT` / `ON DUPLICATE KEY` clause
// of the `INSERT` statement. For example:
//
//	client.DbTag.CreateBulk(builders...).
//		OnConflict(
//			// Update the row with the new values
//			// the was proposed for insertion.
//			sql.ResolveWithNewValues(),
//		).
//		Exec(ctx)
func (dtcb *DbTagCreateBulk) OnConflict(opts ...sql.ConflictOption) *DbTagUpsertBulk {
	dtcb.conflict = opts
	return &DbTagUpsertBulk{
		create: dtcb,
	}
}

// OnConflictColumns calls `OnConflict` and configures the columns
// as conflict target. Using this option is equivalent to using:
//
//	client.DbTag.Create().
//		OnConflict(sql.ConflictColumns(columns...)).
//		Exec(ctx)
func (dtcb *DbTagCreateBulk) OnConflictColumns(columns ...string) *DbTagUpsertBulk {
	dtcb.conflict = append(dtcb.conflict, sql.ConflictColumns(columns...))
	return &DbTagUpsertBulk{
		create: dtcb,
	}
}

// DbTagUpsertBulk is the builder for "upsert"-ing
// a bulk of DbTag nodes.
type DbTagUpsertBulk struct {
	create *DbTagCreateBulk
}

// UpdateNewValues updates the mutable fields using the new values that
// were set on create. Using this option is equivalent to using:
//
//	client.DbTag.Create().
//		OnConflict(
//			sql.ResolveWithNewValues(),
//			sql.ResolveWith(func(u *sql.UpdateSet) {
//				u.SetIgnore(dbtag.FieldID)
//			}),
//		).
//		Exec(ctx)
func (u *DbTagUpsertBulk) UpdateNewValues() *DbTagUpsertBulk {
	u.create.conflict = append(u.create.conflict, sql.ResolveWithNewValues())
	u.create.conflict = append(u.create.conflict, sql.ResolveWith(func(s *sql.UpdateSet) {
		for _, b := range u.create.builders {
			if _, exists := b.mutation.ID(); exists {
				s.SetIgnore(dbtag.FieldID)
			}
		}
	}))
	return u
}

// Ignore sets each column to itself in case of conflict.
// Using this option is equivalent to using:
//
//	client.DbTag.Create().
//		OnConflict(sql.ResolveWithIgnore()).
//		Exec(ctx)
func (u *DbTagUpsertBulk) Ignore() *DbTagUpsertBulk {
	u.create.conflict = append(u.create.conflict, sql.ResolveWithIgnore())
	return u
}

// DoNothing configures the conflict_action to `DO NOTHING`.
// Supported only by SQLite and PostgreSQL.
func (u *DbTagUpsertBulk) DoNothing() *DbTagUpsertBulk {
	u.create.conflict = append(u.create.conflict, sql.DoNothing())
	return u
}

// Update allows overriding fields `UPDATE` values. See the DbTagCreateBulk.OnConflict
// documentation for more info.
func (u *DbTagUpsertBulk) Update(set func(*DbTagUpsert)) *DbTagUpsertBulk {
	u.create.conflict = append(u.create.conflict, sql.ResolveWith(func(update *sql.UpdateSet) {
		set(&DbTagUpsert{UpdateSet: update})
	}))
	return u
}

// Exec executes the query.
func (u *DbTagUpsertBulk) Exec(ctx context.Context) error {
	if u.create.err != nil {
		return u.create.err
	}
	for i, b := range u.create.builders {
		if len(b.conflict) != 0 {
			return fmt.Errorf("ent: OnConflict was set for builder %d. Set it on the DbTagCreateBulk instead", i)
		}
	}
	if len(u.create.conflict) == 0 {
		return errors.New("ent: missing options for DbTagCreateBulk.OnConflict")
	}
	return u.create.Exec(ctx)
}

// ExecX is like Exec, but panics if an error occurs.
func (u *DbTagUpsertBulk) ExecX(ctx context.Context) {
	if err := u.create.Exec(ctx); err != nil {
		panic(err)
	}
}
