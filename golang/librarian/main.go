package main

import (
	"context"
	"log"
	"os"
	authClient "github.com/brahms116/pastureen_mono/golang/auth_client"
	models "github.com/brahms116/pastureen_mono/golang/librarian_models"
  "github.com/brahms116/pastureen_mono/golang/librarian/ent"
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
		BlogBucketName:  os.Getenv("BLOG_BUCKET_NAME"),
		ListenAddr:      os.Getenv("SERVER_LISTEN_ADDR"),
		AuthUrl:         os.Getenv("AUTH_SERVICE_URL"),
		AdminEmail:      os.Getenv("ADMIN_EMAIL"),
	}
}

func Authenticate(authUrl string, adminEmail string) gin.HandlerFunc {
	return func(c *gin.Context) {
		token := c.Request.Header.Get("Authorization")
		token = strings.TrimPrefix(token, "Bearer ")
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

	r.GET("/healthcheck", func(c *gin.Context) {
		c.String(200, "OK")
	})

	r.GET("/link", func(c *gin.Context) {
		var getLinkRequest models.GetLinkRequest
		ctx := c.Copy()
		if err := c.ShouldBindQuery(&getLinkRequest); err != nil {
			c.JSON(400, gin.H{"errorType": "MissingQuery", "message": err.Error()})
			return
		}

		result, err := GetLink(getLinkRequest.Url, client, ctx)
		if err != nil {
			c.JSON(500, gin.H{"errorType": "InternalError", "message": err.Error()})
		} else {
			c.JSON(200, models.GetLinkResponse{Link: result})
		}
	})

	// Handler to retrieve all the tags names using the QueryTagNames func
	r.GET("/tags", func(c *gin.Context) {
		ctx := c.Copy()
		result, err := QueryTagNames(client, ctx)
		if err != nil {
			c.JSON(500, gin.H{"errorType": "InternalError", "message": err.Error()})
		} else {
			c.JSON(200, models.GetTagsResponse{Tags: result})
		}
	})

	r.POST("/post", Authenticate(config.AuthUrl, config.AdminEmail), func(c *gin.Context) {
		var createPostRequest models.CreateNewPostRequest

		ctx := c.Copy()

		if err := c.ShouldBindJSON(&createPostRequest); err != nil {
			c.Error(err)
			return
		}

		resultUrl, err := HandlePost(client, &config, &createPostRequest.Post, ctx)
		if err != nil {
			c.JSON(500, gin.H{"errorType": "InternalError", "message": err.Error()})
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
			c.JSON(500, gin.H{"errorType": "InternalError", "message": err.Error()})
			return
		} else {
			c.JSON(200, models.QueryLinksResponse{Links: result})
		}
	})

	r.Run(config.ListenAddr)
}
