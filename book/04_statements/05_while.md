# 4.5 The while Statement

The while statement is a loop statement with an exit test at the beginning of the loop. Here is the general form of the while statement:

while (test)
  statement

The while statement first evaluates test. If test evaluates to true, statement is executed, and then test is evaluated again. statement continues to execute repeatedly as long as test is true after each execution of statement.

This example prints the integers from zero through nine:

int counter = 0;
while (counter < 10)
  printf ("%d ", counter++);

A break statement can also cause a while loop to exit. 
