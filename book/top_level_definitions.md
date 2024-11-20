# Top-Level Definitions in C Files

At the top level of a C source file (outside of any function), the following definitions are allowed according to the C standard:

## 1. Function Definitions
```c
int main(void) {
    // function body
}

static void helper_function(int x) {
    // function body
}
```

## 2. Global Variable Declarations and Definitions
```c
int global_var;                  // declaration
int initialized_var = 42;        // definition with initialization
static int file_scoped_var = 0;  // static variable
extern int external_var;         // external variable declaration
```

## 3. Type Definitions
```c
// Structure definitions
struct point {
    int x;
    int y;
};

// Union definitions
union number {
    int i;
    float f;
};

// Enumeration definitions
enum color {
    RED,
    GREEN,
    BLUE
};

// Typedef declarations
typedef struct point Point;
typedef unsigned char byte;
```

## 4. Function Declarations (Prototypes)
```c
void function_prototype(int arg1, char arg2);
extern int external_function(void);
static int internal_function(float x);
```

## Key Points

1. **Storage Classes**:
   - `static` - Limits visibility to current translation unit
   - `extern` - Declares variables/functions defined elsewhere
   - Default external linkage for functions and global variables

2. **Initialization**:
   - Global variables can only be initialized with constant expressions
   - Uninitialized global variables are automatically zero-initialized
   - Static variables are initialized once at program startup

3. **Scope Rules**:
   - Definitions are visible from point of declaration to end of file
   - Static definitions are only visible within the current translation unit
   - External definitions can be accessed from other translation units via declarations

4. **Restrictions**:
   - Cannot have code statements at file scope
   - Initializers must be constant expressions
   - Function definitions cannot be nested
   - Each identifier can only have one definition in a program

5. **Order Considerations**:
   - Types must be defined before use
   - Forward declarations allow referring to types/functions before definition
   - Multiple declarations allowed, but only one definition per translation unit
