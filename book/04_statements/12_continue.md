# 4.12 The continue Statement

You can use the continue statement in loops to terminate an iteration of the loop and begin the next iteration. Here is an example:

for (x = 0; x < 100; x++)
  {
    if (x % 2 == 0)
      continue;
    else
      sum_of_odd_numbers + = x;
  }

If you put a continue statement inside a loop which itself is inside a loop, then it affects only the innermost loop. 
