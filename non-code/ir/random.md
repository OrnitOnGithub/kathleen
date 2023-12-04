```rust

fn recursive_instruction() -> Instruction {
    if some_condition {
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

```rust
pub fn generate_ir(tokenized_lines: Vec<Vec<String>>) -> Vec<Instruction> {

    struct IR {
        instructions: Vec<Instruction>,
    }
    impl IR {
        fn new() -> Self {
            IR { instructions: Vec::new() }
        }

        /// This function will call itself recursively to generate the IR.
        fn generate_ir(&mut self, mut tokenized_lines: Vec<Vec<String>>) {
            
            for line in tokenized_lines.iter_mut() {
                // handle the keyword and then remove it.
                // It needs to be removed not to frick up the recursive
                // functions.
                while let Some(word) = line.pop() {
                    match &word as &str {

                        "{" => {
                            self.instructions.push(
                                Instruction {
                                    inst_type: Type::Scope,
                                    parameters: vec![],
                                }
                            )
                        }

                        "}" => {
                            if let Some(scope) = self.instructions.pop() {
                                if let Some(parent_scope) = self.instructions.last_mut() {
                                    parent_scope.parameters.push(scope)
                                }
                            }
                        }
                        
                        _ => {
                            self.instructions.push(
                                Instruction {
                                    inst_type: Type::Undefined,
                                    parameters: vec![] 
                                }
                            )
                        }
                    }
                }
            }
        }
    }

    let mut intermediate_representation = IR::new();
    intermediate_representation.generate_ir(tokenized_lines);
    return intermediate_representation.instructions;
}

```