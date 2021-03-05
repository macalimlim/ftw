{%- assign modules = modules | split: "|" | compact | sort -%}
{%- for module in modules %}
pub mod {{module}};
{%- endfor %}
