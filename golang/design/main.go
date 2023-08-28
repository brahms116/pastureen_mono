package main

import (
	"github.com/gin-gonic/gin"
)

import "embed"
import "html/template"
import "net/http"

type LayoutProps struct {
  Title string
}

type IndexProps struct {
  LayoutProps LayoutProps
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

  r.GET("/", func (c *gin.Context) {
    c.HTML(http.StatusOK, "index.html", IndexProps{
      LayoutProps: LayoutProps{
        Title: "Hello World",
      },
    })
  })

	r.Run()
}
