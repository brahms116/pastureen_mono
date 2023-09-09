package main

import (
	"bytes"
	"embed"
	"encoding/json"
	"fmt"
	"html/template"
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
)

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
			LogoLink: "/",
		},
		NavItems: []NavItemProps{
			{
				Link:              "/",
				Text:              "Home",
				ShouldHaveDivider: true,
				ShouldBeActive:    page == "home",
			},
			{
				Link:              "/forms",
				Text:              "Forms",
				ShouldHaveDivider: true,
				ShouldBeActive:    page == "forms",
			},
			{
				Link:           "/lists",
				Text:           "Lists",
				ShouldBeActive: page == "lists",
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

func main() {
	r := gin.Default()

	indexTemplate := template.Must(template.ParseFS(f, "templates/pages/index.html", "templates/layout/*.html", "templates/page_components/*.html", "templates/components/*.html"))
	formsTemplate := template.Must(template.ParseFS(f, "templates/pages/forms.html", "templates/layout/*.html", "templates/page_components/*.html", "templates/components/*.html"))
	listsTemplate := template.Must(template.ParseFS(f, "templates/pages/lists.html", "templates/layout/*.html", "templates/page_components/*.html", "templates/components/*.html"))

	r.StaticFS("/static", http.FS(assetsFS))

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
						PaginatorRequestUrl: "/htmx/lists_page_list?cursor=0",
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
		var data FakeData

		if err := json.Unmarshal(fakeData, &data); err != nil {
			c.Error(err)
		}

		numPerPage := 10

		cursor, err := strconv.Atoi(c.Query("cursor"))
		if err != nil {
			c.Error(err)
		}

		totalItems := len(data.Items)
		nextCursor := cursor + numPerPage
		isLastPage := nextCursor >= totalItems-1

		var pageEnd int
		if isLastPage {
			pageEnd = totalItems - 1
		} else {
			pageEnd = nextCursor
		}

		responseItems := data.Items[cursor:pageEnd]
		var responseItemsProps []ListItemProps
		for _, item := range responseItems {
			responseItemsProps = append(responseItemsProps, ListItemProps{
				ImageSrc: "/static/assets/logo.png",
				ImageAlt: "Logo",
				Title:    item.Title,
				Subtitle: item.Subtitle,
				Tags:     item.Tags,
			})
		}

		if err := listsTemplate.ExecuteTemplate(&buffer, "lists_page_list_response.html", ListsPageListResponseProps{
			Items: responseItemsProps,
			PaginatorProps: ListsPagePaginatorProps{
				PaginatorRequestUrl: fmt.Sprintf("/htmx/lists_page_list?cursor=%d", nextCursor),
			},
			PaginatorPresent: !isLastPage,
		}); err != nil {
			c.Error(err)
		}

    c.Data(http.StatusOK, "text/html; charset=utf-8", buffer.Bytes())
	})

	r.Run()
}
