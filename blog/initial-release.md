---
CREATED_AT: 2024-08-05T22:30:05.714857359+00:00
TOPIC: blog
category: news
title: Initial Release
---

## Initial Release v0.2.0

I'm proud to announce the first release of `mdbook-journal` @ `v0.2.0`.
There is still more to come; however, I believe this project has
enough to get started with.  In this post I'll outline what is
currently available and what is yet to come!

### What's in `v0.2.0`?

Presently your journal can define any number of "topics".  A topic
in this case represents a collection of similar items that share a
common data set.  To create a new "entry" for a topic use the
following command:

```bash
mdbook-journal new <TOPIC_NAME>
```

This will use the settings for `TOPIC_NAME` to generate a new entry.

#### Topic Settings

A topic presently defines three major concepts:

- `path_mapping`

  A handlebars template which describes what file structure to use
  when creating a new instance of a topic.  This template can refer
  to data defined by the topic **AND** also has access to the time
  in which it was created.

  To reference any user defined topic data simply wrap it in
  handlebars.  It should also be noted that there is a number of
  string modifiers built in you can can take advantage of, such
  as `titleCase` and `kebabCase`.

  The entire string is also interpolate with date-time formatting
  tokens.  For instance if your template had the `%Y` token it would
  be replaced with the `YYYY` four digit calendar year.

- `template`

  This is very similar to the handlebars template based path
  mapping, but it is used to populate a newly created topic entry.
  Use it as a starter for a new entry with the high level structure
  of subjects you want to address, etc.

- `variables`

  A hash definition describing all of the data you want every entry
  to collect for a topic.  Each key translates to a data-point key
  that is collected and the value is hash of settings.  Here is the
  following settings each key can have:

  - `required` (boolean) (default false)

    If true a "non-empty" value is expected.

  - `default` (string) (default "")

    If a value is not provided this is the default used

#### Rendering Output

The rendering is very minimal.  All topics are added to the summary
at render time and are displayed in a directory structure format
that follows the entries path on disk.

## What's Coming Next?

- Templated Topic Layout

  A feature that allows you to define how entries are displayed
  for a topic.  There are many different formats for representing
  entry data.  I want topics to be able to dynamically update where
  and how they display data as the information on it is updated.

- Programatic Entry Interface

  Ultimately I envision this cli tool being driven by a neovim
  plugin.  Aside from that, I believe it would be extremely handy
  to support entry like this:

  ```bash
  mdbook-journal new TOPIC --json '{"title":"Test Title"}'
  ```

- SQL Structured Searching

  The idea is still loose; but I want to support something like
  the following:

  ```bash
  mdbook-journal query TOPIC /
    "SELECT filename FROM entry WHERE category = 'rust'"
  ```
