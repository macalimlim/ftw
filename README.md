# ftw
> A CLI tool to manage your godot-rust project!

## Table of contents
* [General Information](#general-information)
* [Setup](#setup)
* [Usage](#usage)
* [Contact](#contact)

## General Information
This is a tool to help you manage your game project by providing commands to (1) create a project, (2) create a class, (3) create a singleton class, (4) build the library, (5) run your project (and more to come in the future!). Its like [rails](https://rubyonrails.org/) but for game development :wink:

## Setup
It leverages tools like [cargo-generate](https://github.com/ashleygwilliams/cargo-generate) and [godot](https://godotengine.org/) (obviously!) to make it all work!

```shell
$ cargo install cargo-generate
$ cargo install ftw
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

### ftw singleton &lt;class-name&gt;
#### Creates a singleton class for autoloading
```shell
$ ftw singleton MySingleton # creates a class called `MySingleton` that derives from `Node`
```
> Note: This creates the following `rust/src/my_singleton.rs` and `godot/native/MySingleton.gdns` then adds the class inside `rust/src/lib.rs`

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
- ios-x86_64
- linux-x86
- linux-x86_64
- macos-x86_64
- windows-x86-gnu
- windows-x86-msvc
- windows-x86
- windows-x86_64-gnu
- windows-x86_64-msvc
- windows-x86_64

### ftw run
#### Builds the library using `debug` then runs your game
```shell
$ ftw run # enjoy! 😆
```

## Contact
Michael Angelo Calimlim `<macalimlim@gmail.com>`
