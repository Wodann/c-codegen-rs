// Propose a Rust implementation representing an in-memory description of C global variables based on this specification:
//
// # Global Variables in C

// Global variables are variables declared at file scope (outside any function). Understanding the distinction between declaration and definition, as well as initialization rules, is crucial for correct code generation.

// ## Declaration vs Definition

// ### Declaration
// A declaration introduces an identifier and its type without allocating storage:
// ```c
// extern int counter;        // declares counter without defining it
// extern float pi;          // declares pi without defining it
// ```

// ### Definition
// A definition declares an identifier and also allocates storage:
// ```c
// int counter = 0;          // defines and initializes counter
// float pi = 3.14159f;      // defines and initializes pi
// int uninitialized_var;    // defines with implicit initialization to 0
// ```

// ## Storage Classes

// ### External Linkage (default)
// Variables with external linkage are accessible from other translation units:
// ```c
// int global_var = 42;      // external linkage by default
// extern int global_var;    // declaration for use in other files
// ```

// ### Static (Internal Linkage)
// Static variables are only accessible within their translation unit:
// ```c
// static int file_var = 10; // internal linkage
// ```

use core::fmt;
use std::fmt::Write;

use crate::{CType, CValue, StorageClass};

pub struct Declaration {
    pub name: String,
    pub ty: CType,
    pub initializer: Option<CValue>,
    pub storage_class: Option<StorageClass>,
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(storage_class) = &self.storage_class {
            write!(f, "{} ", storage_class)?;
        }

        write!(f, "{} {}", self.ty, self.name)?;
        if let Some(initializer) = &self.initializer {
            write!(f, " = {}", initializer)?;
        }
        f.write_char(';')?;

        Ok(())
    }
}
