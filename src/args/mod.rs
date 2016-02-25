pub use self::arg::Arg;
pub use self::arg_matches::ArgMatches;
pub use self::arg_matcher::ArgMatcher;
pub use self::subcommand::SubCommand;
pub use self::arg_builder::{Flag, Opt, Positional};
pub use self::matched_arg::MatchedArg;
pub use self::group::ArgGroup;
pub use self::settings::ArgSettings;

mod arg;
mod arg_matches;
mod arg_matcher;
mod subcommand;
mod arg_builder;
mod matched_arg;
mod group;
#[allow(dead_code)]
pub mod settings;

use std::rc::Rc;
use std::fmt::Display;

#[doc(hidden)]
pub trait Any<'n, 'e>: Display {
    fn name(&self) -> &'n str;
    fn is_set(&self, ArgSettings) -> bool;
    fn set(&mut self, ArgSettings);
    fn overrides(&self) -> Option<&[&'e str]>;
    fn requires(&self) -> Option<&[&'e str]>;
    fn blacklist(&self) -> Option<&[&'e str]>;
}

#[doc(hidden)]
pub trait HasValues<'n, 'e>: Display {
    fn max_vals(&self) -> Option<u64>;
    fn min_vals(&self) -> Option<u64>;
    fn num_vals(&self) -> Option<u64>;
    fn possible_vals(&self) -> Option<&[&'e str]>;
    fn validator(&self) -> Option<&Rc<Fn(String) -> Result<(), String>>>;
    fn val_delim(&self) -> Option<char>;
}

#[doc(hidden)]
pub trait Switched<'n, 'e>: Display {
    fn short(&self) -> Option<char>;
    fn long(&self) -> Option<&'e str>;
}
