```rust

fn recursive_instruction() -> Instruction {
    // we can have an if here for example
    if CONDITION {
        return Instruction { 
            inst_type: Type::Scope,
            parameters: vec![recursive_instruction()],
        }
    }
    else
    {
        return Instruction { 
            inst_type: Type::Scope,
            parameters: vec![],
        }
    }
}
```

idea 1: flawed
We call the function, the function handles a keyword.
After this the keyword gets deleted, and the function
is called recursively as shown above.
