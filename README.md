# One

My first attempt at making a programming language.

## data

One has a single data type: a variable length bytestring that is interpreted by default as a
little-endian unsigned integer, a string, or raw binary data. There are no named variables in one.
Instead, all operations are performed on a global stack structure. Any literals in a program are
pushed directly to the stack, functions pop their arguments and push their returns, and so on.

```
main {
    0x10
    0x05
    add
}
```

In this simple one program the stack starts empty, then the values `0x10` and `0x05` are pushed to
the stack when the add function is called, it removes the top two values from the stack and replaces
them with their sum, `0x15`.

## flow control

One has a single test operator built into the interpreter: `unzerop`, with another, `zerop` provided
by the one prelude.  Flow control in one is governed by the idea of skipping, a rudimentary way of
dealing with one's lack of nested structure.  The one interpreter has a 'skip counter' that if
nonzero decrements and passes over the current function call or literal without executing it.  The
`unzerop` function pops the top value from the stack and checks if it is nonzero. If it is not
nonzero, it sets the skip counter to one.

```
dosomethingifnonzero {
    unzerop
    dosomething
}
```

The above function executes `dosomething` only if the top value in the stack is nonzero.

If a function call occurs at the last position in a function, one will replace the current entry in
the call stack instead of creating a new entry, giving one zero cost tail recursion.


```
loopdecrement {
    0x1
    sub
    dosomething
    unzerop
    loopdecrement
}

dosomething16times {
    0x10
    loopdec
}
```

It is worth noting that the skip counter persists as the call stack is unwound, allowing programmers to create their own tests.

```
lessthan5 {
    0x4
    sub
    zerop
}

myskip1 {
    0x1
    skip
}

greaterthan4 {
    lessthan5
    myskip1
}
```
