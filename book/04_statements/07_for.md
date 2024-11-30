# 4.7 The for Statement

The for statement is a loop statement whose structure allows easy variable initialization, expression testing, and variable modification. It is very convenient for making counter-controlled loops. Here is the general form of the for statement:

for (initialize; test; step)
  statement

The for statement first evaluates the expression initialize. Then it evaluates the expression test. If test is false, then the loop ends and program control resumes after statement. Otherwise, if test is true, then statement is executed. Finally, step is evaluated, and the next iteration of the loop begins with evaluating test again.

Most often, initialize assigns values to one or more variables, which are generally used as counters, test compares those variables to a predefined expression, and step modifies those variables’ values. Here is another example that prints the integers from zero through nine:

int x;
for (x = 0; x < 10; x++)
  printf ("%d ", x);

First, it evaluates initialize, which assigns x the value 0. Then, as long as x is less than 10, the value of x is printed (in the body of the loop). Then x is incremented in the step clause and the test re-evaluated.

All three of the expressions in a for statement are optional, and any combination of the three is valid. Since the first expression is evaluated only once, it is perhaps the most commonly omitted expression. You could also write the above example as:

int x = 1;
for (; x <= 10; x++)
  printf ("%d ", x);

In this example, x receives its value prior to the beginning of the for statement.

If you leave out the test expression, then the for statement is an infinite loop (unless you put a break or goto statement somewhere in statement). This is like using 1 as test; it is never false.

This for statement starts printing numbers at 1 and then continues indefinitely, always printing x incremented by 1:

for (x = 1; ; x++)
  printf ("%d ", x);

If you leave out the step expression, then no progress is made toward completing the loop—at least not as is normally expected with a for statement.

This example prints the number 1 over and over, indefinitely:

for (x = 1; x <= 10;)
  printf ("%d ", x);

Perhaps confusingly, you cannot use the comma operator (see The Comma Operator) for monitoring multiple variables in a for statement, because as usual the comma operator discards the result of its left operand. This loop:

int x, y;
for (x = 1, y = 10; x <= 10, y >= 1; x+=2, y--)
  printf ("%d %d\n", x, y);

Outputs:

1 10
3 9
5 8
7 7
9 6
11 5
13 4
15 3
17 2
19 1

If you need to test two conditions, you will need to use the && operator:

int x, y;
for (x = 1, y = 10; x <= 10 && y >= 1; x+=2, y--)
  printf ("%d %d\n", x, y);

A break statement can also cause a for loop to exit.

Here is an example of a function that computes the summation of squares, given a starting integer to square and an ending integer to square:

int
sum_of_squares (int start, int end)
{
  int i, sum = 0;
  for (i = start; i <= end; i++)
    sum += i * i;
  return sum;
}
