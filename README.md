# ftw
> A CLI tool to manage your godot-rust project!

## Table of contents
* [General Information](#general-information)
* [Setup](#setup)
* [Usage](#usage)
* [Contact](#contact)

## General Information
This is a tool to help you manage your game project by providing commands to (1) create a project, (2) create a class, (3) create a singleton class, (4) build the library, (5) export your game, (6) run your project (and more to come in the future!). Its like [rails](https://rubyonrails.org/) but for game development :wink:.

## Setup
It leverages tools like [godot, godot-headless and godot-server](https://godotengine.org/download) to make it all work! In Linux you can install all godot, godot-headless and godot-server, on others only godot. For additional setup instructions, check the [wiki](https://github.com/godot-rust/godot-rust-template/wiki) of the default template.

```shell
$ cargo install ftw # to install
$ cargo install --force ftw # to upgrade ftw
```

## Usage
### ftw new &lt;project-name&gt; [template]
#### Creates a new project directory
```shell
$ ftw new my-awesome-game # this creates a new project using the default template
$ ftw new my-awesome-game default # same as above
$ ftw new my-awesome-game /path/to/custom/template # creates a new project using a custom template
```
> Note: The custom template should have same structure as the [default template](https://github.com/godot-rust/godot-rust-template)

### ftw class &lt;class-name&gt; [node-type]
#### Creates a class
```shell
$ ftw class MyHero # creates a class called `MyHero` that is deriving from `Node` as default
$ ftw class MyHero Area2D # creates a class that derives from `Area2D`
```
> Note: This creates the following files `rust/src/my_hero.rs`, `godot/scenes/MyHero.tscn` and `godot/native/MyHero.gdns` then adds the class inside `rust/src/lib.rs`

##### You could also organize rs, tscn and gdns files into submodules or subfolders
```shell
$ ftw class heros/marvel/avengers/IronMan Area2D # creates a class that derives from `Area2D`
```
> Note: This creates the following files `rust/src/heros/marvel/avengers/iron_man.rs`, `godot/scenes/heros/marvel/avengers/IronMan.tscn`, `godot/native/heros/marvel/avengers/IronMan.gdns` and `mod.rs` files in each subfolder in `rust/src` then adds the class inside `rust/src/lib.rs`

### ftw singleton &lt;class-name&gt;
#### Creates a singleton class for autoloading
```shell
$ ftw singleton MySingleton # creates a class called `MySingleton` that derives from `Node`
```
> Note: This creates the following `rust/src/my_singleton.rs` and `godot/native/MySingleton.gdns` then adds the class inside `rust/src/lib.rs`

##### You can also organize the files into submodules/subfolders as in `ftw class` command
```shell
$ ftw singleton network/Network # creates a class called `Network` that derives from `Node`
```

### ftw build [target] [build-type]
#### Builds the library for a particular target
```shell
$ ftw build # builds the library for your current platform as target using `debug` as default
$ ftw build linux-x86_64 # builds the library for the `linux-x86_64` platform using `debug` as default
$ ftw build linux-x86_64 debug # same as above
$ ftw build linux-x86_64 release # builds the library for the `linux-x86_64` platform using `release`
```
#### [target] can be one of the following
- android-aarch64
- android-arm
- android-x86
- android-x86_64
- ios-aarch64
- linux-x86
- linux-x86_64
- macos-x86_64
- windows-x86-gnu
- windows-x86-msvc
- windows-x86
- windows-x86_64-gnu
- windows-x86_64-msvc
- windows-x86_64

### ftw export [target] [build-type]
#### Exports the game for a particular target
```shell
$ ftw export # exports the game for your current platform as target using `debug` as default
$ ftw export linux-x86_64 # exports the game for the `linux-x86_64` platform using `debug` as default
$ ftw export linux-x86_64 debug # same as above
$ ftw export linux-x86_64 release # exports the game for the `linux-x86_64` platform using `release`
```

### ftw run [machine-type]
#### Builds the library using `debug` then runs your game
```shell
$ ftw run # runs the game on desktop
$ ftw run desktop # same as above
$ ftw run server # runs the game as a server
# enjoy! 😆
```

## Custom executables

If you have custom executables to run godot, for example if you have a shell/batch script which do some stuff first before running godot, you can configure using the following inside your project...

### .ftw

You can create a `per-project` configuration file at your project root named `.ftw` with the following contents...

```ini
[ftw]
godot-exe=/path/to/custom/godot-script
godot-headless-exe=/path/to/custom/godot-headless-script
godot-server-exe=godot-server-script # assuming it's on $PATH
```

> Note: Having the .ftw file and the keys inside the `.ftw` file are all optional. If you don't provide them, the defaults (godot, godot-headless and godot-server) will be used. For Windows users use forward-slashes instead of back-slashes (e.g. godot-exe=D:/path/to/godot/bin/godot.windows.tools.64.exe)

## Contact
Michael Angelo Calimlim `<macalimlim@gmail.com>`
