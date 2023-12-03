package client

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strings"
  librarianModels "github.com/brahms116/pastureen_mono/golang/librarian_models"
	authClient "github.com/brahms116/pastureen_mono/golang/auth_client"
	authModels "github.com/brahms116/pastureen_mono/golang/auth_models"
	publisherModels "github.com/brahms116/pastureen_mono/golang/publisher_models"
)

const DEFAULT_PASTUREEN_ENDPOINT = "https://pastureen.davidkwong.net"

type Credentials struct {
	Email             string
	Password          string
	PastureenEndpoint string
}

type TokenCredentials struct {
	AccessToken       string
	RefreshToken      string
	PastureenEndpoint string
}

type PastureenCredentialOptions = Credentials

func MergeCredentiaOptions(a PastureenCredentialOptions, b PastureenCredentialOptions) PastureenCredentialOptions {
	if a.Email == "" {
		a.Email = b.Email
	}
	if a.Password == "" {
		a.Password = b.Password
	}
	if a.PastureenEndpoint == "" {
		a.PastureenEndpoint = b.PastureenEndpoint
	}
	return a
}

func MergeCredentialOptionsList(options []PastureenCredentialOptions) PastureenCredentialOptions {
	merged := PastureenCredentialOptions{}
	for _, option := range options {
		merged = MergeCredentiaOptions(merged, option)
	}
	return merged
}

func CredentialsFromOptions(options PastureenCredentialOptions) (Credentials, error) {
	if options.Email == "" {
		return PastureenCredentialOptions{}, fmt.Errorf("Email is required")
	}
	if options.Password == "" {
		return PastureenCredentialOptions{}, fmt.Errorf("Password is required")
	}

	credentials := PastureenCredentialOptions{
		Email:             options.Email,
		Password:          options.Password,
		PastureenEndpoint: options.PastureenEndpoint,
	}

	if credentials.PastureenEndpoint == "" {
		credentials.PastureenEndpoint = DEFAULT_PASTUREEN_ENDPOINT
	}

	return credentials, nil
}

func resolveCredentialsFromEnv() PastureenCredentialOptions {
	return PastureenCredentialOptions{
		PastureenEndpoint: os.Getenv("PASTUREEN_ENDPOINT"),
		Email:             os.Getenv("PASTUREEN_EMAIL"),
		Password:          os.Getenv("PASTUREEN_PASSWORD"),
	}
}

func resolveCredentialsFromConfigFile() PastureenCredentialOptions {
	// locate the config file
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return PastureenCredentialOptions{}
	}
	configFilePath := filepath.Join(homeDir, ".pastureen")
	configFile, err := os.Open(configFilePath)
	if err != nil {
		return PastureenCredentialOptions{}
	}
	defer configFile.Close()

	// parse the config file
	scanner := bufio.NewScanner(configFile)

	returnConfig := PastureenCredentialOptions{}

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			continue
		}
		if line[0] == '#' {
			continue
		}

		// split the line
		lineParts := strings.Split(line, "=")
		if len(lineParts) != 2 {
			continue
		}
		key := lineParts[0]
		value := lineParts[1]

		switch key {
		case "EMAIL":
			returnConfig.Email = value
		case "PASSWORD":
			returnConfig.Password = value
		case "ENDPOINT":
			returnConfig.PastureenEndpoint = value
		}
	}
	return returnConfig
}

func ResolveCredentials() (Credentials, error) {
	return CredentialsFromOptions(MergeCredentialOptionsList([]PastureenCredentialOptions{
		resolveCredentialsFromEnv(),
		resolveCredentialsFromConfigFile(),
	}))
}

func Login(credentials Credentials) (TokenCredentials, error) {
	tokens, err := authClient.Login(authModels.Credentials{
		Email:    credentials.Email,
		Password: credentials.Password,
		Endpoint: credentials.PastureenEndpoint + "/auth",
	})

	if err != nil {
		return TokenCredentials{}, err
	}

	return TokenCredentials{
		AccessToken:       tokens.AccessToken,
		RefreshToken:      tokens.RefreshToken,
		PastureenEndpoint: credentials.PastureenEndpoint,
	}, nil
}

func ToAuthTokenCredentials(tokenCredentials TokenCredentials) authModels.TokenCredentials {
	return authModels.TokenCredentials{
		AccessToken:  tokenCredentials.AccessToken,
		RefreshToken: tokenCredentials.RefreshToken,
		Endpoint:     tokenCredentials.PastureenEndpoint + "/auth",
	}
}

func ToPublisherCredentials(tokenCredentials TokenCredentials) publisherModels.Credentials {
	return publisherModels.Credentials{
		AccessToken: tokenCredentials.AccessToken,
		Endpoint:    tokenCredentials.PastureenEndpoint + "/publisher",
	}
}

func ToLibrarianCredentials(tokenCredentials TokenCredentials) librarianModels.Credentials {
  return librarianModels.Credentials{
    AccessToken: tokenCredentials.AccessToken,
    Endpoint:    tokenCredentials.PastureenEndpoint + "/librarian",
  }
}
