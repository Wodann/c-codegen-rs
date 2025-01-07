# 3.17 Conditional Expressions

You use the conditional operator to cause the entire conditional expression to evaluate to either its second or its third operand, based on the truth value of its first operand. Here’s an example:

a ? b : c

If expression a is true, then expression b is evaluated and the result is the value of b. Otherwise, expression c is evaluated and the result is c.

Expressions b and c must be compatible. That is, they must both be

    arithmetic types
    compatible struct or union types
    pointers to compatible types (one of which might be the NULL pointer) 

Alternatively, one operand is a pointer and the other is a void* pointer.

Here is an example

a = (x == 5) ? y : z;

Here, if x equals 5, then a will receive the value of y. Otherwise, a will receive the value of z. This can be considered a shorthand method for writing a simple if…else statement. The following example will accomplish the same task as the previous one:

if (x == 5)
    a = y;
else
    a = z;

If the first operand of the conditional operator is true, then the third operand is never evaluated. Similarly, if the first operand is false, then the second operand is never evaluated. The first operand is always evaluated. 
