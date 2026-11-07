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
[keys.normal.space.t]  # hemux key bindings
s = ":sh hemux -m=test-suite -l=%{language} -p=%{buffer_name} -c=%{cursor_line}"  # Run test suite
f = ":sh hemux -m=test-file -l=%{language} -p=%{buffer_name} -c=%{cursor_line}"  # Run test in file
n = ":sh hemux -m=test-nearest -l=%{language} -p=%{buffer_name} -c=%{cursor_line}"  # Run nearest test
```

This will give the following shortcuts (where the comma means "followed by"):

- `Space, t, s`: Run the whole test suite
- `Space, t, f`: Run all tests in the current file
- `Space, t, n`: Run the nearest test

## Limitations

- Running a whole file test: We basically just provide `cargo t filename`,
  where the `filename` contains no extension.
  If you are in the `lib.rs` file for example, nothing will run
  as `cargo t` does not list `lib` as a module, but the main route.
  Also: We are not matching exactly,
  so if some other module contains the name of this file
  and/or a test name contains the name of this file,
  it will run too.
- Running nearest test just looks for above the current cursor
  for a function name. If a function name is encountered
  but it is not the function name of the test that is nearest,
  `cargo t function_name` will probably not find your test.
  We also do not match exactly at the moment,
  so you might get more hits with the filtering that you'd expect.
- Only Rust testing currently supported.

## What is to come

- Support to specify your own comments and run in tmux panes.
- Python support.
- Configuration files so you can tweak hemux to your configuration.
- Support for [nextest].
- Configurations for test runners (maybe).

## What is outside the scope of this project

- Making this blazingly fast.
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

### What test runners are currently supported?

- Rust: `cargo test`
- Python: `pytest`

### Configuration

None at first possible, but ultimately hemux might be able to be configuratble
from a toml file.

### What langugaes are supported?

Rust and Python.

## Is this for me?

Probably not, unless you actually have the same use case as I do and the same workflow
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
[tmux]: https://tmux.app/
[vimux]: https://github.com/preservim/vimux
