# osxnav
A keynav replacement for osx written in Rust. Inspired by keynav and xeasymotion. Mostly just a fun project to work on.


## Build
Run:
`cargo build`


## Run
Run:
`cargo run`

Or, after building, run the executable `./osxnav` in `target/debug`.

This executable can be bound to an automation in Automator, which can then be bound to a key binding: 
- Automator > New > Quick Action > Run Shell Script > `./path/to/osxnav`.
- System Preferences > Keyboard > Shortcuts > Services

## Features
```
h: cut-left
j: cut-down
k: cut-up
l: cut-right

shift + h: translate-left
shift + j: translate-down
shift + k: translate-up
shift + l: translate-right

enter: click
esc: escape
```

## Notes
overwrite method:
http://sasheldon.com/rust-objc/objc/declare/index.html
https://stackoverflow.com/questions/30776875/how-to-set-canbecomekeywindow/30779006
