# 4.11 The break Statement

You can use the break statement to terminate a while, do, for, or switch statement. Here is an example:

int x;
for (x = 1; x <= 10; x++)
  {
    if (x == 8)
      break;
    else
      printf ("%d ", x);
  }

That example prints numbers from 1 to 7. When x is incremented to 8, x == 8 is true, so the break statement is executed, terminating the for loop prematurely.

If you put a break statement inside of a loop or switch statement which itself is inside of a loop or switch statement, the break only terminates the innermost loop or switch statement. 
