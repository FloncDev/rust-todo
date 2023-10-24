# Rust Todo
A simple todo list made as a CLI in rust

<p align="center">
  <img src="https://media.discordapp.net/attachments/858096635966914631/1166512846545375282/lc6BQc1.png?ex=654ac2bf&is=65384dbf&hm=75a2fedc3496c7571f158c79299840f41aba69eb942c915e6653c4ffb2291ec3&=" alt="Screenshot of CLI in use.">
</p>

```bash
$ rust-todo help
A simple todo app written in rust.

Usage: rust-todo <COMMAND>

Commands:
  list  List all todos. -a will also show completed todos.
  new   Create a new todo
  done  Toggle done on a todo
  rm    Remove a todo
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

Personally, I just copied the binary from `./target/release` into `~/.local/bin` as `todo` for easier access
