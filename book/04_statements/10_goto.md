# 4.10 The goto Statement

You can use the goto statement to unconditionally jump to a different place in the program. Here is the general form of a goto statement:

goto label;

You have to specify a label to jump to; when the goto statement is executed, program control jumps to that label. See Labels. Here is an example:

goto end_of_program;
…
end_of_program:

The label can be anywhere in the same function as the goto statement that jumps to it, but a goto statement cannot jump to a label in a different function.

You can use goto statements to simulate loop statements, but we do not recommend it—it makes the program harder to read, and GCC cannot optimize it as well. You should use for, while, and do statements instead of goto statements, when possible.
