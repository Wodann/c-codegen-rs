# 4.4 The switch Statement

You can use the switch statement to compare one expression with others, and then execute a series of sub-statements based on the result of the comparisons. Here is the general form of a switch statement:

switch (test)
  {
    case compare-1:
      if-equal-statement-1
    case compare-2:
      if-equal-statement-2
    …
    default:
      default-statement
  }

The switch statement compares test to each of the compare expressions, until it finds one that is equal to test. Then, the statements following the successful case are executed. All of the expressions compared must be of an integer type, and the compare-N expressions must be of a constant integer type (e.g., a literal integer or an expression built of literal integers).

Optionally, you can specify a default case. If test doesn’t match any of the specific cases listed prior to the default case, then the statements for the default case are executed. Traditionally, the default case is put after the specific cases, but that isn’t required.

switch (x)
  {
    case 0:
      puts ("x is 0");
      break;
    case 1:
      puts ("x is 1");
      break;
    default:
      puts ("x is something else");
      break;
  }

Notice the usage of the break statement in each of the cases. This is because, once a matching case is found, not only are its statements executed, but so are the statements for all following cases:

int x = 0;
switch (x)
  {
    case 0:
      puts ("x is 0");
    case 1:
      puts ("x is 1");
    default:
      puts ("x is something else");
  }

The output of that example is:

x is 0
x is 1
x is something else

This is often not desired. Including a break statement at the end of each case redirects program flow to after the switch statement.

It is common to use a switch statement to handle various possible values of errno. In this case a portable program should watch out for the possibility that two macros for errno values in fact have the same value, for example EWOULDBLOCK and EAGAIN. 
