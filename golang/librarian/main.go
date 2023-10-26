package main

import (
	"context"
	"log"
	"os"
	authClient "pastureen/auth-client"
	models "pastureen/librarian-models"
	"pastureen/librarian/ent"
	"strings"

	"github.com/gin-gonic/gin"
	_ "github.com/lib/pq"
)

type LibrarianConfig struct {
	DbConnectionStr string
	BlogBucketName  string
	ListenAddr      string
	AuthUrl         string
	AdminEmail      string
}

func ConfigFromEnv() LibrarianConfig {
	return LibrarianConfig{
		DbConnectionStr: os.Getenv("LIBRARIAN_DB_CONN_STR"),
		BlogBucketName:  os.Getenv("LIBRARIAN_BLOG_BUCKET_NAME"),
		ListenAddr:      os.Getenv("LIBRARIAN_LISTEN_ADDR"),
		AuthUrl:         os.Getenv("LIBRARIAN_AUTH_URL"),
		AdminEmail:      os.Getenv("LIBRARIAN_ADMIN_EMAIL"),
	}
}

func Authenticate(authUrl string, adminEmail string) gin.HandlerFunc {
	return func(c *gin.Context) {
		token := c.Request.Header.Get("Authorization")
		token = strings.Split(token, " ")[1]
		if token == "" {
			c.AbortWithStatusJSON(401, gin.H{"errorType": "Unauthenticated", "message": "No token present"})
			return
		}

		user, err := authClient.GetUser(authUrl, token)
		if err != nil {
			c.AbortWithStatusJSON(401, gin.H{"errorType": "Unauthenticated", "message": "Invalid token"})
		}

		if user.Email != adminEmail {
			c.AbortWithStatusJSON(403, gin.H{"errorType": "Unauthorized", "message": "User does not have permission to perform this action"})
		}

		c.Next()
	}
}

func main() {
	config := ConfigFromEnv()
	client, err := ent.Open("postgres", config.DbConnectionStr)
	if err != nil {
		log.Fatalf("failed opening connection to postgres: %v", err)
	}
	defer client.Close()

	if err := client.Schema.Create(context.Background()); err != nil {
		log.Fatalf("failed creating schema resources: %v", err)
	}

	r := gin.Default()

	r.GET("/healthcheck", Authenticate(config.AuthUrl, config.AdminEmail), func(c *gin.Context) {
		c.String(200, "OK")
	})

	r.POST("/post", func(c *gin.Context) {
		var createPostRequest models.CreateNewPostRequest

		ctx := c.Copy()

		if err := c.ShouldBindJSON(&createPostRequest); err != nil {
			c.Error(err)
			return
		}

		resultUrl, err := HandlePost(client, &config, &createPostRequest.Post, ctx)
		if err != nil {
			c.Error(err)
		} else {
			c.JSON(200, models.CreateNewPostResponse{Url: resultUrl})
		}
	})

	r.POST("/search", func(c *gin.Context) {
		var queryLinksRequest models.QueryLinksRequest

		ctx := c.Copy()

		if err := c.ShouldBindJSON(&queryLinksRequest); err != nil {
			c.Error(err)
			return
		}

		result, err := QueryLinks(&queryLinksRequest, client, ctx)
		if err != nil {
			c.Error(err)
		} else {
			c.JSON(200, models.QueryLinksResponse{Links: result})
		}
	})

	r.Run(config.ListenAddr)
}
