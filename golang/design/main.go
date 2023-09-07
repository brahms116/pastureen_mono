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

type LayoutProps struct {
	Title       string
	TopbarProps TopbarProps
}

type IndexProps struct {
	LayoutProps LayoutProps
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
}

type UrlListItemActionConfig struct {
	ActionLink string
}

func (c *HtmxListItemActionConfig) ActionType() string {
	return "htmx"
}

func (c *UrlListItemActionConfig) ActionType() string {
	return "url"
}

func ListItemActionConfigToProps(c *ListItemActionConfig) ListItemActionProps {
	switch v := (*c).(type) {
	case *HtmxListItemActionConfig:
		return ListItemActionProps{
			ActionType:      "htmx",
			ActionText:      v.ActionText,
			ActionIndicator: v.ActionIndicator,
			ActionTarget:    v.ActionTarget,
		}
  case *UrlListItemActionConfig:
    return ListItemActionProps{
      ActionType: "url",
      ActionLink: v.ActionLink,
    }
	}
  return ListItemActionProps{}
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
		if err := indexTemplate.ExecuteTemplate(&buffer, "index.html", IndexProps{
			LayoutProps: LayoutProps{
				Title:       "Home",
				TopbarProps: props,
			},
		}); err != nil {
			c.Error(err)
		}
		c.Data(http.StatusOK, "text/html; charset=utf-8", buffer.Bytes())
	})

	r.GET("/forms", func(c *gin.Context) {
		props := GetTopbarProps("forms")
		var buffer bytes.Buffer
		if err := formsTemplate.ExecuteTemplate(&buffer, "forms.html", IndexProps{
			LayoutProps: LayoutProps{
				Title:       "Forms",
				TopbarProps: props,
			},
		}); err != nil {
			c.Error(err)
		}
		c.Data(http.StatusOK, "text/html; charset=utf-8", buffer.Bytes())
	})

	r.GET("/lists", func(c *gin.Context) {
		props := GetTopbarProps("lists")
		var buffer bytes.Buffer
		if err := listsTemplate.ExecuteTemplate(&buffer, "lists.html", IndexProps{
			LayoutProps: LayoutProps{
				Title:       "Lists",
				TopbarProps: props,
			},
		}); err != nil {
			c.Error(err)
		}
		c.Data(http.StatusOK, "text/html; charset=utf-8", buffer.Bytes())
	})

	r.Run()
}
