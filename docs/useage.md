# Usage

> [!warning]
> This system is in a very early alpha phase.

## Add To `book.toml`

```toml
# Sets up mdbook-journal with your book
[preprocessor.journal]
command = "mdbook-journal"

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
[preprocessor.journal.topics.code-blog.variables]
title = { required = true }
```

## Create Entry

From the root directory of the `book.toml` file:

( **NOTE** for this demo my directory is `/tmp/rofl` )

```bash
markdown-journal new code-blog
```

This presents a prompt to input the required data.
you should see the following prompt:

```bash
(title)❯
```

This next part is tricky. You'll need to travel
back in time to `July 20th, 2024` some time in
the evening.

```text
(title)❯ Documention of the Future
Entry Created: /tmp/rofl/src/code-blog/2024/July/20-22-08-17-documention-of-the-future.md
```

## Example File `20-22-08-17-documention-of-the-future.md`

```markdown
---
CREATED_AT: 2024-07-20T22:08:17.623417543+00:00
TOPIC: code-blog
title: Documention of the Future
---
```

- Each file is in "kebab" case of the provided title and
  prefixed with the UTC time it was generated with. The
  next stage of development with this will drive This
  structure; however, for now it's hard coded with the
  following logic:

  `src/journal/topic/dir_mapper.rs`

  ```rust
  impl<'a> DirMapper<'a> {
      pub(super) fn new(topic: &'a Topic) -> Self {
          Self { topic }
      }

      pub fn map(&self, entry: &Entry) -> Result<PathBuf> {
          let year_month = entry.created_at().format("%Y/%B").to_string();
          Ok(year_month.into())
      }
  }
  ```

  `src/journal/topic/filename_mapper.rs`

  ```rust
  impl<'a> FilenameMapper<'a> {
      pub(super) fn new(topic: &'a Topic) -> Self {
          Self { topic }
      }

      pub fn map(&self, entry: &Entry) -> Result<PathBuf> {
          let mut filename = entry.created_at().format("%d-%H-%M-%S ").to_string();
          filename.push_str(
              entry
                  .meta_value(&"title")
                  .unwrap_or_else(|| &DEFAULT_TITLE)
                  .as_str()
                  .unwrap_or("Untitled"),
          );
          let mut filename: PathBuf = filename.from_case(Case::Title).to_case(Case::Kebab).into();
          filename.set_extension("md");
          Ok(filename)
      }
  }
  ```

- Each file is created with it's front-matter that
  was collected. The "body" of the document is left
  blank. My goal is to support a template generation,
  but until then here is the code that sets an empty
  body:

  `src/journal/topic/cli_entry.rs`

  ```rust
  impl EntryGenerationTrait for CliEntryGeneration {
      fn collect_value(&self, variable: &Variable) -> Result<Option<MetaValue>> {
          use std::io::Write;
          let stdout = &mut std::io::stdout();
          let stdin = &mut std::io::stdin();
          let data = &mut String::with_capacity(64);
          write!(stdout, "({})❯ ", variable.key())?;
          stdout.flush()?;
          stdin.read_line(data)?;

          match data.trim() {
              "" => Ok(None),
              value => Ok(Some(MetaValue::String(value.to_owned()))),
          }
      }
  }
  ```

  The keen observer may be wondering how this sets
  the content of an entry. It doesn't!! !RAGE-BAIT!
  What's really happening is the `EntryGenerationTrait`
  as an optional rendering method you can override:

  `src/journal/topic/traits/entry_generation.rs`

  ```rust
  pub trait EntryGenerationTrait {
      fn created_at(&self) -> Result<UtcDateTime> {
          Ok(Utc::now())
      }

      fn collect_value(&self, variable: &Variable) -> Result<Option<MetaValue>>;

      fn generate_content(&self, _topic: &Topic, builder: EntryBuilder) -> Result<EntryBuilder> {
          Ok(builder.content(""))
      }
  }
  ```

## List Entries for Topic

```bash
markdown-journal ls code-blog
```

```text
/tmp/rofl/src/code-blog/2024/July/22-01-14-50-whammy!.md
/tmp/rofl/src/code-blog/2024/July/22-01-19-14-anchors-aweigh.md
/tmp/rofl/src/code-blog/2024/July/22-01-02-05-i-love-rust.md
```

## Generation

This currently is a very primitive preprocessor attached to
the rendering pipeline. With this bear minimum setup it creates
a different top level container for each topic. Each entry is
included with the newest sorted to the top.
