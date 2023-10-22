package main

import (
	"context"
	"entgo.io/ent/dialect/sql"
	"github.com/google/uuid"
	"pastureen/librarian-models"
	"pastureen/librarian/ent"
	"pastureen/librarian/ent/dblink"
	"pastureen/librarian/ent/dbtag"
	"time"
)

func DbLinkToModelLink(item *ent.DbLink) models.Link {
	tags := make([]string, len(item.Edges.Tags))
	for i, tag := range item.Edges.Tags {
		tags[i] = tag.ID
	}

	date := item.Date.Format("2006-01-02")

	return models.Link{
		Id:          item.ID.String(),
		Url:         item.URL,
		Title:       item.Title,
		Description: item.Description,
		Tags:        tags,
		Date:        date,
	}
}

func QueryLinks(query *models.QueryLinksRequest, client *ent.Client, ctx context.Context) ([]models.Link, error) {

	limit := 50

	q := client.DbLink.Query().WithTags()

	if query.Tags != nil {
		q = q.Where(dblink.HasTagsWith(dbtag.IDIn(query.Tags...)))
	}

	if query.StartDate != "" {
		date, err := time.Parse("2006-01-02", query.StartDate)
		if err != nil {
			return []models.Link{}, err
		}
		q = q.Where(dblink.DateGTE(date))
	}

	if query.EndDate != "" {
		date, err := time.Parse("2006-01-02", query.EndDate)
		if err != nil {
			return []models.Link{}, err
		}
		q = q.Where(dblink.DateLTE(date))
	}

	if query.TitleQuery != "" {
		q = q.Where(dblink.TitleContainsFold(query.TitleQuery))
	}

	if query.Limit != 0 {
		limit = query.Limit
	}
	q = q.Limit(limit)

	if query.Page != 0 {
		q = q.Offset((query.Page - 1) * limit)
	}

	links, err := q.Order(dblink.ByDate(sql.OrderDesc())).All(ctx)

	if err != nil {
		return []models.Link{}, err
	}

	result := make([]models.Link, len(links))
	for i, link := range links {
		result[i] = DbLinkToModelLink(link)
	}

	return result, nil
}

func containsTag(s []*ent.DbTag, e string) bool {
	for _, a := range s {
		if a.ID == e {
			return true
		}
	}
	return false
}

func QueryTagNames(client *ent.Client, ctx context.Context) ([]string, error) {
	tags, err := client.DbTag.Query().All(ctx)
	if err != nil {
		return []string{}, err
	}

	result := make([]string, len(tags))
	for i, tag := range tags {
		result[i] = tag.ID
	}

	return result, nil
}

func DeleteLink(id uuid.UUID, client *ent.Client, ctx context.Context) error {
	_, err := client.DbLink.Delete().Where(dblink.ID(id)).Exec(ctx)
	return err
}

func PepareDbLink(link models.Link, client *ent.Client, ctx context.Context) (*ent.DbLinkCreate, error) {

	// Find all the existingDbTags
	existingDbTags, err := client.DbTag.Query().All(ctx)
	if err != nil {
		return &ent.DbLinkCreate{}, err
	}

	dbTagsToAssociate := make([]*ent.DbTag, len(link.Tags))

	// Find all the tags
	for _, tag := range link.Tags {
		if !containsTag(existingDbTags, tag) {
			tag, err := client.DbTag.Create().SetID(tag).Save(ctx)
			if err != nil {
				return &ent.DbLinkCreate{}, err
			}
			dbTagsToAssociate = append(dbTagsToAssociate, tag)
		} else {
			for _, t := range existingDbTags {
				if t.ID == tag {
					dbTagsToAssociate = append(dbTagsToAssociate, t)
				}
			}
		}
	}

	linkDate := time.Now()

	if link.Date != "" {
		linkDate, err = time.Parse("2006-01-02", link.Date)
		if err != nil {
			linkDate = time.Now()
		}
	}

	var nillableImageAlt *string = nil
	var nillableImageUrl *string = nil

	if link.ImageAlt != "" {
		nillableImageAlt = &link.ImageAlt
	}

	if link.ImageUrl != "" {
		nillableImageUrl = &link.ImageUrl
	}

	newDbLink := client.DbLink.Create().
		SetDate(linkDate).
		SetDescription(link.Description).
		SetURL(link.Url).
		SetSubtitle(link.Subtitle).
		SetTitle(link.Title).
		SetNillableImageAlt(nillableImageAlt).
		SetNillableImageURL(nillableImageUrl).
		AddTags(dbTagsToAssociate...)

  return newDbLink, nil
}
