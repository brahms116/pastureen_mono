module pastureen/librarian-client

go 1.21.1

replace pastureen/auth-models => ../auth_models

replace pastureen/http-utils => ../http_utils

replace pastureen/blog-models => ../blog_models

replace pastureen/librarian-models => ../librarian_models

replace pastureen/auth-client => ../auth_client

require (
	github.com/google/uuid v1.3.1 // indirect
	pastureen/auth-client v0.0.0-00010101000000-000000000000 // indirect
	pastureen/auth-models v0.0.0-00010101000000-000000000000 // indirect
	pastureen/blog-models v0.0.0-00010101000000-000000000000 // indirect
	pastureen/http-utils v0.0.0-00010101000000-000000000000 // indirect
	pastureen/librarian-models v0.0.0-00010101000000-000000000000 // indirect
)
