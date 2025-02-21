# WatcheRs

Compile all of your starred repositories into one MarkDown list.

# Installation

# Help

```
Create an Awesome list with starred repos

Usage: watchers [OPTIONS] --token <TOKEN>

Options:
  -t, --token <TOKEN>
  -f, --file <FILE>      [default: README.md]
  -p, --preview
  -F, --format <FORMAT>  [default: "+ **[{owner}/{name}]({url})** `⭐ {star}`"]
  -c, --content-table
  -h, --help             Print help
  -V, --version          Print version
```

Check [Github Markdown spec](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax) to see how format works. Currently supported values are:

| Name | Value |
|-|-|
| owner | owner of repo |
| name | name of repo |
| star | stars count |
| issue | issues count |
