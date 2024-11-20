# 3.4 Arithmetic Operators

C provides operators for standard arithmetic operations: addition, subtraction, multiplication, and division, along with modular division and negation. Usage of these operators is straightforward; here are some examples:

TODO:
/* Addition. */
x = 5 + 3;
y = 10.23 + 37.332;
quux_pointer = foo_pointer + bar_pointer;

/* Subtraction. */
x = 5 - 3;
y = 57.223 - 10.903;
quux_pointer = foo_pointer - bar_pointer;

You can add and subtract memory pointers, but you cannot multiply or divide them.

TODO:
/* Multiplication. */
x = 5 * 3;
y = 47.4 * 1.001;

/* Division. */
x = 5 / 3;
y = 940.0 / 20.2;

Integer division of positive values truncates towards zero, so 5/3 is 1. However, if either operand is negative, the direction of rounding is implementation-defined. Signed Integer Division for information about overflow in signed integer division.

You use the modulus operator % to obtain the remainder produced by dividing its two operands. You put the operands on either side of the operator, and it does matter which operand goes on which side: 3 % 5 and 5 % 3 do not have the same result. The operands must be expressions of a primitive data type.

TODO:
/* Modular division. */
x = 5 % 3;
y = 74 % 47;

Modular division returns the remainder produced after performing integer division on the two operands. The operands must be of a primitive integer type.

TODO:
/* Negation. */
int x = -5;
float y = -3.14159;

If the operand you use with the negative operator is of an unsigned data type, then the result cannot negative, but rather is the maximum value of the unsigned data type, minus the value of the operand.

Many systems use twos-complement arithmetic, and on such systems the most negative value a signed type can hold is further away from zero than the most positive value. For example, on one platform, this program:

TODO:
#include <limits.h>
#include <stdio.h>

int main (int argc, char *argv[]) 
{
  int x;
  x = INT_MAX;
  printf("INT_MAX  = %d\n", x);
  x = INT_MIN;
  printf("INT_MIN  = %d\n", x);
  x = -x;
  printf("-INT_MIN = %d\n", x);
  return 0;
}

Produces this output:

```bash
INT_MAX  = 2147483647
INT_MIN  = -2147483648
-INT_MIN = -2147483648
```

Trivially, you can also apply a positive operator to a numeric expression:

TODO:
int x = +42;

Numeric values are assumed to be positive unless explicitly made negative, so this operator has no effect on program operation. 
