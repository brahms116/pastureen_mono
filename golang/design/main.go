package main

import (
	"bytes"
	"embed"
	"encoding/json"
	"fmt"
	"github.com/gin-gonic/gin"
	"html/template"
	"net/http"
	"os"
	"pastureen/components"
	"pastureen/styles"
	"strconv"
	"strings"
)

var config = GetConfigFromEnv()

type ApplicationConfig struct {
	BASE_URL string
}

func GetConfigFromEnv() ApplicationConfig {
	return ApplicationConfig{
		BASE_URL: os.Getenv("DESIGN_SYSTEM_BASE_URL"),
	}
}

func GetTopbarProps(page string, baseUrl string) components.TopbarProps {
	return components.TopbarProps{
		LogoSrc:  baseUrl + "/static/assets/logo.png",
		LogoLink: baseUrl + "/",
		LogoText: "PastureenDesign",
		NavItemsProps: []components.NavItemProps{
			{
				Link:     config.BASE_URL + "/",
				Text:     "Home",
				IsActive: page == "home",
			},
			{
				Link:     config.BASE_URL + "/forms",
				Text:     "Forms",
				IsActive: page == "forms",
			},
			{
				Link:     config.BASE_URL + "/lists",
				Text:     "Lists",
				IsActive: page == "lists",
			},
			{
				Link:     config.BASE_URL + "/color",
				Text:     "Color",
				IsActive: page == "color",
			},
			{
				Link: config.BASE_URL + "/typography",
				Text: "Type",
			},
		},
	}
}

//go:embed templates/*
var f embed.FS

//go:embed assets/*
var assetsFS embed.FS

//go:embed fake_data.json
var fakeData []byte

type FakeDataItem struct {
	Title    string   `json:"title"`
	Subtitle string   `json:"subtitle"`
	Tags     []string `json:"tags"`
}

type FakeData struct {
	Items []FakeDataItem `json:"items"`
}

type ListsPagePaginatorData struct {
	PaginatorRequestUrl string
  IsInitialLoad bool
}

type ListsPagePaginatorProps struct {
	PaginatorRequestUrl string
  IsInitialLoad bool
}

func (p ListsPagePaginatorProps) ToData() ListsPagePaginatorData {
	return ListsPagePaginatorData{
		PaginatorRequestUrl: p.PaginatorRequestUrl,
    IsInitialLoad: p.IsInitialLoad,
	}
}

type ListsPageProps struct {
	PaginatorProps ListsPagePaginatorProps
	ActionsProps   components.ActionMenuProps
	SearchUrl      string
}

type ListsPageData struct {
	PaginatorData ListsPagePaginatorData
	ActionsData   components.ActionMenuData
	SearchUrl     string
}

func (p ListsPageProps) ToData() ListsPageData {
	return ListsPageData{
		PaginatorData: p.PaginatorProps.ToData(),
		ActionsData:   p.ActionsProps.ToData(),
		SearchUrl:     p.SearchUrl,
	}
}

type IndexPageData struct {
	LandingImageSrc string
}

type IndexPageProps struct {
	LandingImageSrc string
}

func (p IndexPageProps) ToData() IndexPageData {
	return IndexPageData{
		LandingImageSrc: p.LandingImageSrc,
	}
}

type ListsPageListResponseData struct {
	ItemsData        []components.ListItemData
	PaginatorData    ListsPagePaginatorData
	PaginatorPresent bool
}

type ListsPageListResponseProps struct {
	ItemsProps       []components.ListItemProps
	PaginatorProps   ListsPagePaginatorProps
	PaginatorPresent bool
}

func (p ListsPageListResponseProps) ToData() ListsPageListResponseData {
	itemsData := make([]components.ListItemData, len(p.ItemsProps))
	for i, itemProps := range p.ItemsProps {
		itemsData[i] = itemProps.ToData()
	}
	return ListsPageListResponseData{
		ItemsData:        itemsData,
		PaginatorData:    p.PaginatorProps.ToData(),
		PaginatorPresent: p.PaginatorPresent,
	}
}

