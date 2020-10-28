//! This library provides safe read-only access to unibilium C library. The latter gives read/write
//! access to terminal capabilities from termcap database.
//!
//! This library provides means to create a `Term` struct representing a terminal. Using this
//! struct you can read boolean, numeric and string capabilities of the terminal. You can also read
//! the extended versions of each of the above capabilities.
//!
//! Currently you cannot modify capabilities or add new extended ones.
//!
//! # Examples
//!
//! Create a Term struct using TERM environment variable.
//!
//! ```
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! use unibilium::Term;
//!
//! let term = Term::from_env()?;
//! #
//! #     Ok(())
//! # }
//! ```
//!
//! Create a Term struct for specific terminal.
//!
//! ```
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! use unibilium::Term;
//!
//! let term = Term::from_term_name("vt100")?;
//! #
//! #     Ok(())
//! # }
//! ```
//!
//! Dump boolean capabilities of a terminal.
//!
//! ```
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! use unibilium::Term;
//!
//! let term = Term::from_term_name("vt100")?;
//! for bool_cap in term.booleans() {
//!     println!("{}", bool_cap);
//! }
//! #
//! #     Ok(())
//! # }
//! ```
//!
//! Dump extended boolean capabilities of a terminal.
//!
//! ```
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! use unibilium::Term;
//!
//! let term = Term::from_term_name("vt100")?;
//! for bool_cap in term.ext_booleans() {
//!     println!("{}", bool_cap);
//! }
//! #
//! #     Ok(())
//! # }
//! ```

pub mod boolean;
pub mod error;
pub mod numeric;
pub mod string;
pub mod term;

pub use boolean::{Boolean, ExtBoolean};
pub use numeric::{ExtNumeric, Numeric};
pub use string::{ExtString, String};
pub use term::Term;
