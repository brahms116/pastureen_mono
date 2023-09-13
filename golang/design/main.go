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

type LogoProps struct {
	LogoText string
	LogoLink string
}

type NavItemProps struct {
	Link              string
	Text              string
	Id                string
	ShouldHaveDivider bool
	ShouldBeActive    bool
}

type TopbarProps struct {
	LogoProps     LogoProps
	NavItems      []NavItemProps
	MenuOpen      bool
	CurrentPageId string
}

type PageProps struct {
	Title       string
	TopbarProps TopbarProps
	BodyProps   interface{}
}

func GetTopbarProps(page string) TopbarProps {
	return TopbarProps{
		LogoProps: LogoProps{
			LogoText: "Logo",
			LogoLink: config.BASE_URL + "/",
		},
		NavItems: []NavItemProps{
			{
				Link:              config.BASE_URL + "/",
				Text:              "Home",
				ShouldHaveDivider: true,
				ShouldBeActive:    page == "home",
			},
			{
				Link:              config.BASE_URL + "/buttons",
				Text:              "Forms",
				ShouldHaveDivider: true,
				ShouldBeActive:    page == "forms",
			},
			{
				Link:              config.BASE_URL + "/lists",
				Text:              "Lists",
				ShouldHaveDivider: true,
				ShouldBeActive:    page == "lists",
			},
			{
				Link:              config.BASE_URL + "/color",
				Text:              "Color",
				ShouldHaveDivider: true,
				ShouldBeActive:    page == "color",
			},
			{
				Link:           config.BASE_URL + "/typography",
				Text:           "Type",
				ShouldBeActive: page == "typography",
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

type ActionItemProps struct {
	ActionType      string
	ActionText      string
	ActionLink      string
	ActionIndicator string
	ActionTarget    string
}

type HtmxActionItemConfig struct {
	ActionText      string
	ActionIndicator string
	ActionTarget    string
	ActionLink      string
}

type UrlActionItemConfig struct {
	ActionLink string
	ActionText string
}

func (c *HtmxActionItemConfig) ActionType() string {
	return "htmx"
}

func (c *UrlActionItemConfig) ActionType() string {
	return "url"
}

var _ ActionItemConfig = &UrlActionItemConfig{}

func ActionItemConfigToProps(c *ActionItemConfig) ActionItemProps {
	switch v := (*c).(type) {
	case *HtmxActionItemConfig:
		return ActionItemProps{
			ActionType:      "htmx",
			ActionText:      v.ActionText,
			ActionIndicator: v.ActionIndicator,
			ActionTarget:    v.ActionTarget,
			ActionLink:      v.ActionLink,
		}
	case *UrlActionItemConfig:
		return ActionItemProps{
			ActionType: "url",
			ActionLink: v.ActionLink,
			ActionText: v.ActionText,
		}
	}
	panic("unknown list action type")
}

type ActionItemConfig interface {
	ActionType() string
}

type ListItemProps struct {
	ImageSrc string
	ImageAlt string
	Title    string
	Subtitle string
	Actions  []ActionItemProps
	Tags     []string
	Link     string
}

type ListsPageProps struct {
	Paginator ListsPagePaginatorProps
	Actions   []ActionItemProps
}

type ListsPagePaginatorProps struct {
	PaginatorRequestUrl string
}

type ListsPageListResponseProps struct {
	Items            []ListItemProps
	PaginatorProps   ListsPagePaginatorProps
	PaginatorPresent bool
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

func FakeDataItemToListItemProps(item FakeDataItem) ListItemProps {
	var actionConfig ActionItemConfig = &UrlActionItemConfig{
		ActionLink: config.BASE_URL + "/",
		ActionText: "Fire me",
	}
	return ListItemProps{
		ImageSrc: config.BASE_URL + "/static/assets/logo.png",
		ImageAlt: "Logo",
		Title:    item.Title,
		Subtitle: item.Subtitle,
		Tags:     item.Tags,
		Actions: []ActionItemProps{
			ActionItemConfigToProps(&actionConfig),
		},
	}
}

func FakeDataItemsToListItemProps(items []FakeDataItem) []ListItemProps {
	var result []ListItemProps
	for _, item := range items {
		result = append(result, FakeDataItemToListItemProps(item))
	}
	return result
}

func ListItemPropsToListsPageResponseProps(
	items []ListItemProps,
	searchString string, cursor int, isLastPage bool,
) ListsPageListResponseProps {
	var paginatorProps ListsPagePaginatorProps
	if !isLastPage {
		paginatorProps = ListsPagePaginatorProps{
			PaginatorRequestUrl: config.BASE_URL + fmt.Sprintf("htmx/lists_page_list?search=%s&cursor=%d", searchString, cursor),
		}
	}
	return ListsPageListResponseProps{
		Items:            items,
		PaginatorProps:   paginatorProps,
		PaginatorPresent: !isLastPage,
	}
}

func main() {

	r := gin.Default()

	indexTemplate := template.Must(template.ParseFS(f, "templates/pages/index.html", "templates/layout/*.html", "templates/page_components/*.html", "templates/components/*.html"))
	formsTemplate := template.Must(template.ParseFS(f, "templates/pages/forms.html", "templates/layout/*.html", "templates/page_components/*.html", "templates/components/*.html"))
	listsTemplate := template.Must(template.ParseFS(f, "templates/pages/lists.html", "templates/layout/*.html", "templates/page_components/*.html", "templates/components/*.html"))

	r.StaticFS("/static", http.FS(assetsFS))

	r.GET("/healthcheck", func(c *gin.Context) {
		c.String(http.StatusOK, "OK")
	})

	r.GET("/", func(c *gin.Context) {
		props := GetTopbarProps("home")
		var buffer bytes.Buffer
		if err := indexTemplate.ExecuteTemplate(&buffer, "index.html", PageProps{
			Title:       "Home",
			TopbarProps: props,
		}); err != nil {
			c.Error(err)
		}
		c.Data(http.StatusOK, "text/html; charset=utf-8", buffer.Bytes())
	})

	r.GET("/forms", func(c *gin.Context) {
		props := GetTopbarProps("forms")
		var buffer bytes.Buffer
		if err := formsTemplate.ExecuteTemplate(&buffer, "forms.html", PageProps{
			Title:       "Forms",
			TopbarProps: props,
		},
		); err != nil {
			c.Error(err)
		}
		c.Data(http.StatusOK, "text/html; charset=utf-8", buffer.Bytes())
	})

	r.GET("/lists", func(c *gin.Context) {
		props := GetTopbarProps("lists")
		var buffer bytes.Buffer

		var addNewPersonActionConfig ActionItemConfig = &UrlActionItemConfig{
			ActionLink: "/lists",
			ActionText: "Add New Person",
		}

		if err := listsTemplate.ExecuteTemplate(&buffer, "lists.html",
			PageProps{
				Title:       "Lists",
				TopbarProps: props,
				BodyProps: ListsPageProps{
					Paginator: ListsPagePaginatorProps{
						PaginatorRequestUrl: config.BASE_URL + "/htmx/lists_page_list?cursor=0",
					},
					Actions: []ActionItemProps{
						ActionItemConfigToProps(&addNewPersonActionConfig),
					},
				},
			},
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
		if err := listsTemplate.ExecuteTemplate(&buffer, "lists_page_list_response.html", responseProps); err != nil {
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
		if err := listsTemplate.ExecuteTemplate(&buffer, "lists_page_list_response.html", responseProps); err != nil {
			c.Error(err)
		}
		c.Data(http.StatusOK, "text/html; charset=utf-8", buffer.Bytes())
	})

	r.Run()
}
