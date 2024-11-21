# 4.10 The goto Statement

You can use the goto statement to unconditionally jump to a different place in the program. Here is the general form of a goto statement:

goto label;

You have to specify a label to jump to; when the goto statement is executed, program control jumps to that label. See Labels. Here is an example:

goto end_of_program;
…
end_of_program:

The label can be anywhere in the same function as the goto statement that jumps to it, but a goto statement cannot jump to a label in a different function.

You can use goto statements to simulate loop statements, but we do not recommend it—it makes the program harder to read, and GCC cannot optimize it as well. You should use for, while, and do statements instead of goto statements, when possible.

As an extension, GCC allows a goto statement to jump to an address specified by a void* variable. To make this work, you also need to take the address of a label by using the unary operator && (not &). Here is a contrived example:

enum Play { ROCK=0, PAPER=1, SCISSORS=2 };
enum Result { WIN, LOSE, DRAW };

static enum Result turn (void) 
{
  const void * const jumptable[] = {&&rock, &&paper, &&scissors};
  enum Play opp;                /* opponent’s play */
  goto *jumptable[select_option (&opp)];
 rock:
  return opp == ROCK ? DRAW : (opp == PAPER ? LOSE : WIN);
 paper:
  return opp == ROCK ? WIN  : (opp == PAPER ? DRAW : LOSE);
 scissors:
  return opp == ROCK ? LOSE : (opp == PAPER ? WIN  : DRAW);
}
