# ftw

## Commands

### new

creates a new godot-rust project

```shell
$ ftw new my-awesome-game # creates a godot-rust project using the default template

$ ftw new my-awesome-game default # same as above

$ ftw new my-awesome-game /path/to/some/template # creates a godot-rust project using a custom template
```

### class

creates a class file in `rust/src/`, a tscn file in `godot/scenes`, a gdns file in `godot/native` and initializes the class in `rust/src/lib.rs`

```shell
$ ftw class Player Node # creates a class with Node as the node type, node type can be any type supported by godot

$ ftw class Player # same as above, defaults to Node as node type

$ ftw class Player Node2D # creates a class with Node2D as node type
```
