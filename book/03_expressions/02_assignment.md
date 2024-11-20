# 3.2 Assignment Operators

Assignment operators store values in variables. C provides several variations of assignment operators.

The standard assignment operator = simply stores the value of its right operand in the variable specified by its left operand. As with all assignment operators, the left operand (commonly referred to as the “lvalue”) cannot be a literal or constant value.

TODO:
```c
int x = 10;
float y = 45.12 + 2.0;
int z = (2 * (3 + function () ));

struct foo {
  int bar;
  int baz;
} quux = {3, 4};
```

Note that, unlike the other assignment operators described below, you can use the plain assignment operator to store values of a structure type.

Compound assignment operators perform an operation involving both the left and right operands, and then assign the resulting expression to the left operand. Here is a list of the compound assignment operators, and a brief description of what they do:

    +=

    Adds the two operands together, and then assign the result of the addition to the left operand.
    -=

    Subtract the right operand from the left operand, and then assign the result of the subtraction to the left operand.
    *=

    Multiply the two operands together, and then assign the result of the multiplication to the left operand.
    /=

    Divide the left operand by the right operand, and assign the result of the division to the left operand.
    %=

    Perform modular division on the two operands, and assign the result of the division to the left operand.
    <<=

    Perform a left shift operation on the left operand, shifting by the number of bits specified by the right operand, and assign the result of the shift to the left operand.
    >>=

    Perform a right shift operation on the left operand, shifting by the number of bits specified by the right operand, and assign the result of the shift to the left operand.
    &=

    Perform a bitwise conjunction operation on the two operands, and assign the result of the operation to the left operand.
    ^=

    Performs a bitwise exclusive disjunction operation on the two operands, and assign the result of the operation to the left operand.
    |=

    Performs a bitwise inclusive disjunction operation on the two operands, and assign the result of the operation to the left operand.

Here is an example of using one of the compound assignment operators:

```c
x += y;
```

Since there are no side effects wrought by evaluating the variable x as an lvalue, the above code produces the same result as:

```c
x = x + y;
```
