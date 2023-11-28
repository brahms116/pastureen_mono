package client

import (
	"bufio"
	"fmt"
	authModels "github.com/brahms116/pastureen_mono/golang/auth_models"
	"os"
	"path/filepath"
	"strings"
)

const PASTUREEN_ENDPOINT = "https://pastureen.davidkwong.net"

type CredentialOptions = authModels.Credentials

func MergeCredentialOptions(options CredentialOptions, options2 CredentialOptions) CredentialOptions {
	if options.Endpoint == "" {
		options.Endpoint = options2.Endpoint
	}
	if options.Email == "" {
		options.Email = options2.Email
	}
	if options.Password == "" {
		options.Password = options2.Password
	}
	return options
}

func MergeCredentialOptionsList(optionsList []CredentialOptions) CredentialOptions {
	if len(optionsList) == 0 {
		return CredentialOptions{}
	}
	options := optionsList[0]
	return MergeCredentialOptions(options, MergeCredentialOptionsList(optionsList[1:]))
}

func CredentialsFromOptions(options CredentialOptions) (authModels.Credentials, error) {

	if options.Email == "" {
		return authModels.Credentials{}, fmt.Errorf("Email is required")
	}

	if options.Password == "" {
		return authModels.Credentials{}, fmt.Errorf("Password is required")
	}

	credentials := authModels.Credentials{
		Endpoint: options.Endpoint,
		Email:    options.Email,
		Password: options.Password,
	}

	if credentials.Endpoint == "" {
		credentials.Endpoint = PASTUREEN_ENDPOINT
	}

	return credentials, nil
}

func resolveCredentialsFromEnv() CredentialOptions {
	return CredentialOptions{
		Endpoint: os.Getenv("PASTUREEN_ENDPOINT"),
		Email:    os.Getenv("PASTUREEN_EMAIL"),
		Password: os.Getenv("PASTUREEN_PASSWORD"),
	}
}

func resolveCredentialsFromConfigFile() CredentialOptions {
	// locate the config file
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return CredentialOptions{}
	}
	configFilePath := filepath.Join(homeDir, ".pastureen")
	configFile, err := os.Open(configFilePath)
	if err != nil {
		return CredentialOptions{}
	}
	defer configFile.Close()

	// parse the config file
	scanner := bufio.NewScanner(configFile)

	returnConfig := CredentialOptions{}

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
			returnConfig.Endpoint = value
		}
	}
	return returnConfig
}

func ResolveCredentials() (authModels.Credentials, error) {
	return CredentialsFromOptions(
		MergeCredentialOptionsList(
			[]CredentialOptions{
				resolveCredentialsFromEnv(),
				resolveCredentialsFromConfigFile(),
			},
		),
	)
}
