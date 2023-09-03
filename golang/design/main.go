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
				Link:           "/forms",
				Text:           "Forms",
				ShouldBeActive: page == "forms",
			},
		},
	}
}

//go:embed templates/*
var f embed.FS

//go:embed assets/*
var assetsFS embed.FS

func main() {
	r := gin.Default()

	indexTemplate := template.Must(template.ParseFS(f, "templates/pages/index.html", "templates/layout/*.html", "templates/components/*.html"))
	formsTemplate := template.Must(template.ParseFS(f, "templates/pages/forms.html", "templates/layout/*.html", "templates/components/*.html"))

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

	r.Run()
}
