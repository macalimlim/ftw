{%- assign modules = modules | split: "|" | compact | sort -%}
{%- for module in modules %}
mod {{module}};
{%- endfor %}

use gdnative::prelude::{godot_init, InitHandle};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
{%- assign classes = classes | split: "|" | compact | sort -%}
{%- for class in classes %}
    handle.add_class::<{{class}}>();
{%- endfor %}
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
