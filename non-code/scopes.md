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

### Idea: create a stack of scopes.

For example in example 1:
- First bracket : Add scope to stack
- Second bracket : Add scope to stack
- Thirs bracket : Add scope to stach
- Fourth bracket is closed : close most recent scope on stack, so scope 3
- Fith bracket is open, add a scope to the stack
- Sixth bracket is closed, close the last scope of the stack (scope 4)
- Seventh bracket is closed, close 2nd scope
- Eighth bracket is closed, close 1st scope