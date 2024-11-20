# 3.9 Bitwise Logical Operators

C provides operators for performing bitwise conjunction, inclusive disjunction, exclusive disjunction, and negation (complement).

Biwise conjunction examines each bit in its two operands, and when two corresponding bits are both 1, the resulting bit is 1. All other resulting bits are 0. Here is an example of how this works, using binary numbers:

11001001 & 10011011 = 10001001

Bitwise inclusive disjunction examines each bit in its two operands, and when two corresponding bits are both 0, the resulting bit is 0. All other resulting bits are 1.

11001001 | 10011011 = 11011011

Bitwise exclusive disjunction examines each bit in its two operands, and when two corresponding bits are different, the resulting bit is 1. All other resulting bits are 0.

11001001 ^ 10011011 = 01010010

Bitwise negation reverses each bit in its operand:

~11001001 = 00110110

In C, you can only use these operators with operands of an integer (or character) type, and for maximum portability, you should only use the bitwise negation operator with unsigned integer types. Here are some examples of using these operators in C code:

unsigned int foo = 42;
unsigned int bar = 57;
unsigned int quux;

quux = foo & bar;
quux = foo | bar;
quux = foo ^ bar;
quux = ~foo;
