# Rofi Taskwarrior

Some glue to use
[Taskwarrior](https://taskwarrior.org/)
with
[Rofi.](https://github.com/davatorium/rofi)
The practical upshot
is that you can bind a key in a Linux window manager
and have a quick view of your 10 most urgent tasks,
and then use fuzzy searches
to refine that view.
Additionally,
you can perform some actions on the tasks:
start and stop,
mark them done,
and edit them.

## Use

Try:
```shell
> rofi -modi tasks:rofi-taskwarrior -show tasks
```

What you're seeing is purely
a wrapper around `task`.
It won't do anything you couldn't do with `task` from the command line.
I find that having access to tasks from a keystroke
(as opposed to having to bring up a terminal all the time)
makes me more likely to use them.

Additionally,
you can add tasks by typing their description
into the rofi prompt,
which makes it quick to record new tasks.

For this initial release,
the "Alt-1" indications in the task menus
is merely aspirational.
I've got a todo to implement them.

### Daily use

Having to type out a `rofi` command in the terminal
sort of defeats the streamlining purpose here.
What you really want to do
is to arrange for that command
to be launched at a keystroke.
How you do that depends on you window manager;
my advice will come from Xmonad,
but PRs with example configuration are welcome.

### Terminal

You may find that,
when you edit tasks,
you'd rather have a different terminal -
in which case, set a `TERMINAL` environment variable.
`rofi-taskwarrior`
uses `rofi`'s `rofi-sensible-terminal`,
which defaults to a specific ordering of
potentially installed terminal applications.
If you don't like its choice,
the `TERMINAL` environment variable overrides that.
Likewise, the choice of editor is based on
`task`'s global configuration.

## Future Work

There's a few little features I still have in mind,
but I'm sure other ideas will surface with usage.

It might be handy
to spin off the Taskwarrior module
as its own crate.
Reach out if that would be useful to you.

## License

For the time being, `rofi-taskwarrior` is licensed under the
LicenseZero Parity license.
When the L0 successor opens up,
I'll consider next steps regarding licensing.
