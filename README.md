A macro for conveniently navigating `serde_json::Values` without having
to do all the error handling manually. This is especially useful
for situations where you have do not have consistent or predictable json documents
and you want to try multiple paths to find the one where the relevant information
is located.

# Example
```rust
use serde_json::{json, Value};
use json_nav::{json_nav, JsonNavError};

let value = json!({
    "code": 200u16,
    "success": true,
    "payload": {
        "features": [
            "serde",
            "json"
        ]
    }
});
let first_feature = json_nav! {
    value => "payload" => "features" => 0; as str
};
assert_eq!(Ok("serde"), first_feature);
let type_error = json_nav! {
    value => "payload" => "features" => 1; as object
};
assert_eq!(Err(JsonNavError::TypeMismatch { expected: "object" }), type_error);
let path_error = json_nav! {
    value => "payload" => "failure"
};
assert_eq!(Err(JsonNavError::Navigation { path: "value.payload.failure" }), path_error);
```
