# 3.8 Bit Shifting

You use the left-shift operator << to shift its first operand’s bits to the left. The second operand denotes the number of bit places to shift. Bits shifted off the left side of the value are discarded; new bits added on the right side will all be 0.

x = 47;    /* 47 is 00101111 in binary. */
x << 1;    /* 00101111 << 1 is 01011110. */

Similarly, you use the right-shift operator >> to shift its first operand’s bits to the right. Bits shifted off the right side are discarded; new bits added on the left side are usually 0, but if the first operand is a signed negative value, then the added bits will be either 0 or whatever value was previously in the leftmost bit position.

x = 47;   /* 47 is 00101111 in binary. */
x >> 1;   /* 00101111 >> 1 is 00010111. */

For both << and >>, if the second operand is greater than the bit-width of the first operand, or the second operand is negative, the behavior is undefined.

You can use the shift operators to perform a variety of interesting hacks. For example, given a date with the day of the month numbered as d, the month numbered as m, and the year y, you can store the entire date in a single number x:

int d = 12;
int m = 6;
int y = 1983;
int x = (((y << 4) + m) << 5) + d;

You can then extract the original day, month, and year out of x using a combination of shift operators and modular division:

d = x % 32;
m = (x >> 5) % 16;
y = x >> 9;
