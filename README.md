# qx

A simple "productivity booster" tool to quickly run a set of actions.

- [Latest release and installation instructions](https://github.com/Srynetix/qx/releases)
  - Thanks to [cargo-dist] !
- [CHANGELOG](./CHANGELOG.md)
  - Thanks to [git-cliff] !

## What? Why?

I'm often switching between projects which can greatly differ.

It can be:

- Game development
- System or tool development
- Web development
- Music making
- Drawing
- Reading
- Learning
- Gaming
- ...other things?

And each time the context switching is not easy, and I would like to have something to keep me focused when I work on these things.

So here's an example.

Suppose I'm currently developing a Godot game, called "My Game".  
When developing this project, I need:

- A VS Code instance opened in the right folder
- A Godot instance opened in the right folder
- A web browser opened to the official Godot Documentation
- Some kind of music (like https://musicforprogramming.net)

Using `qx`, you can define "environments", containing a set of actions.

Here's a sample for the tasks I described above, as a `qx` configuration file:

```yaml
variables:
  my_project_directory: "C:\\godot\\my-game"

environments:
  my-game:
    actions:
      # Open Godot in the project folder
      - type: run
        target: "C:\\godot\\godot.exe"
        working_directory: "${my_project_directory}"

      # Open VS Code in the project folder
      - type: vscode
        target: "${my_project_directory}"

      # Open Godot documentation
      - type: open_url
        target: "https://docs.godotengine.org"

      # Open some music
      - type: open_url
        target: "https://musicforprogramming.net"
```

Now, I can type `qx my-game`.
I can also type `qx m`, because I only have one environment in my list that matches `m`.  
And that's it.

_What ? You are on Windows and you don't want to use the terminal ?_  
Type <kbd>Win+R</kbd>, then `qx m` in the "Execute" window.

And if you want, you can also use a _terminal user interface_ by typing `qx -i`.

## Available actions

- **run**: run an executable
  - **target**: executable to run
  - _args_: arguments to pass
  - _working_directory_: working directory

- **open_url**: open URL in the default web browser
  - **target**: URL to open

- **open_file**: open a file/folder using the default associated app
  - **target**: file/folder to open

- **vscode**: open a VS Code instance on a target
  - **target**: file/folder to open in VS Code

- **show_message**: display a message in the console
  - **message**: message to show

## TODO

- Define more actions
- Add tests
- Setup abstractions where needed

[cargo-dist]: https://github.com/axodotdev/cargo-dist
[git-cliff]: https://github.com/orhun/git-cliff/