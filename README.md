## rlox-tree
A tree-walking interpreter for the language Lox described in [crafting interpreters](https://www.craftinginterpreters.com/).

## Running

To run in REPL mode:
`cargo run`

To run a script:
`cargo run tests/basic_operation.lox`

## Tests
Test programs are located in the `tests/` directory. Each program begins with a block of comments. the content of these comments are what the program should print when it is run.

## Features

Basic expressions:
```
Welcome to Lox REPL!
> print "Hello" + " " + "World!";
Hello World!
> print 5 * 4 / (3 + 2);
4
> print 6 > 7 or true;
true
```

Functions:
```
fun square(x) {
    return x * x;
}

print square(3); // prints 9
print square(6); // prints 36
```

Closures:
```
fun makeCounter() {
  var i = 0;
  fun count() {
    i = i + 1;
    print i;
  }

  return count;
}

var counter = makeCounter();
counter(); // prints 1
counter(); // prints 2
```

Native Functions:
```
> var earlier = clock(); // clock() gets time since unix epoch in seconds
> var later = clock();
> print later;
1616110221
> print later - earlier;
6
```

Static scoping:
```
var a = "global";
{
  fun showA() {
    print a;
  }

  showA(); // prints "global"
  var a = "block";
  showA(); // prints "global"
}
```

Classes:
```
class Foo {
    init(x, y) {
        this.x = this.bar(x);
        this.y = y + 2;
    }

    bar(z) {
        return z * 2;
    }
}

var f = Foo(5,6);
print f.x; // prints 10
print f.y; // prints 8
```

### To-do
Inheritance

## Limitations
Instances are reference counted. So its easy for 2 instances to reference each other and form a cycle and they will not get cleaned up until the program finishes execution.