type QueryResult struct {
	Items          []FakeDataItem
	IsLastPage     bool
	NextPageCursor int
}

func QueryFakeData(searchString string, cursor int) (QueryResult, error) {
	var data FakeData
	if err := json.Unmarshal(fakeData, &data); err != nil {
		return QueryResult{}, err
	}

	var items []FakeDataItem

	if searchString == "" {
		items = data.Items
	} else {
		for _, item := range data.Items {
			upperCaseSearchString := strings.ToUpper(searchString)
			upperCaseTitle := strings.ToUpper(item.Title)
			if strings.Contains(upperCaseTitle, upperCaseSearchString) {
				items = append(items, item)
			}
		}
	}

	if len(items) == 0 {
		return QueryResult{
			IsLastPage: true,
		}, nil
	}

	numPerPage := 10
	totalItems := len(items)
	nextPageCursor := cursor + numPerPage
	isLastPage := nextPageCursor >= totalItems-1
	var pageEnd int

	if isLastPage {
		pageEnd = totalItems - 1
	} else {
		pageEnd = nextPageCursor
	}
	responseItems := items[cursor:pageEnd]

	return QueryResult{
		Items:          responseItems,
		IsLastPage:     isLastPage,
		NextPageCursor: nextPageCursor,
	}, nil
}

func FakeDataItemToListItemProps(item FakeDataItem) components.ListItemProps {
	var actionConfig components.ActionItemProps = &components.UrlActionItemProps{
		ActionLink: config.BASE_URL + "/",
		ActionText: "Fire me",
	}
	return components.ListItemProps{
		ImageSrc: config.BASE_URL + "/static/assets/logo.png",
		ImageAlt: "Logo",
		Title:    item.Title,
		Subtitle: item.Subtitle,
		Tags:     item.Tags,
		ActionMenuProps: components.ActionMenuProps{
			ItemsProps: []components.ActionItemProps{
				actionConfig,
			},
		},
	}
}

func FakeDataItemsToListItemProps(items []FakeDataItem) []components.ListItemProps {
	var result []components.ListItemProps
	for _, item := range items {
		result = append(result, FakeDataItemToListItemProps(item))
	}
	return result
}

func ListItemPropsToListsPageResponseProps(
	items []components.ListItemProps,
	searchString string, cursor int, isLastPage bool,
) ListsPageListResponseProps {
	var paginatorProps ListsPagePaginatorProps
	if !isLastPage {
		paginatorProps = ListsPagePaginatorProps{
			PaginatorRequestUrl: config.BASE_URL + fmt.Sprintf("/htmx/lists_page_list?search=%s&cursor=%d", searchString, cursor),
		}
	}
	return ListsPageListResponseProps{
		ItemsProps:       items,
		PaginatorProps:   paginatorProps,
		PaginatorPresent: !isLastPage,
	}
}

