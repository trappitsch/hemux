# hemux

This small Rust CLI is my attempt to bring simpler test running to [Helix].
As [Helix] in its current version does not have a plugin system,
we can still get some decent testing setup by configuring keyboard shortcuts
and feeding variables to an external CLI.

This program is supposed to be very opinionated and should suit my workflow.
While an effort is made to make it somewhat generalizable,
I will not spend much time for now on generalizing it.

## What hemux currently does

We assume that you are using the [Helix] editor
from within a [tmux] session.
You are writing code and tests.
This program should allow you to set up simple shortcuts
in order to run different tests in a secondary, adjacent tmux pane.

What tests are you able to run?

- The nearest tests to your current cursor location.
- All the tests in the file that you are currently in.
- The whole test suite.
- Re-run the last command.

## Installation and setup

### Installation

1. Clone the project.
2. `cd` into the cloned folder.
3. `cargo install --path .`

OR

1. `cargo install --git https://github.com/trappitsch/hemux`

### Setup

Add the following keymapping to your `~/.config/helix/config.toml` file:

```toml
[keys.normal.'\']
i = ":sh hemux cmd -n=info -a -l=%{language} -p=%{buffer_name} -c=%{cursor_line}"  # language agnostic info command.
l = ":sh hemux last"  # Run last run command

[keys.normal.'\'.r]
r = ":sh hemux cmd -n='run' -l=%{language}"  # language specific run command
R = ":sh hemux cmd -n='run-alt' -l=%{language}"  # language specific run-alt command
b = ":sh hemux cmd -n='build' -l=%{language}"  # language specific build command
B = ":sh hemux cmd -n='build-alt' -l=%{language}"  # language specific buld-alt command

[keys.normal.'\'.t]
s = ":sh hemux test-suite -l=%{language} -p=%{buffer_name} -c=%{cursor_line}"  # Run test suite
f = ":sh hemux test-file -l=%{language} -p=%{buffer_name} -c=%{cursor_line}"  # Run test in file
n = ":sh hemux test-nearest -l=%{language} -p=%{buffer_name} -c=%{cursor_line}"  # Run nearest test
```

This will give the following shortcuts (where the comma means "followed by"):

- `\, i`: Run a language agnostic information comand
  to print some variables we can use in commands (see below).
- `\, l`: Run the last command again
- `\, r, r`: Run a language specific command name `run`.
- `\, r, R`: Run a language specific command name `run-alt`.
- `\, r, b`: Run a language specific command name `build`.
- `\, r, B`: Run a language specific command name `build`.
- `\, t, s`: Run the whole test suite
- `\, t, f`: Run all tests in the current file
- `\, t, n`: Run the nearest test

Here we use `\` as a new menu, like `Space`.
If you are coming from neovim, this would be similar to the local leader
(or hemux leader ;)).

## Configuration

When you first run hemux (e.g., with `hemux -h`),
it runs with its default configuration.
The default configuration is automatically written to a `hemux/config.toml` file
in the OS specific configuration folder.
On Linux this would be in `$HOME/.config/hemux/config.toml`.

The default configuration file at the moment looks like this:

```toml
[tmux_config]
split_direction = "Vertical"
split_percentage = 20

[test_runners.rust]
test_suite = "cargo nextest run"
test_file = "cargo nextest run %{file_name_no_suffix}"
test_nearest = "cargo nextest run %{function_name}"
regex_nearest = '(?:.*\s+|^)fn\s+([A-Za-z_][A-Za-z0-9_]*)'

[command_any_lang.info]
command = "echo file_name: %{file_name} ; echo file_path: %{file_path} ; echo cursor_line: %{cursor_line}"

[command.rust.run-alt]
command = "cargo run -r"

[command.rust.build-alt]
command = "cargo build -r"

[command.rust.build]
command = "cargo build"

