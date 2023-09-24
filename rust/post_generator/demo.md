---
tags: tech, rust
---

# Heading number 1

Some random content. Here is a [link](www.google.com)

This is some **bold** text

This is _italic_

This is a code block

```yaml
on: workflow_dispatch
jobs:
    build:
        runs-on: ubuntu-latest
        steps:
            - uses: actions/checkout@v2
              with:
                  ref: "main"
```
