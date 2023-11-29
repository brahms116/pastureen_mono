package client

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

const DEFAULT_PASTUREEN_ENDPOINT = "https://pastureen.davidkwong.net"

type PastureenCredentials struct {
	Email             string
	Password          string
	PastureenEndpoint string
}

type PastureenCredentialOptions = PastureenCredentials

func (p PastureenCredentialOptions) Merge(other PastureenCredentialOptions) PastureenCredentialOptions {
	if p.Email == "" {
		p.Email = other.Email
	}
	if p.Password == "" {
		p.Password = other.Password
	}
	if p.PastureenEndpoint == "" {
		p.PastureenEndpoint = other.PastureenEndpoint
	}
	return p
}

func (p PastureenCredentialOptions) ToPastureenCredentials() (PastureenCredentialOptions, error) {
	if p.Email == "" {
		return PastureenCredentialOptions{}, fmt.Errorf("Email is required")
	}
	if p.Password == "" {
		return PastureenCredentialOptions{}, fmt.Errorf("Password is required")
	}

	credentials := PastureenCredentialOptions{
		Email:             p.Email,
		Password:          p.Password,
		PastureenEndpoint: p.PastureenEndpoint,
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

func ResolveCredentials() (PastureenCredentials, error) {
	return resolveCredentialsFromEnv().Merge(resolveCredentialsFromConfigFile()).ToPastureenCredentials()
}
