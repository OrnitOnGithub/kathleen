# Variable assignments

## Let binding

### Int (u64)

`let varname int = 12;`

Allocates a 64 bit integer called `varname`.

### String

NOT IMPLEMENTED

`let varname str = "hello";`

Allocates a dynamic string to memory.

## Constants

### Constant String

`const varname str = "hello";`

Defines a constant string.

### Constant Integer

NOT IMPLEMENTED

`const varname int = 12;`

Defines a 64 bit integer.

# Output

## Print

`print(varname1 varname1);`

Prints varname1 and varname2 consecutively, without appending a newline.

## Print Line

`println(varname1 varname2);` or `println();`

Prints varname1, varname2 and a newline or just the newline.

# Logic

## Loop (named)

```c
loop example {
  // do something
}
```

`loop <name> {<content>}` defines a loop with a name that infinitely loops over whatever is between the two curly braces. All loops have to be named. `loop {<content>}` is not allowed. 

### Break (named)

```c
loop example {
  break example;
}
```

`break <name>` exits the named loop. In the example above, it will exit the loop `example`. This also allows inner loops to exit outer loops:

```c
// A "Hello World!" program.
const hello str = "Hello ";
const world str = "World!";

loop outer {
  print(hello);
  loop inner {
    println(world);
    break outer;
  }
}
```

# Maths

## Increment

`inc varname`

Increments the integer `varname`.

```c
let var int = 1;
println(var);
inc var;
println(var);
```
output:
```
1
2
```
