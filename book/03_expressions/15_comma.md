# 3.15 The Comma Operator

You use the comma operator , to separate two (ostensibly related) expressions. For instance, the first expression might produce a value that is used by the second expression:

x++, y = x * x;

More commonly, the comma operator is used in for statements, like this:

/* Using the comma operator in a for statement. */

for (x = 1, y = 10;  x <=10 && y >=1;  x++, y--)
  {
    …
  }

This lets you conveniently set, monitor, and modify multiple control expressions for the for statement.

A comma is also used to separate function parameters; however, this is not the comma operator in action. In fact, if the comma operator is used as we have discussed here in a function call, then the compiler will interpret that as calling the function with an extra parameter.

If you want to use the comma operator in a function argument, you need to put parentheses around it. That’s because commas in a function argument list have a different meaning: they separate arguments. Thus,

foo (x,  y=47,  x,  z);

is interpreted as a function call with four arguments, but

foo (x,  (y=47,  x),  z);

is a function call with just three arguments. (The second argument is (y=47, x).) 
