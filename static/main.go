package main

import (
	"github.com/gin-gonic/gin"
)

func main() {
  r := gin.Default()
  r.Static("/", "./content")
  r.Run(":8081")
}
