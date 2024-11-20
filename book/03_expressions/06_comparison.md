# 3.6 Comparison Operators

You use the comparison operators to determine how two operands relate to each other: are they equal to each other, is one larger than the other, is one smaller than the other, and so on. When you use any of the comparison operators, the result is either 1 or 0, meaning true or false, respectively.

(In the following code examples, the variables x and y stand for any two expressions of arithmetic types, or pointers.)

The equal-to operator == tests its two operands for equality. The result is 1 if the operands are equal, and 0 if the operands are not equal.

if (x == y)
  puts ("x is equal to y");
else
  puts ("x is not equal to y");

The not-equal-to operator != tests its two operands for inequality. The result is 1 if the operands are not equal, and 0 if the operands are equal.

if (x != y)
  puts ("x is not equal to y");
else
  puts ("x is equal to y");

Comparing floating-point values for exact equality or inequality can produce unexpected results. Real Number Types for more information.

You can compare function pointers for equality or inequality; the comparison tests if two function pointers point to the same function or not.

Beyond equality and inequality, there are operators you can use to test if one value is less than, greater than, less-than-or-equal-to, or greater-than-or-equal-to another value. Here are some code samples that exemplify usage of these operators:

if (x < y)
  puts ("x is less than y");

if (x <= y)
  puts ("x is less than or equal to y");

if (x > y)
  puts ("x is greater than y");

if (x >= y)
  puts ("x is greater than or equal to y");
