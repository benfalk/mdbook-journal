<!-- markdownlint-disable MD041 -->
<!-- markdownlint-disable MD013 -->

<!-- markdownlint-disable MD033 -->
<img alt="journing robot" src="./assets/robot-journaling.jpeg" width="100%" />
<!-- markdownlint-enable MD033 -->

---

# 🗒️ mdBook Journal _(beta)_

Workflow tool that allows you to generate templated documentation,
notes, measurements... or really anything. At it's core is the concept
of a journal "topic". You specify what core data is tracked for
each topic and then get to work!

**NOTE:** Plugin project for the [`mdBook`] documentation system. It's
not very complex; however, you will need to understand how it works in
order to take advantage of some of the functionality of this tool.

[`mdBook`]: https://rust-lang.github.io/mdBook/

## 🔩 Demo Example

Let's say that you want to document every major [`architectural decision`]
your team makes.

From your `book.toml` you can define such a topic:

```toml
# book.toml

# Sets up mdbook-journal with your book
[preprocessor.journal]
command = "mdbook-journal"

[preprocessor.journal.topics.ADR]
# The `path_mapping` drives the strategy for generating
# the path location of each entry.  It supports a mixture
# of handlebars for any topic data and also is formatted
# against the `CREATED_AT` time.
#
# In this example the directory structure will be the
# year of the title as the root directory, followed by
# a directory for the category, and then lastly the title
# as the filename.  It should be pointed out that a `.md`
# extension is automatically appended to the path.
#
# Date Interpolation Docs:
#   <https://docs.rs/chrono/latest/chrono/format/strftime/index.html>
#
# default = "%Y/%B/%d-%H-%M-%S-{{kebabCase title}}"
#
path_mapping = "%Y/{{kebabCase category}}/{{kebabCase title}}"

# This is how you specify a topic's meta data.
#
# Each key maps to the front-matter which is pinned
# to all journal entries.  Currently `title` is
# required to generate file names.  Feel free to add
# as many more fields that you want.
#
# Field Options
#
# - required = false
#
#   If a field is required this means it's creation
#   and updates must have a valid representation of
#   the data.  Presently that just means it cannot
#   be empty.
#
# - default = ""
#
#   Value to provide if the field has failed the
#   validation phase.  It should be noted that a
#   default is also provided even if the field is
#   not required.  ** This is likely to change **
#
#   This is the topic name ---+
#                             |
[preprocessor.journal.topics.ADR.variables]
title = { required = true }
category = { required = true }
priority = { required = true, default = "low" }
```

In this example the topic name is `ADR` and has three
defined data points of `title`, `category`, and `priority`.
This ensures that each record created will collect
those three following data points.

Once a topic is setup an "entry" can be created for
it. Here is an example command to create a decision
record:

```bash
markdown-journal new ADR
```

This will produce a prompt that collects these
data points:

```bash
(category)❯ psql
(priority)❯ high
(title)❯ Migrate from Postgres v14
```

Assuming the above was entered it will produce the
following file in your project:

`ADR/2024/psql/migrate-from-postgres-v14.md`

```markdown
---
CREATED_AT: 2024-07-23T02:16:12.682975620+00:00
TOPIC: ADR
category: psql
priority: high
title: Migrate from Postgres v14
---
```

This is part of a markdown spec called [front-matter]. It
allows for assigning specific data and is at the forefront
of how the journal operates. All entries added this way
are automatically included in the generated HTML documents
without needing to include them in the `SUMMARY.md` file.

[`architectural decision`]: https://adr.github.io/

## ⚙️ Install Locally

Currently to install you'll need to clone this repo and
install with cargo:

```bash
git clone https://github.com/benfalk/mdbook-journal.git
cargo install --path mdbook-journal
```

Verify that it's installed correctly:

```bash
mdbook-journal -V
```

`mdbook-journal 0.1.3-alpha`
