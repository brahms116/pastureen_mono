package schema

import (
	"entgo.io/ent"
	"entgo.io/ent/schema/field"
	"entgo.io/ent/schema/edge"
	"github.com/google/uuid"
)

// DbTag holds the schema definition for the DbTag entity.
type DbTag struct {
	ent.Schema
}

// Fields of the DbTag.
func (DbTag) Fields() []ent.Field {
  return []ent.Field{
		field.UUID("id", uuid.UUID{}).Default(uuid.New).Unique(),
    field.String("name"),
  }
}

// Edges of the DbTag.
func (DbTag) Edges() []ent.Edge {
  return []ent.Edge{
    edge.From("links", DbLink.Type).Ref("tags"),
  }
}
