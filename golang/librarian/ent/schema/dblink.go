package schema

import (
	"entgo.io/ent"
	"entgo.io/ent/schema/field"
	"entgo.io/ent/schema/edge"
	"entgo.io/ent/schema/index"
	"github.com/google/uuid"
)

// DbLink holds the schema definition for the DbLink entity.
type DbLink struct {
	ent.Schema
}

// Fields of the DbLink.
func (DbLink) Fields() []ent.Field {
	return []ent.Field{
		field.UUID("id", uuid.UUID{}).Default(uuid.New).Unique(),
		field.String("title"),
		field.Time("date"),
		field.String("url"),
		field.String("subtitle"),
		field.String("description"),
		field.String("image_url").Optional(),
		field.String("image_alt").Optional(),
	}
}

// Indexes of the DbLink.
func (DbLink) Indexes() []ent.Index {
  return []ent.Index{
    index.Fields("date"),
  }
}

// Edges of the DbLink.
func (DbLink) Edges() []ent.Edge {
  return []ent.Edge{
    edge.To("tags", DbTag.Type),
  }
}