[command.rust.run]
command = "cargo run"
```

### Tmux configuration

- `split_direction`: Valid values are "Vertical" or "Horizontal".
- `split_percentage`: hemux will clamp this value between 5 and 95.

### Test runners

You can specify as many test runners as you like.
The imporant thing is that the language that helix passes to hemux
via the `-l` flag
must be identical to the name of this test runner,
e.g., `[test_runner.rust]` as above for "rust" files.

The next three commands are:

- `test_suite`: The command to run for running the whole test suite.
- `test_file`: The command to run for running the tests in one specific file.
- `test_nearest`: The command to run for running a test by a search pattern
  upward from the current cursor.
  See below for more detail.

In these commands, you can use the following variables:

- `%{file_name}`: Filename only.
- `%{file_name_no_suffix}`: Filename stem, no extension.
- `%{file_path}`: Relative path of the file and filename.
- `%{file_path_no_suffix}`: Relative path of the file and filename without extension.
- `%{function_name}`: Name of the nearest function as found by matching
  the given regex pattern from cursor line up.
- `%{cursor_line}`: The line number the cursor is at when called.

The test runners default to [nextest],
which in my opinion is much more suited for this kind of testing
as its output displays very nicely in a limited height pane.
If you prefer `cargo test`, you could do the following replacement:

`cargo nextest run` -> `cargo test`

### Regex pattern

To find the function name, we start scanning the line the cursor is currently on.
If the regex pattern matches, we have found the line.
The regex pattern itself must select the function name that we want to pass
to the `%{function_name}` variable as the first and only matching group.

Let us for example analyze the above regex pattern:
`(?:.*\s+|^)fn\s+([A-Za-z_][A-Za-z0-9_]*) `

- The first group `(?:.*]s+|^)` is not returned (`?:`).
  We then allow any character(s) followed by at least one space
  or the following has to be the start of the line.
- Then comes the keyword `fn` that we are looking for
  followed by at least one space.
- Then comes the matching group that we do want to return:
  It must start with one letter (capitalized or not),
  followed by zero or more letters (any case) or numbers.

Does the pattern capture everything?
Probably not.
If you want to make it better, you can configure it better now for your own hemux.
Please also raise an issue, I'd be happy to improve the default pattern.

To test out regex pattern on multiple lines,
the [Regular Expressions 101] website has been very useful for me.

### Language agnostic commands

The section `[command_any_lang.info]`
defines a command named "info",
which does not depend on a given file's language from which it is run.
It is thus a language agnostic command.
Here, it would print out some helix variables.

### Language specific commands

This is a little bit like filetype specific commands.
We define four commands: "run", "run-alt", "build", and "build-alt".
These are defined for rust for now as:

```toml
[command.rust.run]
command = "cargo run"
```

We build the command by specificing `command`,
followed by the language (in the spelling that Helix provides),
followed by the name of the comman (here `run`).
Then the command is provided.
This can now be run from within helix as:

```shell
:sh hemux cmd -n='run' -l=%{language}
```

Here, we would simply run in shell (`:sh`)
the command named `run` with a given language specifier.

## Limitations

### Rust test runners

- Running a whole file test: We basically just provide `cargo t filename`,
  where the `filename` contains no extension.
  If you are in the `lib.rs` file for example, nothing will run
  as `cargo t` does not list `lib` as a module, but the main route.
  Also: We are not matching exactly,
  so if some other module contains the name of this file
  and/or a test name contains the name of this file,
  it will run too.
- Running nearest test just looks above the current cursor
  for a function name. If a function name is encountered
  but it is not the function name of the test that is nearest,
  `cargo t function_name` will probably not find your test.
  We also do not match exactly at the moment,
  so you might get more hits with the filtering that you'd expect.
- Only Rust testing currently supported.

## What is to come

- Support to specify your own commands and run in tmux panes.
- Python support.

## What is outside the scope of this project

- Making this blazingly fast (it is fast enough, it's Rust after all).
- Catching every eventuality and always only running the intended test.

Note: While a non-goal is to always run only the intended test,
hemux should always run the wanted test (but it might run a few more...).

## Notes on inner workings

### What we will use to make this work

[Helix expansions] are variables that provide information about the environment.
Specifically, hemux uses the following:

- `%{buffer_name}`: The path of the currently open buffer/file
  relative to the folder [Helix] was started in.
- `%{language}`: The langugage that is currently loaded. This will be used
  to select the runner we are going to use.
- `%{cursor_line}`: The line number the cursor is currently on.
  This is used to search for the nearest test name.

Note: If you get these values out of any other editor,
you probably can use hemux as well.

## Is this for me?

Maybe, assuming you actually have the same use case as I do, the same workflow,
and/or if you are willing to tinker with the program.

## Should I bank on this project to be maintained for a long time?

No (or hopefully no)!
Hopefully [Helix] gets plugins and we will get a wonderful plugin to do this.
Or maybe I find something better and archive this project by just leaving a link.
No promises for the future, this is a tool for myself primarily.
If it is useful to you, even better!
I'm happy to discuss issues and ideas, but please do not expect me
to spend a significant amount of time on this.
Please feel free to fork, build on it, or just take the idea and build your own.
I'd be happy to hear what you make.
And if your plugin/solution/etc. is better than this solution,
then we hit the "hopefully no" part of the first line in this paragraph :)

## Why hemux?

Well, it's of course a word play.
I have previously used the [vimux] plugin for vim.
The rest should now be obvious.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

[Helix]: https://helix-editor.com/
[Helix expansions]: https://docs.helix-editor.com/command-line.html#expansions
[nextest]: https://nexte.st
[Regular Expressions 101]: https://regex101.com/
[tmux]: https://tmux.app/
[vimux]: https://github.com/preservim/vimux
