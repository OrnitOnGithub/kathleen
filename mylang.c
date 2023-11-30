// Using .c file extension just to have some syntax highlighting inside vscode

const a: int = 7

main() {
    {
        print(a); print(a);
        print(a);
    } // the compiler should understand this closing bracket isn't for main
}
