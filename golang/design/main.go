package main

import (
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

func GetTopbarProps(page string, menuOpen bool) TopbarProps {
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
				Link:           "/about",
				Text:           "About",
				ShouldBeActive: page == "about",
			},
		},
		MenuOpen: menuOpen,
	}
}

//go:embed templates/*
var f embed.FS

//go:embed assets/*
var assetsFS embed.FS

func main() {
	r := gin.Default()

	html := template.Must(template.ParseFS(f, "templates/**/*"))
	r.SetHTMLTemplate(html)

	r.StaticFS("/static", http.FS(assetsFS))

	r.GET("/", func(c *gin.Context) {
		props := GetTopbarProps("home", false)
		c.HTML(http.StatusOK, "index.html", IndexProps{
			LayoutProps: LayoutProps{
				Title:       "Hello World",
				TopbarProps: props,
			},
		})
	})

	r.GET("/htmx/topbar", func(c *gin.Context) {
		isOpen := c.Query("state") == "open"
		props := GetTopbarProps("home", isOpen)
		c.HTML(http.StatusOK, "topbar.html", props)
	})

	r.Run()
}
