module pastureen/author

go 1.21.1

replace pastureen/publisher-models => ../publisher_models

replace pastureen/publisher-client => ../publisher_client

replace pastureen/http-utils => ../http_utils

replace pastureen/auth-client => ../auth_client

replace pastureen/auth-models => ../auth_models

replace pastureen/blog-models => ../blog_models

replace pastureen/librarian-models => ../librarian_models

replace pastureen/librarian-client => ../librarian_client

require (
	pastureen/auth-client v0.0.0-00010101000000-000000000000 // indirect
	pastureen/auth-models v0.0.0-00010101000000-000000000000 // indirect
	pastureen/blog-models v0.0.0-00010101000000-000000000000 // indirect
	pastureen/http-utils v0.0.0-00010101000000-000000000000 // indirect
	pastureen/librarian-client v0.0.0-00010101000000-000000000000 // indirect
	pastureen/librarian-models v0.0.0-00010101000000-000000000000 // indirect
	pastureen/publisher-client v0.0.0-00010101000000-000000000000 // indirect
	pastureen/publisher-models v0.0.0-00010101000000-000000000000 // indirect
)
