// Code generated by ent, DO NOT EDIT.

package ent

import (
	"context"
	"errors"
	"fmt"
	"time"

	"entgo.io/ent/dialect/sql"
	"entgo.io/ent/dialect/sql/sqlgraph"
	"entgo.io/ent/schema/field"
	"github.com/brahms116/pastureen_mono/golang/librarian/ent/dblink"
	"github.com/brahms116/pastureen_mono/golang/librarian/ent/dbtag"
	"github.com/brahms116/pastureen_mono/golang/librarian/ent/predicate"
)

// DbLinkUpdate is the builder for updating DbLink entities.
type DbLinkUpdate struct {
	config
	hooks    []Hook
	mutation *DbLinkMutation
}

// Where appends a list predicates to the DbLinkUpdate builder.
func (dlu *DbLinkUpdate) Where(ps ...predicate.DbLink) *DbLinkUpdate {
	dlu.mutation.Where(ps...)
	return dlu
}

// SetTitle sets the "title" field.
func (dlu *DbLinkUpdate) SetTitle(s string) *DbLinkUpdate {
	dlu.mutation.SetTitle(s)
	return dlu
}

// SetDate sets the "date" field.
func (dlu *DbLinkUpdate) SetDate(t time.Time) *DbLinkUpdate {
	dlu.mutation.SetDate(t)
	return dlu
}

// SetSubtitle sets the "subtitle" field.
func (dlu *DbLinkUpdate) SetSubtitle(s string) *DbLinkUpdate {
	dlu.mutation.SetSubtitle(s)
	return dlu
}

// SetDescription sets the "description" field.
func (dlu *DbLinkUpdate) SetDescription(s string) *DbLinkUpdate {
	dlu.mutation.SetDescription(s)
	return dlu
}

// SetImageURL sets the "image_url" field.
func (dlu *DbLinkUpdate) SetImageURL(s string) *DbLinkUpdate {
	dlu.mutation.SetImageURL(s)
	return dlu
}

// SetNillableImageURL sets the "image_url" field if the given value is not nil.
func (dlu *DbLinkUpdate) SetNillableImageURL(s *string) *DbLinkUpdate {
	if s != nil {
		dlu.SetImageURL(*s)
	}
	return dlu
}

// ClearImageURL clears the value of the "image_url" field.
func (dlu *DbLinkUpdate) ClearImageURL() *DbLinkUpdate {
	dlu.mutation.ClearImageURL()
	return dlu
}

// SetImageAlt sets the "image_alt" field.
func (dlu *DbLinkUpdate) SetImageAlt(s string) *DbLinkUpdate {
	dlu.mutation.SetImageAlt(s)
	return dlu
}

// SetNillableImageAlt sets the "image_alt" field if the given value is not nil.
func (dlu *DbLinkUpdate) SetNillableImageAlt(s *string) *DbLinkUpdate {
	if s != nil {
		dlu.SetImageAlt(*s)
	}
	return dlu
}

// ClearImageAlt clears the value of the "image_alt" field.
func (dlu *DbLinkUpdate) ClearImageAlt() *DbLinkUpdate {
	dlu.mutation.ClearImageAlt()
	return dlu
}

// AddTagIDs adds the "tags" edge to the DbTag entity by IDs.
func (dlu *DbLinkUpdate) AddTagIDs(ids ...string) *DbLinkUpdate {
	dlu.mutation.AddTagIDs(ids...)
	return dlu
}

// AddTags adds the "tags" edges to the DbTag entity.
func (dlu *DbLinkUpdate) AddTags(d ...*DbTag) *DbLinkUpdate {
	ids := make([]string, len(d))
	for i := range d {
		ids[i] = d[i].ID
	}
	return dlu.AddTagIDs(ids...)
}

// Mutation returns the DbLinkMutation object of the builder.
func (dlu *DbLinkUpdate) Mutation() *DbLinkMutation {
	return dlu.mutation
}

// ClearTags clears all "tags" edges to the DbTag entity.
func (dlu *DbLinkUpdate) ClearTags() *DbLinkUpdate {
	dlu.mutation.ClearTags()
	return dlu
}

