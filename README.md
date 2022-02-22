a simple proc macro for Axum IntoResponse trait

```rust
#[derive(Serialize, Deserialize, IntoResponse, Debug)]
pub struct ErrorResponse {
    pub code: String,
    pub msg: String,
    pub errors: Vec<ErrorResponse>,
}
```
