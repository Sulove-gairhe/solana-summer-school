# Rust Relearn Files

These files mirror your recent Git commits with fresh examples:

- `src/bin/relearn_struct_impl.rs`: structs, `impl`, methods, `&self`, and `&mut self`
- `src/bin/relearn_result_match.rs`: returning `Result<(), String>` and handling `Ok` / `Err`
- `src/bin/relearn_question_operator.rs`: chaining fallible steps with the `?` operator
- `src/bin/relearn_derive_debug_clone.rs`: using `#[derive(Debug, Clone)]`

Run them from inside `vault-playground`:

```powershell
cargo run --bin relearn_struct_impl
cargo run --bin relearn_result_match
cargo run --bin relearn_question_operator
cargo run --bin relearn_derive_debug_clone
```

Practice ideas:

- Change the starting balances so the success path becomes the error path.
- Add a `deposit` method to the `Result` examples.
- Replace `String` errors with `&'static str`.
- Add one more chained step to `relearn_question_operator.rs`.