// RemoveTagIDs removes the "tags" edge to DbTag entities by IDs.
func (dlu *DbLinkUpdate) RemoveTagIDs(ids ...string) *DbLinkUpdate {
	dlu.mutation.RemoveTagIDs(ids...)
	return dlu
}

// RemoveTags removes "tags" edges to DbTag entities.
func (dlu *DbLinkUpdate) RemoveTags(d ...*DbTag) *DbLinkUpdate {
	ids := make([]string, len(d))
	for i := range d {
		ids[i] = d[i].ID
	}
	return dlu.RemoveTagIDs(ids...)
}

// Save executes the query and returns the number of nodes affected by the update operation.
func (dlu *DbLinkUpdate) Save(ctx context.Context) (int, error) {
	return withHooks(ctx, dlu.sqlSave, dlu.mutation, dlu.hooks)
}

// SaveX is like Save, but panics if an error occurs.
func (dlu *DbLinkUpdate) SaveX(ctx context.Context) int {
	affected, err := dlu.Save(ctx)
	if err != nil {
		panic(err)
	}
	return affected
}

// Exec executes the query.
func (dlu *DbLinkUpdate) Exec(ctx context.Context) error {
	_, err := dlu.Save(ctx)
	return err
}

// ExecX is like Exec, but panics if an error occurs.
func (dlu *DbLinkUpdate) ExecX(ctx context.Context) {
	if err := dlu.Exec(ctx); err != nil {
		panic(err)
	}
}

func (dlu *DbLinkUpdate) sqlSave(ctx context.Context) (n int, err error) {
	_spec := sqlgraph.NewUpdateSpec(dblink.Table, dblink.Columns, sqlgraph.NewFieldSpec(dblink.FieldID, field.TypeString))
	if ps := dlu.mutation.predicates; len(ps) > 0 {
		_spec.Predicate = func(selector *sql.Selector) {
			for i := range ps {
				ps[i](selector)
			}
		}
	}
	if value, ok := dlu.mutation.Title(); ok {
		_spec.SetField(dblink.FieldTitle, field.TypeString, value)
	}
	if value, ok := dlu.mutation.Date(); ok {
		_spec.SetField(dblink.FieldDate, field.TypeTime, value)
	}
	if value, ok := dlu.mutation.Subtitle(); ok {
		_spec.SetField(dblink.FieldSubtitle, field.TypeString, value)
	}
	if value, ok := dlu.mutation.Description(); ok {
		_spec.SetField(dblink.FieldDescription, field.TypeString, value)
	}
	if value, ok := dlu.mutation.ImageURL(); ok {
		_spec.SetField(dblink.FieldImageURL, field.TypeString, value)
	}
	if dlu.mutation.ImageURLCleared() {
		_spec.ClearField(dblink.FieldImageURL, field.TypeString)
	}
	if value, ok := dlu.mutation.ImageAlt(); ok {
		_spec.SetField(dblink.FieldImageAlt, field.TypeString, value)
	}
	if dlu.mutation.ImageAltCleared() {
		_spec.ClearField(dblink.FieldImageAlt, field.TypeString)
	}
	if dlu.mutation.TagsCleared() {
		edge := &sqlgraph.EdgeSpec{
			Rel:     sqlgraph.M2M,
			Inverse: false,
			Table:   dblink.TagsTable,
			Columns: dblink.TagsPrimaryKey,
			Bidi:    false,
			Target: &sqlgraph.EdgeTarget{
				IDSpec: sqlgraph.NewFieldSpec(dbtag.FieldID, field.TypeString),
			},
		}
		_spec.Edges.Clear = append(_spec.Edges.Clear, edge)
	}
	if nodes := dlu.mutation.RemovedTagsIDs(); len(nodes) > 0 && !dlu.mutation.TagsCleared() {
		edge := &sqlgraph.EdgeSpec{
			Rel:     sqlgraph.M2M,
			Inverse: false,
			Table:   dblink.TagsTable,
			Columns: dblink.TagsPrimaryKey,
			Bidi:    false,
			Target: &sqlgraph.EdgeTarget{
				IDSpec: sqlgraph.NewFieldSpec(dbtag.FieldID, field.TypeString),
			},
		}
		for _, k := range nodes {
			edge.Target.Nodes = append(edge.Target.Nodes, k)
		}
		_spec.Edges.Clear = append(_spec.Edges.Clear, edge)
	}
	if nodes := dlu.mutation.TagsIDs(); len(nodes) > 0 {
		edge := &sqlgraph.EdgeSpec{
			Rel:     sqlgraph.M2M,
			Inverse: false,
			Table:   dblink.TagsTable,
			Columns: dblink.TagsPrimaryKey,
			Bidi:    false,
			Target: &sqlgraph.EdgeTarget{
				IDSpec: sqlgraph.NewFieldSpec(dbtag.FieldID, field.TypeString),
			},
		}
		for _, k := range nodes {
			edge.Target.Nodes = append(edge.Target.Nodes, k)
		}
		_spec.Edges.Add = append(_spec.Edges.Add, edge)
	}
	if n, err = sqlgraph.UpdateNodes(ctx, dlu.driver, _spec); err != nil {
		if _, ok := err.(*sqlgraph.NotFoundError); ok {
			err = &NotFoundError{dblink.Label}
		} else if sqlgraph.IsConstraintError(err) {
			err = &ConstraintError{msg: err.Error(), wrap: err}
		}
		return 0, err
	}
	dlu.mutation.done = true
	return n, nil
}

