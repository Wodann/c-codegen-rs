# 4.1 Labels

You can use labels to identify a section of source code for use with a later goto (see The goto Statement). A label consists of an identifier (such as those used for variable names) followed by a colon. Here is an example:

treet:

You should be aware that label names do not interfere with other identifier names:

int treet = 5;    /* treet the variable. */
treet:            /* treet the label. */

The ISO C standard mandates that a label must be followed by at least one statement, possibly a null statement (see The Null Statement). GCC will compile code that does not meet this requirement, but be aware that if you violate it, your code may have portability issues. 
