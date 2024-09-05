---
CREATED_AT: 2024-09-05T23:19:39.372206664+00:00
TOPIC: blog
category: news
title: Leaf Templates are Here
---

## Leaf Templates Are Here

With the release of `0.3.0` comes an experimental new feature of leaf
templates.  This new template will power virtual directories in your
markdown book that contain only entries.

### New Topic Setting

- `leaf_template`

  This is very similar to the `template` setting for templates; however,
  instead of working with a single entry it uses a collection of `entries`.
  These can be iterated through.  Below is an example leaf template:

  ```handlebars
  # {{path}}

  {{#each entries}}
    - [{{this.meta.title}}](/{{this.virtual_path}})
  {{/each}}
  ```

  This demonstrates the data available for this template.  It is provided
  a `path` which is the virtual path for the README index being generated.
  It also receives `entries` which can be iterated over.  Any of the
  variables collected can be accessed under the `meta` keyword.  Here is
  a full list of the current available data points as of `0.3.0`:

  - `topic` : The string name of the topic this entry belongs to
  - `created_at` : string serialized to a string in rfc3339 format
  - `file_loc` : full file path of the entry on disk
  - `virtual_path` : relative path from the root of the markdown book
  - `meta` : hash of all variables for the entry
  - `content` : full string content of the entry
