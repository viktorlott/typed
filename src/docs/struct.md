Use `{name}::ty` as a type.
```no_run
{source_code}
```
# Example
```no_run
fn get(input: {name}::ty) -> {name}::ty {{
    let field: {name}::ty = input;
    input
}}
```