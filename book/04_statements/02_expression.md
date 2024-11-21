# 4.2 Expression Statements

You can turn any expression into a statement by adding a semicolon to the end of the expression. Here are some examples:

5;
2 + 2;
10 >= 9;

In each of those, all that happens is that each expression is evaluated. However, they are useless because they do not store a value anywhere, nor do they actually do anything, other than the evaluation itself. The compiler is free to ignore such statements.

Expression statements are only useful when they have some kind of side effect, such as storing a value, calling a function, or (this is esoteric) causing a fault in the program. Here are some more useful examples:

x++;
y = x + 25;
puts ("Hello, user!");
*cucumber;

The last of those statements, *cucumber;, could potentially cause a fault in the program if the value of cucumber is both not a valid pointer and has been declared as volatile. 
