# 3.12 Type Casts

You can use a type cast to explicitly cause an expression to be of a specified data type. A type cast consists of a type specifier enclosed in parentheses, followed by an expression. To ensure proper casting, you should also enclose the expression that follows the type specifier in parentheses. Here is an example:

float x;
int y = 7;
int z = 3;
x = (float) (y / z);

In that example, since y and z are both integers, integer division is performed, and even though x is a floating-point variable, it receives the value 2. Explicitly casting the result of the division to float does no good, because the computed value of y/z is already 2.

To fix this problem, you need to convert one of the operands to a floating-point type before the division takes place:

float x;
int y = 7;
int z = 3;
x = (y / (float)z);

Here, a floating-point value close to 2.333… is assigned to x.

Type casting only works with scalar types (that is, integer, floating-point or pointer types). Therefore, this is not allowed:

struct fooTag { /* members ... */ };
struct fooTag foo;
unsigned char byteArray[8];

foo = (struct fooType) byteArray; /* Fail! */
