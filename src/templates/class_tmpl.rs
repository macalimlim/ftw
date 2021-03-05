use gdnative::api::*;
use gdnative::prelude::*;

/// The {{class_name}} "class"
#[derive(NativeClass)]
#[inherit({{node_type}})]
#[register_with(Self::register_builder)]
pub struct {{class_name}} {
    name: String,
}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl {{class_name}} {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("{{class_name}} builder is registered!");
    }

    /// The "constructor" of the class.
    fn new(_owner: &{{node_type}}) -> Self {
        godot_print!("{{class_name}} is created!");
        {{class_name}} {
            name: "".to_string(),
        }
    }

    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are "attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    unsafe fn _ready(&mut self, _owner: &{{node_type}}) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        self.name = "{{class_name}}".to_string();
        godot_print!("{} is ready!", self.name);
    }

    // This function will be called in every frame
    #[export]
    unsafe fn _process(&self, _owner: &{{node_type}}, delta: f64) {
        godot_print!("Inside {} _process(), delta is {}", self.name, delta);
    }
}
