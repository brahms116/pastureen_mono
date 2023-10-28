module pastureen/publisher-client

replace pastureen/publisher-models => ../publisher_models

replace pastureen/http-utils => ../http_utils

replace pastureen/auth-client => ../auth_client

replace pastureen/auth-models => ../auth_models

replace pastureen/blog-models => ../blog_models

go 1.21

require (
	pastureen/auth-client v0.0.0-00010101000000-000000000000
	pastureen/auth-models v0.0.0-00010101000000-000000000000
	pastureen/http-utils v0.0.0-00010101000000-000000000000
	pastureen/publisher-models v0.0.0-00010101000000-000000000000
)

require pastureen/blog-models v0.0.0-00010101000000-000000000000
