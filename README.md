## How To

This project is still in the early stages of development.

Enter a file containing one closed term in arrow function notation. For example, the combinator **S** can be written as:
```
x => y => z => x z (y z)
```

The smallest closed recursive term is ω:
```
x => x x
```

Which is typeable as:
```
A := [A] => B
A => B
```
Here, the first line is a definition, and the second line contains the type expression itself. This type encodes [Curry's paradox](https://en.wikipedia.org/wiki/Curry's_paradox) with a guarded cyclic definition.
