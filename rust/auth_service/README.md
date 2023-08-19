
# Auth Api Service

An authentication service for Pastureen, it exposes a rust struct `AuthApi` to interact with. It is currently designed to serve a hobby capacity and missing many features.

It is currently backed by a postgres database.

### Api

- Login using email and password, retrieves a token pair consisting of an access token and a refresh token

- Retreives user information by providing an access token

- Retrieves a new token pair by providing a refresh token. Please note that the refresh token is rotated, a used refresh token is considered invalid. In an attempt to use
an old refresh token, all of its generated refresh tokens descendants will also be invalidated.

### Setup

[The example env file](./example.env) contains the environment variables needed for this service. Alternatively, `AuthApi::fromConfig` can be used to create an api specifying any envs.

### Tests

Tests can be setup by...

1. Create a `.test.env` following the [example env file](./example.env)
2. Setup the database by running 
   ```bash
   ./setup_test.sh
   ```
3. Run the tests with
   ```bash
   ./test.sh
   ```

Steps 1 and 2 only have to be executed initially, subsequent executions can start from step 3.
