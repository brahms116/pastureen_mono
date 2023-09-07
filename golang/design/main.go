package main

import (
	"bytes"
	"github.com/gin-gonic/gin"
)

import "embed"
import "html/template"
import "net/http"

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

type ListItemActionProps struct {
	ActionType      string
	ActionText      string
	ActionLink      string
	ActionIndicator string
	ActionTarget    string
}

type HtmxListItemActionConfig struct {
	ActionText      string
	ActionIndicator string
	ActionTarget    string
	ActionLink      string
}

type UrlListItemActionConfig struct {
	ActionLink string
	ActionText string
}

func (c *HtmxListItemActionConfig) ActionType() string {
	return "htmx"
}

func (c *UrlListItemActionConfig) ActionType() string {
	return "url"
}

var _ ListItemActionConfig = &UrlListItemActionConfig{}

func ListItemActionConfigToProps(c *ListItemActionConfig) ListItemActionProps {
	switch v := (*c).(type) {
	case *HtmxListItemActionConfig:
		return ListItemActionProps{
			ActionType:      "htmx",
			ActionText:      v.ActionText,
			ActionIndicator: v.ActionIndicator,
			ActionTarget:    v.ActionTarget,
			ActionLink:      v.ActionLink,
		}
	case *UrlListItemActionConfig:
		return ListItemActionProps{
			ActionType: "url",
			ActionLink: v.ActionLink,
			ActionText: v.ActionText,
		}
	}
	panic("unknown list action type")
}

type ListItemActionConfig interface {
	ActionType() string
}

type ListItemProps struct {
	ImageSrc string
	ImageAlt string
	Title    string
	Subtitle string
	Actions  []ListItemActionProps
	Link     string
}

type ListPageProps struct {
	ListItems []ListItemProps
}

func main() {
	r := gin.Default()

	indexTemplate := template.Must(template.ParseFS(f, "templates/pages/index.html", "templates/layout/*.html", "templates/components/*.html"))
	formsTemplate := template.Must(template.ParseFS(f, "templates/pages/forms.html", "templates/layout/*.html", "templates/components/*.html"))
	listsTemplate := template.Must(template.ParseFS(f, "templates/pages/lists.html", "templates/layout/*.html", "templates/components/*.html"))

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
		var _ ListItemActionConfig = &UrlListItemActionConfig{
			ActionLink: "/",
			ActionText: "Home",
		}

		listItem := ListItemProps{
			ImageSrc: "/static/assets/light.png",
			ImageAlt: "Light",
			Title:    "Light",
			Subtitle: "A light",
			Actions: []ListItemActionProps{
				// ListItemActionConfigToProps(&urlListItemActionConfig),
			},
		}

		if err := listsTemplate.ExecuteTemplate(&buffer, "lists.html",
			PageProps{
				Title:       "Lists",
				TopbarProps: props,
				BodyProps: ListPageProps{
					ListItems: []ListItemProps{
            listItem,
            listItem,
            listItem,
            listItem,
            listItem,
            listItem,
					},
				},
			},
		); err != nil {
			c.Error(err)
		}
		c.Data(http.StatusOK, "text/html; charset=utf-8", buffer.Bytes())
	})

	r.Run()
}
