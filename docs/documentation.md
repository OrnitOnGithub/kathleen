# Tokeniser (tokenizer)

The tokenizer is a funtion inside `src/tokenizer.rs`, `pub fn tokenize(lines: Vec<String>) -> Vec<Token>` called in `src/main.rs`

The tokenizer works in multiple steps, to finally provide a Vector of `Token` structs.

## Step 1 -- Separate by whitespace

## Step 2 -- Separate special characters

prior to this step, one token could totally have been "callfunction(arg1,arg2)"

It is now necessary to separate futher by things like brackets, commas, etc.

The list of characters to separate is:

```
( ) [ ] { }
- + * % / & = < > ! | ^
, . ; : ' "
```
```rust
let special_chars: HashSet<char> = [
    '(', ')',                       
    '{', '}',                       
    '[', ']',                       
    '<', '>',                       
    '\'', '"',                      
    '!', '|', '&',                  
    ',', '.', ':', ';',
    '+', '*', '/', '-', '=', '^',
].iter().cloned().collect();
```

## Step 3 -- Remove comments

If two consecutive "/" tokens are met, delete them and the rest of the line.

## Step 4 -- Turn into `Token` struct

```rust
{
    token: String,          // the token itself
    line: usize,            // which line it is at
    token_number: usize,    // index of token in line
}
```

Turning each token into a struct that describes it is great for error handling, as soon as a problematic token is encountered it can easily be passed to `error::print_error`, who will easily know where the token is located.