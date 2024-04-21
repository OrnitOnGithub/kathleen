How to handle functions?

IDEA 1: call create_instructions inside ir_generator once per function/loop/scope/cjmp, have another program that pre-processes function (finds functions and separates them)

IDEA 2: Have the tokenizer (or an intermediate program) split scopes and have each scope compiled separately?


data types
- static integer
- static string
- dynamic string
- vector
- static booleans

static x int
static x str
let    x str
let    x vec<t>
static x bool
