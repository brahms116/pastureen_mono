fn main() {
    println!("Hello, world!");
}

// fn parse_search_query -> gives back the queries needed from librarian
//
// first iteration should be simple, if its a tag, then just return the tag

// handler POST global-search
//  - get list of tags
//  - parse query
//  - send the query to librarian
//  - use the results to generate the html (fn generate_global_search_results)
//  - return

// handler GET healthcheck
