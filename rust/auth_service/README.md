
# Auth Api Service

An authentication service for Pastureen, it exposes a rust struct `AuthApi` to interact with. It is currently designed to serve a hobby capacity and missing many features.

It is currently backed by a postgres database.

### Api

- Login using email and password, retrieves a token pair consisting of an access token and a refresh token

- Retreives user information by providing an access token

- Retrieves a new token pair by providing a refresh token. Please note that the refresh token is rotated, a used refresh token is considered invalid. In an attempt to use
an old refresh token, all of its generated refresh tokens descendants will also be invalidated.







