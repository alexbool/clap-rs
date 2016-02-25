use std::fmt::{Display, Formatter, Result};
use std::convert::From;
use std::io;
use std::ops::Deref;

use Arg;
use args::{Any, Switched};
use args::settings::ArgSettings;

#[doc(hidden)]
pub struct Flag<'n, 'e> where 'n: 'e {
    pub a: Arg<'n, 'e>,
}

impl<'n, 'e> Flag<'n, 'e> {
    pub fn write_help<W: io::Write>(&self, w: &mut W, tab: &str, longest: usize, nlh: bool) -> io::Result<()> {
        write_arg_help!(@flag self, w, tab, longest, nlh);
        write!(w, "\n")
    }
}

impl<'a, 'b, 'z> From<&'z Arg<'a, 'b>> for Flag<'a, 'b> {
    fn from(a: &'z Arg<'a, 'b>) -> Self {
        Flag {
            a: Arg {
                name: a.name,
                short: a.short,
                long: a.long,
                help: a.help,
                requires: a.requires.clone(),
                group: a.group,
                overrides: a.overrides.clone(),
                settings: a.settings,
                blacklist: a.blacklist.clone(),
                ..Default::default()
            },
        }
    }
}

impl<'n, 'e> Deref for Flag<'n, 'e> {
    type Target = Arg<'n, 'e>;
    fn deref(&self) -> &Self::Target {
        &self.a
    }
}

impl<'n, 'e> Display for Flag<'n, 'e> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(l) = self.long {
            write!(f, "--{}", l)
        } else {
            write!(f, "-{}", self.short.unwrap())
        }
    }
}

impl<'n, 'e> Any<'n, 'e> for Flag<'n, 'e> {
    fn name(&self) -> &'n str { self.name }
    fn is_set(&self, s: ArgSettings) -> bool { self.settings.is_set(s) }
    fn set(&mut self, s: ArgSettings) { self.a.settings.set(s) }
    fn overrides(&self) -> Option<&[&'e str]> { self.overrides.as_ref().map(|o| &o[..]) }
    fn requires(&self) -> Option<&[&'e str]> { self.requires.as_ref().map(|o| &o[..]) }
    fn blacklist(&self) -> Option<&[&'e str]> { self.blacklist.as_ref().map(|o| &o[..]) }
}

impl<'n, 'e> Switched<'n, 'e> for Flag<'n, 'e> {
    fn short(&self) -> Option<char> { self.short }
    fn long(&self) -> Option<&'e str> { self.long }
}

#[cfg(test)]
mod test {
    use super::FlagBuilder;
    use args::settings::ArgSettings;

    #[test]
    fn flagbuilder_display() {
        let mut f = FlagBuilder::new("flg");
        f.settings.set(ArgSettings::Multiple);
        f.long = Some("flag");

        assert_eq!(&*format!("{}", f), "--flag");

        let mut f2 = FlagBuilder::new("flg");
        f2.short = Some('f');

        assert_eq!(&*format!("{}", f2), "-f");
    }
}
