{%- assign pairs = module_class_name_pairs | split: "|" | compact | sort -%}
{%- for p in pairs -%}
{%-   assign it = p | split: "," -%}
{%-   assign module = it[0] %}
mod {{module}};
{%- endfor %}

use gdnative::prelude::{godot_init, InitHandle};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
{%- for p in pairs -%}
{%-   assign it = p | split: "," -%}
{%-   assign module = it[0] -%}
{%-   assign class_name = it[1] %}
    handle.add_class::<{{module}}::{{class_name}}>();
{%- endfor %}
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
