# qx

A simple "productivity booster" tool to quickly run a set of actions.

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
envs:
  my-game:
    actions:
      # Open Godot in the project folder
      - type: run
        target: "C:\\godot\\godot.exe"
        working_directory: "C:\\godot\\my-game"

      # Open VS Code in the project folder
      - type: run
        target: "C:\\Users\\Me\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe"
        args: ["."]
        working_directory: "C:\\godot\\my-game"

      # Open Godot documentation
      - type: run
        target: "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe"
        args: ["-url", "https://docs.godotengine.org"]

      # Open some music
      - type: run
        target: "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe"
        args: ["-url", "https://musicforprogramming.net"]
```

Now, I can type `qx boot my-game`.
I can also type `qx boot m`, because I only have one environment in my list that matches `m`.  
And that's it.

_What ? You are on Windows and you don't want to use the terminal ?_  
Type <kbd>Win+R</kbd>, then `qx boot m` in the "Execute" window.

## TODO

- Simple variable system and override system
- Define more actions
- Find how to use PATH on Windows
- Setup cargo-dist
- Setup CI
- Add TUI ("qx -i")
- Add tests
- Setup abstractions where needed