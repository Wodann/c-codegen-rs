# 3.1 Expressions

An expression consists of at least one operand and zero or more operators. Operands are typed objects such as constants, variables, and function calls that return values. Here are some examples:

47
2 + 2
cosine(3.14159) /* We presume this returns a floating point value. */

Parentheses group subexpressions:

( 2 * ( ( 3 + 10 ) - ( 2 * 6 ) ) )

Innermost expressions are evaluated first. In the above example, 3 + 10 and 2 * 6 evaluate to 13 and 12, respectively. Then 12 is subtracted from 13, resulting in 1. Finally, 1 is multiplied by 2, resulting in 2. The outermost parentheses are completely optional.

An operator specifies an operation to be performed on its operand(s). Operators may have one, two, or three operands, depending on the operator. 