// DbLinkUpdateOne is the builder for updating a single DbLink entity.
type DbLinkUpdateOne struct {
	config
	fields   []string
	hooks    []Hook
	mutation *DbLinkMutation
}

// SetTitle sets the "title" field.
func (dluo *DbLinkUpdateOne) SetTitle(s string) *DbLinkUpdateOne {
	dluo.mutation.SetTitle(s)
	return dluo
}

// SetDate sets the "date" field.
func (dluo *DbLinkUpdateOne) SetDate(t time.Time) *DbLinkUpdateOne {
	dluo.mutation.SetDate(t)
	return dluo
}

// SetSubtitle sets the "subtitle" field.
func (dluo *DbLinkUpdateOne) SetSubtitle(s string) *DbLinkUpdateOne {
	dluo.mutation.SetSubtitle(s)
	return dluo
}

// SetDescription sets the "description" field.
func (dluo *DbLinkUpdateOne) SetDescription(s string) *DbLinkUpdateOne {
	dluo.mutation.SetDescription(s)
	return dluo
}

// SetImageURL sets the "image_url" field.
func (dluo *DbLinkUpdateOne) SetImageURL(s string) *DbLinkUpdateOne {
	dluo.mutation.SetImageURL(s)
	return dluo
}

// SetNillableImageURL sets the "image_url" field if the given value is not nil.
func (dluo *DbLinkUpdateOne) SetNillableImageURL(s *string) *DbLinkUpdateOne {
	if s != nil {
		dluo.SetImageURL(*s)
	}
	return dluo
}

// ClearImageURL clears the value of the "image_url" field.
func (dluo *DbLinkUpdateOne) ClearImageURL() *DbLinkUpdateOne {
	dluo.mutation.ClearImageURL()
	return dluo
}

// SetImageAlt sets the "image_alt" field.
func (dluo *DbLinkUpdateOne) SetImageAlt(s string) *DbLinkUpdateOne {
	dluo.mutation.SetImageAlt(s)
	return dluo
}

// SetNillableImageAlt sets the "image_alt" field if the given value is not nil.
func (dluo *DbLinkUpdateOne) SetNillableImageAlt(s *string) *DbLinkUpdateOne {
	if s != nil {
		dluo.SetImageAlt(*s)
	}
	return dluo
}

// ClearImageAlt clears the value of the "image_alt" field.
func (dluo *DbLinkUpdateOne) ClearImageAlt() *DbLinkUpdateOne {
	dluo.mutation.ClearImageAlt()
	return dluo
}

