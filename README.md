# pulls_since [![Build Status](https://travis-ci.org/budziq/pulls_since.svg?branch=master)](https://travis-ci.org/budziq/pulls_since) [![crates.io](https://img.shields.io/crates/v/pulls_since.svg)](https://crates.io/crates/pulls_since)

Micro tool to print Markdown formatted list of pull requests
closed on a given github repository since given date

```
pulls_since 0.4.0
Print Markdown formatted list of pull requests closed since given date

USAGE:
    pulls_since [OPTIONS] --owners <owner>... --repos <repo>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --exclude-login <login>    ommit PR's by given login (bots etc.)
    -o, --owners <owner>...        space separated list of owners or org names
    -r, --repos <repo>...          space separated list of 'owner/repo'
    -s, --since <since>            start date argument dd.mm.yyyy
    -u, --until <until>            end date argument dd.mm.yyyy
```

## Example usage

- Show all pull requests to [rust-lang-nursery/rust-cookbook](https://github.com/rust-lang-nursery/rust-cookbook) and [budziq/pulls_since](https://github.com/budziq/pulls_since)
between 30.09.2017 and 07.10.2017 omitting ones made by user [budziq](https://github.com/budziq).
```bash
pulls_since --repos rust-lang-nursery/rust-cookbook budziq/pulls_since -s 30.09.2017 -u 07.10.2017 -e budziq
```

- Show all pull requests within github organizaion [rust-lang-nursery](https://github.com/rust-lang-nursery) or against repositories
owned by user [budziq](https://github.com/budziq) made since 30.09.2017.
```bash
pulls_since --owners rust-lang-nursery budziq -s 30.09.2017
```

Few date formats are available. Including "dd.mm.yyyy", "dd.mm" and "yyyy/mm/dd"

## Example output

```markdown
#### rust-lang-nursery/rust-cookbook

- @mykalu [Match semver crate examples' styling](https://github.com/rust-lang-nursery/rust-cookbook/pull/315)
- @oldmanmike [Add "Run an external command passing it stdin and check for an error code" example](https://github.com/rust-lang-nursery/rust-cookbook/pull/310)
- @FaultyRAM [Add "Parse a complex version string" example](https://github.com/rust-lang-nursery/rust-cookbook/pull/308)
- @V1shvesh [Add num_cpus example](https://github.com/rust-lang-nursery/rust-cookbook/pull/307)
- @sb89 [Added "Check webpage for broken links" example](https://github.com/rust-lang-nursery/rust-cookbook/pull/299)
- @ludwigpacifici [Add "Run piped external commands" example](https://github.com/rust-lang-nursery/rust-cookbook/pull/297)
- @ericho [Use a threadpool to calculate SHA1 in all *.iso files in a folder.](https://github.com/rust-lang-nursery/rust-cookbook/pull/274)

#### budziq/pulls_since

- @nabijaczleweli [Added --until/-u option](https://github.com/budziq/pulls_since/pull/7)
- @KodrAus [Add clap for arg parsing](https://github.com/budziq/pulls_since/pull/2)
```

### Rendered output

#### rust-lang-nursery/rust-cookbook

- @mykalu [Match semver crate examples' styling](https://github.com/rust-lang-nursery/rust-cookbook/pull/315)
- @oldmanmike [Add "Run an external command passing it stdin and check for an error code" example](https://github.com/rust-lang-nursery/rust-cookbook/pull/310)
- @FaultyRAM [Add "Parse a complex version string" example](https://github.com/rust-lang-nursery/rust-cookbook/pull/308)
- @V1shvesh [Add num_cpus example](https://github.com/rust-lang-nursery/rust-cookbook/pull/307)
- @sb89 [Added "Check webpage for broken links" example](https://github.com/rust-lang-nursery/rust-cookbook/pull/299)
- @ludwigpacifici [Add "Run piped external commands" example](https://github.com/rust-lang-nursery/rust-cookbook/pull/297)
- @ericho [Use a threadpool to calculate SHA1 in all *.iso files in a folder.](https://github.com/rust-lang-nursery/rust-cookbook/pull/274)

#### budziq/pulls_since

- @nabijaczleweli [Added --until/-u option](https://github.com/budziq/pulls_since/pull/7)
- @KodrAus [Add clap for arg parsing](https://github.com/budziq/pulls_since/pull/2)

### Authorization

By default `pulls_since` uses unauthorized flow which will get your requests
throthled quickly. To make large number of requests or operate on really big
repositories please use the github
[token authorization](https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line/).

Either export your token as an environmental variable or put it in an `.env`
file somewhere above your current woking directory.

```bash
GITHUB_TOKEN=39984770ba9ba1c663b6b50beab9b004
```

## License

[MIT](LICENSE)
