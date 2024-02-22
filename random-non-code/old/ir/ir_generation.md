```rust
struct IRGenerator {
    stack: Vec<Instruction>,
}

impl IRGenerator {
    fn new() -> Self {
        IRGenerator { stack: vec![] }
    }

    fn generate_ir(&mut self, code: &str) {
        // Code parsing logic here to identify brackets and build IR
        for c in code.chars() {
            match c {
                '{' => {
                    self.stack.push(Instruction {
                        inst_type: Type::Scope,
                        parameters: vec![],
                    });
                }
                '}' => {
                    if let Some(scope) = self.stack.pop() {
                        // Handle the parsed scope here
                        // You can add this scope to the parent scope or handle it accordingly
                        // For example:
                        if let Some(parent_scope) = self.stack.last_mut() {
                            parent_scope.parameters.push(scope);
                        } else {
                            // This is the top-level scope
                            // Do whatever you need to do with the parsed scope
                        }
                    } else {
                        // Error handling for mismatched brackets or empty stack
                    }
                }
                _ => {
                    // Other parsing logic
                }
            }
        }
    }
}

// Usage
fn main() {
    let mut generator = IRGenerator::new();
    let code = r#"
    const a: int = 7

    main() {
        {
            print(a); print(a);
            print(a);
        } // the compiler should understand this closing bracket isn't for main
    }
    "#;

    generator.generate_ir(code);
    // Now, the 'generator' object will contain the generated intermediate representation
}
```