// AddTagIDs adds the "tags" edge to the DbTag entity by IDs.
func (dluo *DbLinkUpdateOne) AddTagIDs(ids ...string) *DbLinkUpdateOne {
	dluo.mutation.AddTagIDs(ids...)
	return dluo
}

// AddTags adds the "tags" edges to the DbTag entity.
func (dluo *DbLinkUpdateOne) AddTags(d ...*DbTag) *DbLinkUpdateOne {
	ids := make([]string, len(d))
	for i := range d {
		ids[i] = d[i].ID
	}
	return dluo.AddTagIDs(ids...)
}

// Mutation returns the DbLinkMutation object of the builder.
func (dluo *DbLinkUpdateOne) Mutation() *DbLinkMutation {
	return dluo.mutation
}

// ClearTags clears all "tags" edges to the DbTag entity.
func (dluo *DbLinkUpdateOne) ClearTags() *DbLinkUpdateOne {
	dluo.mutation.ClearTags()
	return dluo
}

// RemoveTagIDs removes the "tags" edge to DbTag entities by IDs.
func (dluo *DbLinkUpdateOne) RemoveTagIDs(ids ...string) *DbLinkUpdateOne {
	dluo.mutation.RemoveTagIDs(ids...)
	return dluo
}

// RemoveTags removes "tags" edges to DbTag entities.
func (dluo *DbLinkUpdateOne) RemoveTags(d ...*DbTag) *DbLinkUpdateOne {
	ids := make([]string, len(d))
	for i := range d {
		ids[i] = d[i].ID
	}
	return dluo.RemoveTagIDs(ids...)
}

// Where appends a list predicates to the DbLinkUpdate builder.
func (dluo *DbLinkUpdateOne) Where(ps ...predicate.DbLink) *DbLinkUpdateOne {
	dluo.mutation.Where(ps...)
	return dluo
}

// Select allows selecting one or more fields (columns) of the returned entity.
// The default is selecting all fields defined in the entity schema.
func (dluo *DbLinkUpdateOne) Select(field string, fields ...string) *DbLinkUpdateOne {
	dluo.fields = append([]string{field}, fields...)
	return dluo
}

// Save executes the query and returns the updated DbLink entity.
func (dluo *DbLinkUpdateOne) Save(ctx context.Context) (*DbLink, error) {
	return withHooks(ctx, dluo.sqlSave, dluo.mutation, dluo.hooks)
}

// SaveX is like Save, but panics if an error occurs.
func (dluo *DbLinkUpdateOne) SaveX(ctx context.Context) *DbLink {
	node, err := dluo.Save(ctx)
	if err != nil {
		panic(err)
	}
	return node
}

// Exec executes the query on the entity.
func (dluo *DbLinkUpdateOne) Exec(ctx context.Context) error {
	_, err := dluo.Save(ctx)
	return err
}

// ExecX is like Exec, but panics if an error occurs.
func (dluo *DbLinkUpdateOne) ExecX(ctx context.Context) {
	if err := dluo.Exec(ctx); err != nil {
		panic(err)
	}
}