func main() {

	r := gin.Default()

	componentTemplate := components.GetTemplate()
	designSystemTemplate := template.Must(componentTemplate.ParseFS(f, "templates/page_components/*.html"))

	indexTemplate := template.Must(designSystemTemplate.Clone())
	indexTemplate = template.Must(indexTemplate.ParseFS(f, "templates/pages/index.html"))

	formsTemplate := template.Must(designSystemTemplate.Clone())
	formsTemplate = template.Must(formsTemplate.ParseFS(f, "templates/pages/forms.html"))

	listsTemplate := template.Must(designSystemTemplate.Clone())
	listsTemplate = template.Must(listsTemplate.ParseFS(f, "templates/pages/lists.html"))

	r.StaticFS("/static", http.FS(assetsFS))

	r.GET("/embed/styles.css", func(c *gin.Context) {
		c.Data(http.StatusOK, "text/css; charset=utf-8", styles.PastureenCss)
	})

	r.GET("/healthcheck", func(c *gin.Context) {
		c.String(http.StatusOK, "OK")
	})

	r.GET("/", func(c *gin.Context) {
		props := GetTopbarProps("home", config.BASE_URL)
		var buffer bytes.Buffer
		if err := indexTemplate.ExecuteTemplate(&buffer, "index.html", components.LayoutProps{
			Title:               "Home",
			TopbarProps:         props,
			StylesheetUri:       config.BASE_URL + "/embed/styles.css",
			CustomStylesheetUri: config.BASE_URL + "/static/assets/design-system.css",
			BodyData: IndexPageProps{
				LandingImageSrc: config.BASE_URL + "/static/assets/light.png",
			}.ToData(),
		}.ToData()); err != nil {
			c.Error(err)
		}
		c.Data(http.StatusOK, "text/html; charset=utf-8", buffer.Bytes())
	})

	r.GET("/forms", func(c *gin.Context) {
		props := GetTopbarProps("forms", config.BASE_URL)
		var buffer bytes.Buffer
		if err := formsTemplate.ExecuteTemplate(&buffer, "forms.html", components.LayoutProps{
			Title:               "Forms",
			StylesheetUri:       config.BASE_URL + "/embed/styles.css",
			CustomStylesheetUri: config.BASE_URL + "/static/assets/design-system.css",
			TopbarProps:         props,
		}.ToData(),
		); err != nil {
			c.Error(err)
		}
		c.Data(http.StatusOK, "text/html; charset=utf-8", buffer.Bytes())
	})

	r.GET("/lists", func(c *gin.Context) {
		props := GetTopbarProps("lists", config.BASE_URL)
		var buffer bytes.Buffer

		var addNewPersonActionConfig components.ActionItemProps = &components.UrlActionItemProps{
			ActionLink: "/lists",
			ActionText: "Add New Person",
		}

		if err := listsTemplate.ExecuteTemplate(&buffer, "lists.html",
			components.LayoutProps{
				Title:               "Lists",
				TopbarProps:         props,
				StylesheetUri:       config.BASE_URL + "/embed/styles.css",
				CustomStylesheetUri: config.BASE_URL + "/static/assets/design-system.css",
				BodyData: ListsPageProps{
					PaginatorProps: ListsPagePaginatorProps{
						PaginatorRequestUrl: config.BASE_URL + "/htmx/lists_page_list?cursor=0",
            IsInitialLoad: true,
					},
					ActionsProps: components.ActionMenuProps{
						ItemsProps: []components.ActionItemProps{
							addNewPersonActionConfig,
						},
					},
					SearchUrl: config.BASE_URL + "/htmx/lists_page_search",
				}.ToData(),
			}.ToData(),
		); err != nil {
			c.Error(err)
		}
		c.Data(http.StatusOK, "text/html; charset=utf-8", buffer.Bytes())
	})

	r.GET("/htmx/lists_page_list", func(c *gin.Context) {
		var buffer bytes.Buffer
		searchString := c.Query("search")
		cursor, err := strconv.Atoi(c.Query("cursor"))
		if err != nil {
			c.Error(err)
		}
		result, err := QueryFakeData(searchString, cursor)
		if err != nil {
			c.Error(err)
		}
		itemsProps := FakeDataItemsToListItemProps(result.Items)
		responseProps := ListItemPropsToListsPageResponseProps(itemsProps, searchString, result.NextPageCursor, result.IsLastPage)
		if err := listsTemplate.ExecuteTemplate(&buffer, "lists_page_list_response.html", responseProps.ToData()); err != nil {
			c.Error(err)
		}
		c.Data(http.StatusOK, "text/html; charset=utf-8", buffer.Bytes())
	})

	r.POST("/htmx/lists_page_search", func(c *gin.Context) {
		var buffer bytes.Buffer
		queryString := c.PostForm("search")
		result, err := QueryFakeData(queryString, 0)
		if err != nil {
			c.Error(err)
		}
		itemsProps := FakeDataItemsToListItemProps(result.Items)
		responseProps := ListItemPropsToListsPageResponseProps(itemsProps, queryString, result.NextPageCursor, result.IsLastPage)
		if err := listsTemplate.ExecuteTemplate(&buffer, "lists_page_list_response.html", responseProps.ToData()); err != nil {
			c.Error(err)
		}
		c.Data(http.StatusOK, "text/html; charset=utf-8", buffer.Bytes())
	})

	r.Run()
}
