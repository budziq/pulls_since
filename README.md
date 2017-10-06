# pulls_since [![Build Status](https://travis-ci.org/budziq/pulls_since.svg?branch=master)](https://travis-ci.org/budziq/pulls_since) [![crates.io](https://img.shields.io/crates/v/pulls_since.svg)](https://crates.io/crates/pulls_since)

Micro tool to print Markdown formatted list of pull requests
closed on a given github repository since given date

## Example usage

```bash
pulls_since --repo 'rust-lang-nursery/rust-cookbook' --since 31.10.2017
```

Few date formats are available. Including "dd.mm.yyyy", "dd.mm" and "yyyy/mm/dd"

## Example output

```markdown
- @sb89 [Added "Check webpage for broken links" example](https://github.com/rust-lang-nursery/rust-cookbook/pull/299)
- @jaemk [Add build tools section & basic `cc` example](https://github.com/rust-lang-nursery/rust-cookbook/pull/298)
- @ludwigpacifici [Add "Run piped external commands" example](https://github.com/rust-lang-nursery/rust-cookbook/pull/297)
- @lucasem [centralize links for badges, categories, and crates](https://github.com/rust-lang-nursery/rust-cookbook/pull/279)
- @matklad [Add HTML word to make Ctrl+F easier](https://github.com/rust-lang-nursery/rust-cookbook/pull/278)
- @nocduro [Add rayon thumbnail generation example](https://github.com/rust-lang-nursery/rust-cookbook/pull/275)
- @ericho [Use a threadpool to calculate SHA1 in all *.iso files in a folder.](https://github.com/rust-lang-nursery/rust-cookbook/pull/274)
```

### Rendered

- @sb89 [Added "Check webpage for broken links" example](https://github.com/rust-lang-nursery/rust-cookbook/pull/299)
- @jaemk [Add build tools section & basic `cc` example](https://github.com/rust-lang-nursery/rust-cookbook/pull/298)
- @ludwigpacifici [Add "Run piped external commands" example](https://github.com/rust-lang-nursery/rust-cookbook/pull/297)
- @lucasem [centralize links for badges, categories, and crates](https://github.com/rust-lang-nursery/rust-cookbook/pull/279)
- @matklad [Add HTML word to make Ctrl+F easier](https://github.com/rust-lang-nursery/rust-cookbook/pull/278)
- @nocduro [Add rayon thumbnail generation example](https://github.com/rust-lang-nursery/rust-cookbook/pull/275)
- @ericho [Use a threadpool to calculate SHA1 in all *.iso files in a folder.](https://github.com/rust-lang-nursery/rust-cookbook/pull/274)

## License

[MIT](LICENSE)