func (dluo *DbLinkUpdateOne) sqlSave(ctx context.Context) (_node *DbLink, err error) {
	_spec := sqlgraph.NewUpdateSpec(dblink.Table, dblink.Columns, sqlgraph.NewFieldSpec(dblink.FieldID, field.TypeString))
	id, ok := dluo.mutation.ID()
	if !ok {
		return nil, &ValidationError{Name: "id", err: errors.New(`ent: missing "DbLink.id" for update`)}
	}
	_spec.Node.ID.Value = id
	if fields := dluo.fields; len(fields) > 0 {
		_spec.Node.Columns = make([]string, 0, len(fields))
		_spec.Node.Columns = append(_spec.Node.Columns, dblink.FieldID)
		for _, f := range fields {
			if !dblink.ValidColumn(f) {
				return nil, &ValidationError{Name: f, err: fmt.Errorf("ent: invalid field %q for query", f)}
			}
			if f != dblink.FieldID {
				_spec.Node.Columns = append(_spec.Node.Columns, f)
			}
		}
	}
	if ps := dluo.mutation.predicates; len(ps) > 0 {
		_spec.Predicate = func(selector *sql.Selector) {
			for i := range ps {
				ps[i](selector)
			}
		}
	}
	if value, ok := dluo.mutation.Title(); ok {
		_spec.SetField(dblink.FieldTitle, field.TypeString, value)
	}
	if value, ok := dluo.mutation.Date(); ok {
		_spec.SetField(dblink.FieldDate, field.TypeTime, value)
	}
	if value, ok := dluo.mutation.Subtitle(); ok {
		_spec.SetField(dblink.FieldSubtitle, field.TypeString, value)
	}
	if value, ok := dluo.mutation.Description(); ok {
		_spec.SetField(dblink.FieldDescription, field.TypeString, value)
	}
	if value, ok := dluo.mutation.ImageURL(); ok {
		_spec.SetField(dblink.FieldImageURL, field.TypeString, value)
	}
	if dluo.mutation.ImageURLCleared() {
		_spec.ClearField(dblink.FieldImageURL, field.TypeString)
	}
	if value, ok := dluo.mutation.ImageAlt(); ok {
		_spec.SetField(dblink.FieldImageAlt, field.TypeString, value)
	}
	if dluo.mutation.ImageAltCleared() {
		_spec.ClearField(dblink.FieldImageAlt, field.TypeString)
	}
	if dluo.mutation.TagsCleared() {
		edge := &sqlgraph.EdgeSpec{
			Rel:     sqlgraph.M2M,
			Inverse: false,
			Table:   dblink.TagsTable,
			Columns: dblink.TagsPrimaryKey,
			Bidi:    false,
			Target: &sqlgraph.EdgeTarget{
				IDSpec: sqlgraph.NewFieldSpec(dbtag.FieldID, field.TypeString),
			},
		}
		_spec.Edges.Clear = append(_spec.Edges.Clear, edge)
	}
	if nodes := dluo.mutation.RemovedTagsIDs(); len(nodes) > 0 && !dluo.mutation.TagsCleared() {
		edge := &sqlgraph.EdgeSpec{
			Rel:     sqlgraph.M2M,
			Inverse: false,
			Table:   dblink.TagsTable,
			Columns: dblink.TagsPrimaryKey,
			Bidi:    false,
			Target: &sqlgraph.EdgeTarget{
				IDSpec: sqlgraph.NewFieldSpec(dbtag.FieldID, field.TypeString),
			},
		}
		for _, k := range nodes {
			edge.Target.Nodes = append(edge.Target.Nodes, k)
		}
		_spec.Edges.Clear = append(_spec.Edges.Clear, edge)
	}
	if nodes := dluo.mutation.TagsIDs(); len(nodes) > 0 {
		edge := &sqlgraph.EdgeSpec{
			Rel:     sqlgraph.M2M,
			Inverse: false,
			Table:   dblink.TagsTable,
			Columns: dblink.TagsPrimaryKey,
			Bidi:    false,
			Target: &sqlgraph.EdgeTarget{
				IDSpec: sqlgraph.NewFieldSpec(dbtag.FieldID, field.TypeString),
			},
		}
		for _, k := range nodes {
			edge.Target.Nodes = append(edge.Target.Nodes, k)
		}
		_spec.Edges.Add = append(_spec.Edges.Add, edge)
	}
	_node = &DbLink{config: dluo.config}
	_spec.Assign = _node.assignValues
	_spec.ScanValues = _node.scanValues
	if err = sqlgraph.UpdateNode(ctx, dluo.driver, _spec); err != nil {
		if _, ok := err.(*sqlgraph.NotFoundError); ok {
			err = &NotFoundError{dblink.Label}
		} else if sqlgraph.IsConstraintError(err) {
			err = &ConstraintError{msg: err.Error(), wrap: err}
		}
		return nil, err
	}
	dluo.mutation.done = true
	return _node, nil
}
