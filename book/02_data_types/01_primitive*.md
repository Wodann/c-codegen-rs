# 2.1 Primitive Data Types
• Integer Types:	  	
• Real Number Types:	  	

## 2.1.1 Integer Types

The integer data types range in size from at least 8 bits to at least 32 bits. The C99 standard extends this range to include integer sizes of at least 64 bits. You should use integer types for storing whole number values (and the char data type for storing characters). The sizes and ranges listed for these types are minimums; depending on your computer platform, these sizes and ranges may be larger.

While these ranges provide a natural ordering, the standard does not require that any two types have a different range. For example, it is common for int and long to have the same range. The standard even allows signed char and long to have the same range, though such platforms are very unusual.

    signed char
    The 8-bit signed char data type can hold integer values in the range of -128 to 127.
    unsigned char
    The 8-bit unsigned char data type can hold integer values in the range of 0 to 255.
    char
    Depending on your system, the char data type is defined as having the same range as either the signed char or the unsigned char data type (they are three distinct types, however). By convention, you should use the char data type specifically for storing ASCII characters (such as `m'), including escape sequences (such as `\n').
    short int
    The 16-bit short int data type can hold integer values in the range of -32,768 to 32,767. You may also refer to this data type as short, signed short int, or signed short.
    unsigned short int
    The 16-bit unsigned short int data type can hold integer values in the range of 0 to 65,535. You may also refer to this data type as unsigned short.
    int
    The 32-bit int data type can hold integer values in the range of -2,147,483,648 to 2,147,483,647. You may also refer to this data type as signed int or signed.
    unsigned int
    The 32-bit unsigned int data type can hold integer values in the range of 0 to 4,294,967,295. You may also refer to this data type simply as unsigned.
    long int
    The 32-bit long int data type can hold integer values in the range of at least -2,147,483,648 to 2,147,483,647. (Depending on your system, this data type might be 64-bit, in which case its range is identical to that of the long long int data type.) You may also refer to this data type as long, signed long int, or signed long.
    unsigned long int
    The 32-bit unsigned long int data type can hold integer values in the range of at least 0 to 4,294,967,295. (Depending on your system, this data type might be 64-bit, in which case its range is identical to that of the unsigned long long int data type.) You may also refer to this data type as unsigned long.
    long long int
    The 64-bit long long int data type can hold integer values in the range of -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807. You may also refer to this data type as long long, signed long long int or signed long long. This type is not part of C89, but is both part of C99 and a GNU C extension.
    unsigned long long int
    The 64-bit unsigned long long int data type can hold integer values in the range of at least 0 to 18,446,744,073,709,551,615. You may also refer to this data type as unsigned long long. This type is not part of C89, but is both part of C99 and a GNU C extension. 

Here are some examples of declaring and defining integer variables:

int foo;
unsigned int bar = 42;
char quux = 'a';

The first line declares an integer named foo but does not define its value; it is left uninitialized, and its value should not be assumed to be anything in particular.

## 2.1.2 Real Number Types

There are three data types that represent fractional numbers. While the sizes and ranges of these types are consistent across most computer systems in use today, historically the sizes of these types varied from system to system. As such, the minimum and maximum values are stored in macro definitions in the library header file float.h. In this section, we include the names of the macro definitions in place of their possible values; check your system’s float.h for specific numbers.

    float
    The float data type is the smallest of the three floating point types, if they differ in size at all. Its minimum value is stored in the FLT_MIN, and should be no greater than 1e-37. Its maximum value is stored in FLT_MAX, and should be no less than 1e37.
    double
    The double data type is at least as large as the float type, and it may be larger. Its minimum value is stored in DBL_MIN, and its maximum value is stored in DBL_MAX.
    long double
    The long double data type is at least as large as the float type, and it may be larger. Its minimum value is stored in LDBL_MIN, and its maximum value is stored in LDBL_MAX. 

All floating point data types are signed; trying to use unsigned float, for example, will cause a compile-time error.

Here are some examples of declaring and defining real number variables:

float foo;
double bar = 114.3943;

The first line declares a float named foo but does not define its value; it is left uninitialized, and its value should not be assumed to be anything in particular.

The real number types provided in C are of finite precision, and accordingly, not all real numbers can be represented exactly. Most computer systems that GCC compiles for use a binary representation for real numbers, which is unable to precisely represent numbers such as, for example, 4.2. For this reason, we recommend that you consider not comparing real numbers for exact equality with the == operator, but rather check that real numbers are within an acceptable tolerance.

There are other more subtle implications of these imprecise representations; for more details, see David Goldberg’s paper What Every Computer Scientist Should Know About Floating-Point Arithmetic and section 4.2.2 of Donald Knuth’s The Art of Computer Programming. 
