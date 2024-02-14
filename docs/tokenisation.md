## Documentation for the tokeniser

The tokeniser is a funtion inside `src/tokenizer.rs`, `pub fn tokenize(lines: Vec<String>) -> Vec<Token>` called in `src/main.rs`

The tokenizer works in multiple steps, to finally provide a Vector of `Token` structs.

### Step 1 -- Separate by whitespace

### Step 2 -- Separate special characters

prior to this step, one token could totally have been "main(arg1,arg2)"

It is now necessary to separate things by brackets, commas, etc.

The list of characters to separate is:

```
( ) [ ] { }
- + * % / & = < > ! | ^
, . ; : ' "
```

### Step 3 -- Remove comments

If two consecutive "/" tokens are met, delete them and the rest of the line.

### Step 4 -- Turn into `Token` struct

```rust
{
    token: String,          // the token itself
    line: usize,            // which line this at
    token_number: usize,    // which token in the line this is (1st, 2nd...)
}
```