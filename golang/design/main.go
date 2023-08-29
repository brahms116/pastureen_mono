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

type NavItem struct {
	Link string
	Text string
	Id   string
}

type TopBarProps struct {
	LogoProps     LogoProps
	NavItems      []NavItem
	MenuOpen      bool
	CurrentPageId string
}

type LayoutProps struct {
	Title string
}

type IndexProps struct {
	LayoutProps LayoutProps
}

func GetTopBarProps (pageId string) (TopBarProps) {
  return TopBarProps{
    LogoProps: LogoProps{
      LogoText: "Logo",
      LogoLink: "/",
    },
    NavItems: []NavItem{
      {
        Link: "/",
        Text: "Home",
        Id: "home",
      },
      {
        Link: "/about",
        Text: "About",
        Id: "about",
      },
    },
    MenuOpen: false,
    CurrentPageId: pageId,
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
		c.HTML(http.StatusOK, "index.html", IndexProps{
			LayoutProps: LayoutProps{
				Title: "Hello World",
			},
		})
	})

	r.Run()
}
