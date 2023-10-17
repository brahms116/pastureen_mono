package main

import (
	"context"
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

	return models.Link{
		Id:          item.ID.String(),
		Url:         item.URL,
		Title:       item.Title,
		Description: item.Description,
		Tags:        tags,
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

	links, err := q.All(ctx)

	if err != nil {
		return []models.Link{}, err
	}

	result := make([]models.Link, len(links))
	for i, link := range links {
		result[i] = DbLinkToModelLink(link)
	}

	return result, nil
}
