<h1 align=center><code>friggen</code></h1>
<hr/>

> A friggen task runner for simpletons.
> Y'know... like `friggen build` or `friggen test` and stuff. Hehe. Get it? Hehe.
<hr/>

[![Build Status](https://img.shields.io/github/actions/workflow/status/troykinsella/friggen/ci.yml?branch=master&style=round-square)](https://github.com/troykinsella/friggen/actions/workflows/ci.yml?query=branch%3Amaster)
[![Downloads](https://img.shields.io/github/downloads/troykinsella/friggen/total.svg)](https://github.com/troykinsella/friggen/releases)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)

![friggen_just_work.png](friggen_just_work.png)
Computers. Am I right?

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

## Installation

Stroll your tookus on over to [Releases](https://github.com/troykinsella/friggen/releases/latest) and
download the right archive for your system. Don't see an archive for your
system? Well... this is awkward. File a ticket?

Extract it and slap the `friggen` binary in `/usr/local/bin` or something. I dunno.

## Quickstart

### Rock a `friggenfile` Lickity-Split:

```
# I'm a comment. lol.

# I'm an environment variable!
dog_status = good

# Here's a task definition.
# Whatever's indented after it is the task script.
# The default shell is 'bash'. Psh.
check:
  shellcheck bin/*
  test -f version

# Slap some task names on the end to depend on 'em.
build: check
  make build # lol, that'd be hilarious, eh?

install-stuff:
  # Get all 4 trillion of your dependencies installed
  npm install

# Giv'r a hash bang to do a different interpreter.
# Space-separated task dependencies to have multiples.
test: install-stuff build
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

### Friggen Use It

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

## The Friggen `friggenfile`

Name it `friggenfile` or `.friggenfile`, and place it at the root of your project.

### Defining a Task

Put a task name, like `do-somethin`, at the beginning of a line. 
It's kind of a typical identifier! It can have `a-z`, `A-Z`, `0-9`, `_`, and `-`.
Just don't put a number at the front, for some reason, I dunno.

```
do-somethin:
  echo "somethin"
```

You can make a ton of 'em like that. Doesn't matter what order you put 'em.

```
task1:
  echo 1
task_B:
  echo B
task4:
  echo 4
```

### Task Dependencies

If ya go ahead and rock another task name after the colon (what's that, the small intestine??? amirite?? lol), 
y'end up making a task depend on another task! Wild, eh!?

```
# fooh can't live without bahr... pathetic.
fooh: bahr
  echo "hehe"
```

So if ya ran `friggen fooh`, sure that's the task you're gonna get, but
guess what? It's gonna run `bahr` first.

Want to depend on another task? And another? Just keep slappin' them on 
the butt-end of that same line, separated by whitespace! They run in-order, left to right, just like
we read cereal boxes and court orders and stuff.

By default, a task will only run once, even if it's depended upon multiple times.
Even depended upon from different tasks. Yeah.

### The Task Script

Everything below the task name that's indented is part of the task's script.

```
my_task:
  if ./do_something_ridiculous; then 
    echo "haha, did you see that?"
  fi
```

"Indented by how much??", you might ask, but let me cut you off right there,
because I'm about to tell you. Chill out.

See how indented your first line in the script is? That's how much is trimmed off each line in the whole
task script, okay?

Like, if you ran `my_task`, up there, it would basically be like you're running a script like this:

```bash
#!/usr/bin/env bash
if ./do_something_ridiculous; then 
  echo "haha, did you see that?"
fi
```

`friggen` took all that ugly, ugly whitespace off the front of each line. It's the whitespace you don't want, 
and the whitespace you don't need. Don't ever say I don't got your back.

### The Task Script Shell

The default shell is... guess......... `bash`. It's `bash`. I know.

But you can put a hash bang, or a shebang, or whatever, as the first line of the script
to specify whatever interpreter you like.

```
go-on-say-something:
  #!/usr/bin/env ruby
  puts "aw ya, son"
```

### Task Documentation

If ya blast a comment starting with `##` in front of a task definition, your wise words
will show up in the task listing (when you run `friggen` with no args).
Add a couple of 'em, even. The world is your oyster.

Like this, or something:

```
# I'm a friggenfile!

## Deploy right to prod
send-er:
  ./deploy_it_right_in_the_prod.sh
```

Then, get this...

```
$ friggen
friggen tasks:
send-er

╭──( send-er )──○
│ Deploy right to prod
╰──○
```

lol

### Assigning Variables

Variables. Can't live with 'em. Can't live without 'em. I just made that up, but...

Here's how you set some in a `friggenfile`. They end up just being your
run o' the mill environment variable available to all task scripts.

```
# No quotes: (Take the value until the end of the line)
cat_size = extremely large

# Single quotes:
hotdog_flavour = 'gross'

# Double quotes:
funny_episode = "That time Kramer came through the door all fast and weird haha"

# Triple quotes:
a_short_story = """Then Micheal, 
"Mr. Smartypants", showed up, being all, 
"I know how to fix that problem", and stuff.
It sucked.
"""

# Command substitution:
liver_and = $(shuf -n 1 onion_varieties.txt)
```

Pretty self-explanatory.

`friggen` doesn't do any kind of crazy stuff with escaping and nested quotes and stuff.
That's on purpose. Like, you have three different kinds of quotes to work with,
and you wanna escape and nest stuff? What is this? A programming language?
Maybe settle down. Grab a hot sandwich or something.

## How `friggen` Does Stuff

### The Default Task

Haha, there isn't one. I'll get ya a refund going, here. 

But seriously, if you want to friggen do something, just say what you want.

Running `friggen` with no args will show you what's up for grabs.

### Task Execution Order

Check this out.

```
# It's a friggenfile in here

bazz: 
  echo "hehe bazz"

bizz:
  echo "hehe bizz"

bahr: bizz
  echo "hehe bahr"

fooh: bahr bazz
  echo "hehe fooh"
```

All this business would run in this order:

```bash
$ friggen -q fooh
hehe bizz
hehe bahr
hehe bazz
hehe fooh
```

By the way, with `-q` I just told it to shut up a bit (only print the task output, hehe).

## License

`friggen` is free and open source.
Print out the source code and use it to make a
paper mache cat to be your friend. I don't care.
All code in this repository is dual-licensed under
either of the following, at your option:

* MIT License ([`LICENSE-MIT`](LICENSE-MIT) or http://opensource.org/licenses/MIT)
* Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
