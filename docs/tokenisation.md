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
+ * % / & = < > ! | ^
, . ; : ' 
```

