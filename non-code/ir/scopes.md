example 1
```c
{ // Scope 1
    { // Scope 2
        { // Scope 3

        } // Scope 3

        { // Scope 4

        } // Scope 4
    } // Scope 2
}
{ // Scope 5

} // Scope 5
```
example 2
```

{
    Scope 1
    {
        Scope 2
    }
}
{
    Scope 3
}
{
    {
        {

        }
    }
}

```

Goal: Separate contents of code into scopes.

### Idea 1: create a stack of scopes.

For example in example 1:
- First bracket : Add scope to stack
- Second bracket : Add scope to stack
- Thirs bracket : Add scope to stach
- Fourth bracket is closed : close most recent scope on stack, so scope 3
- Fith bracket is open, add a scope to the stack
- Sixth bracket is closed, close the last scope of the stack (scope 4)
- Seventh bracket is closed, close 2nd scope
- Eighth bracket is closed, close 1st scope


### Idea 2: Don't make scopes an actual data structure.

The current scope a variable or instruction is in is just numerically kept track of

```rust
/// Represents an instruction or a set of instructions
/// in the intermediate representation.
/// `inst_type` defines the type of instruction, while `parameters`
/// contains a vector of additional instructions or arguments
/// associated with this instruction.
#[derive(Debug)]
pub struct Instruction {
    inst_type: Type,
    parameters: Vec<Instruction>,
    scope: usize, // new
}
```

then we can have instructions like StartofScope and EndofScope,
which increment or decrement the scope counter respectively.

though this seems a little messy idk