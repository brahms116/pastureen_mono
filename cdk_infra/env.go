package main

const ENV_TEST = "test"
const ENV_DEV = "dev"
const ENV_PROD = "prod"

func parseEnv(env string) string {
  if env != ENV_PROD && env != ENV_TEST && env != ENV_DEV {
    return ENV_DEV
  }
  return env
}
