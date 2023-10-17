package main


import (
  "os"
  "context"
  "log"
  _ "github.com/lib/pq"
  "pastureen/librarian/ent"
)

type LibrarianConfig struct {
  DbConnectionStr string
}

func ConfigFromEnv() LibrarianConfig {
  return LibrarianConfig{
    DbConnectionStr: os.Getenv("LIBRARIAN_DB_CONN_STR"),
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
}
