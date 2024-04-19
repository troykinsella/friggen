<h1 align=center><code>friggen</code></h1>
<hr/>

> A friggen task runner for simpletons.
> Y'know... like `friggen build` or `friggen test` and stuff. Hehe. Get it? Hehe.
<hr/>

[![Build Status](https://img.shields.io/github/actions/workflow/status/troykinsella/friggen/ci.yml?branch=master&style=round-square)](https://github.com/troykinsella/friggen/actions/workflows/ci.yml?query=branch%3Amaster)
[![Downloads](https://img.shields.io/github/downloads/troykinsella/friggen/total.svg)](https://github.com/troykinsella/friggen/releases)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)

## Why Would...

* Tasks are just shell scripts, and that's about it, I guess.
* Variables are just environment variables and stuff.
* Not really any fancy-pants task runner syntax.
* No surprises, like a default task being different or changing up on ya. I hate surprises.

## No Really, Why

* Yeah, you should probably just use `make` or `just` or something. I don't care.

## Whatever, How's It Work?

You define some friggen tasks and uh...
they can have dependencies if you want.
Task bodies are just shell scripts.

You can set the shell with a hash bang (`#!`), if that tickles ya.

### Rock a `friggenfile`:

```
# I'm a comment. lol.

# I'm an environment variable!
dog_status = good

# Here's a task definition.
# Whatever's indented after it is the task script.
# The default shell is 'sh'. Psh.
check:
  shellcheck bin/*
  test -f version

# Slap some task names on the end to depend on 'em.
build: check
  make build # lol, that'd be hilarious, eh?

install-dev-tools:
  npm install --or-whatever-mistakes-you-kids-use

# Giv'r a hash bang to do a different interpreter.
# Space-separated task dependencies to have multiples.
test: build install-dev-tools
  #!/usr/bin/env python
  print("haha I'm slow")

## Use double-hash comments to write task docs.
## This stuff shows up in the friggen task list.
release: test
  echo "consider it released!"

# Reference environment variables.
# Friggen even loads a .env file! How about them apples?
say-hello:
  echo "Hi $USER. I have some smoked oysters in my pocket. Want one?"
```

## Friggen Use It

Find out what tasks you can run:

```
$ friggen
friggen tasks:
bar, foo

╭──( bar )──○
│ » foo
│ Bar it down to bar town, but foo it first.
╰──○
╭──( foo )──○
│ Run a foo so hard.
╰──○
```

Run a task:

```
$ friggen bar
○──( » start: foo )──○
Fetching radical...
○──( ✓ done: foo )──( 0.003 sec. )──○
○──( » start: bar )──○
Executing tubular...
○──( ✓ done: bar )──( 0.014 sec. )──○
○──( ★ done )──( 0.017 sec. )──○
```

Run `friggen -h` to find out more business. Or don't.

## License

`friggen` is free and open source.
Print out the source code and use it to make a
paper mache cat to be your friend. I don't care.
All code in this repository is dual-licensed under
either of the following, at your option:

* MIT License ([`LICENSE-MIT`](LICENSE-MIT) or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
