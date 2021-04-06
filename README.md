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

Inheritance:
```
class Doughnut {
  cook() {
    print "Fry until golden brown.";
  }
}

class BostonCream < Doughnut {}

BostonCream().cook(); // prints "Fry until golden brown."
```

and calling a superclass's method:
```

class Doughnut {
  cook() {
    print "Fry until golden brown.";
  }
}

class BostonCream < Doughnut {
  cook() {
    super.cook();
    print "Pipe full of custard and coat with chocolate.";
  }
}

//prints "Fry until golden brown." then "Pipe full of custard and coat with chocolate."
BostonCream().cook();
```

## Limitations
Instances are reference counted. So its easy for 2 instances to reference each other and form a cycle and they will not get cleaned up until the program finishes execution.

The tree walking interpreter described in the first half of the Crafting Interpreters book is written in Java and the author uses the Java runtime in order for implementation objects to be garbage collected. This Rust implementation does a lot of copying and `clone()`-ing. I would need to do some rearchitecting in order to avoid these unnecessary copies, but I chose not to do that since this is a toy interpreter and I was following along with the book. The goal of this project was to learn more about interpeters, not to make a fast interpeter.

In the second half of the book, the author implements a byte code interpreter written in C and I believe they create a garbage collector/runtime. The garbage collector would also avoid the issue of instances being reference counted in this interpreter. If I ever read and follow along that part of the book, I will try to implement the byte code interpreter in Rust with a garbage collector and hopefully avoid the many copies that this interpreter makes.
