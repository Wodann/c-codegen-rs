# 3.7 Logical Operators

Logical operators test the truth value of a pair of operands. Any nonzero expression is considered true in C, while an expression that evaluates to zero is considered false.

The logical conjunction operator && tests if two expressions are both true. If the first expression is false, then the second expression is not evaluated.

if ((x == 5) && (y == 10))
  printf ("x is 5 and y is 10");

The logical disjunction operator || tests if at least one of two expressions it true. If the first expression is true, then the second expression is not evaluated.

if ((x == 5) || (y == 10))
   printf ("x is 5 or y is 10");

You can prepend a logical expression with a negation operator ! to flip the truth value:

if (!(x == 5))
  printf ("x is not 5");

Since the second operand in a logical expression pair is not necessarily evaluated, you can write code with perhaps unintuitive results:

if (foo && x++)
  bar();

If foo is ever zero, then not only would bar not be called, but x would not be incremented. If you intend to increment x regardless of the value of foo, you should do so outside of the conjunction expression. 
