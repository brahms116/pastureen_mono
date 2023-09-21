package main

import (
	"github.com/gin-gonic/gin"
)

func main() {
  r := gin.Default()
  r.Static("/", "./assets")
  r.Run(":8081")
}
