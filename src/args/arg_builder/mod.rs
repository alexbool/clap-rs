pub use self::flag::Flag;
pub use self::option::Opt;
pub use self::positional::Pos;
pub use self::help_writer::HelpWriter;

#[macro_use]
mod macros;
#[allow(dead_code)]
mod flag;
#[allow(dead_code)]
mod positional;
#[allow(dead_code)]
mod option;
mod help_writer;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ArgKind {
    Opt,
    Pos,
    Flag,
}
