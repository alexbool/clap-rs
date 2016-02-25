use std::fmt::{Display, Formatter, Result};
use std::result::Result as StdResult;
use std::rc::Rc;
use std::io;
use std::ops::Deref;

use Arg;
use args::{Any, HasValues};
use args::settings::ArgSettings;

#[allow(missing_debug_implementations)]
#[doc(hidden)]
pub struct Positional<'n, 'e>  where 'n: 'e {
    #[doc(hidden)]
    pub a: Arg<'n, 'e>,
}

impl<'n, 'e> Positional<'n, 'e> {
    pub fn write_help<W: io::Write>(&self, w: &mut W, tab: &str, longest: usize, skip_pv: bool, nlh: bool) -> io::Result<()> {
        write_arg_help!(@pos self, w, tab, longest, skip_pv, nlh);
        write!(w, "\n")
    }
}

impl<'n, 'e, 'z> From<&'z Arg<'n, 'e>> for Positional<'n, 'e> {
    fn from(a: &'z Arg<'n, 'e>) -> Self {
        Positional {
            a: Arg {
                name: a.name,
                help: a.help,
                index: a.index,
                blacklist: a.blacklist.clone(),
                possible_vals: a.possible_vals.clone(),
                requires: a.requires.clone(),
                group: a.group,
                val_names: a.val_names.clone(),
                num_vals: a.num_vals,
                max_vals: a.max_vals,
                min_vals: a.min_vals,
                validator: a.validator.clone(),
                overrides: a.overrides.clone(),
                settings: a.settings,
                val_delim: a.val_delim,
                default_val: a.default_val,
                ..Default::default()
            }
        }
    }
}


impl<'n, 'e> Display for Positional<'n, 'e> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.settings.is_set(ArgSettings::Required) {
            if let Some(ref names) = self.val_names {
                try!(write!(f, "{}", names.values().map(|n| format!("<{}>", n)).collect::<Vec<_>>().join(" ")));
            } else {
                try!(write!(f, "<{}>", self.name));
            }
        } else {
            if let Some(ref names) = self.val_names {
                try!(write!(f, "{}", names.values().map(|n| format!("[{}]", n)).collect::<Vec<_>>().join(" ")));
            } else {
                try!(write!(f, "[{}]", self.name));
            }
        }
        if self.settings.is_set(ArgSettings::Multiple) && self.val_names.is_none() {
            try!(write!(f, "..."));
        }

        Ok(())
    }
}

impl<'n, 'e> Deref for Positional<'n, 'e> {
    type Target = Arg<'n, 'e>;
    fn deref(&self) -> &Self::Target {
        &self.a
    }
}

impl<'n, 'e> Any<'n, 'e> for Positional<'n, 'e> {
    fn name(&self) -> &'n str { self.name }
    fn is_set(&self, s: ArgSettings) -> bool { self.settings.is_set(s) }
    fn set(&mut self, s: ArgSettings) { self.a.settings.set(s) }
    fn overrides(&self) -> Option<&[&'e str]> { self.overrides.as_ref().map(|o| &o[..]) }
    fn requires(&self) -> Option<&[&'e str]> { self.requires.as_ref().map(|o| &o[..]) }
    fn blacklist(&self) -> Option<&[&'e str]> { self.blacklist.as_ref().map(|o| &o[..]) }
}

impl<'n, 'e> HasValues<'n, 'e> for Positional<'n, 'e> {
    fn max_vals(&self) -> Option<u64> { self.max_vals }
    fn num_vals(&self) -> Option<u64> { self.num_vals }
    fn possible_vals(&self) -> Option<&[&'e str]> { self.possible_vals.as_ref().map(|o| &o[..]) }
    fn validator(&self) -> Option<&Rc<Fn(String) -> StdResult<(), String>>> {
        self.validator.as_ref()
    }
    fn min_vals(&self) -> Option<u64> { self.min_vals }
    fn short(&self) -> Option<char> { None }
    fn long(&self) -> Option<&'e str> { None }
    fn val_delim(&self) -> Option<char> { self.val_delim }
}

#[cfg(test)]
mod test {
    use super::PosBuilder;
    use args::settings::ArgSettings;
    use vec_map::VecMap;

    #[test]
    fn display_mult() {
        let mut p = PosBuilder::new("pos", 1);
        p.settings.set(ArgSettings::Multiple);

        assert_eq!(&*format!("{}", p), "[pos]...");
    }

    #[test]
    fn display_required() {
        let mut p2 = PosBuilder::new("pos", 1);
        p2.settings.set(ArgSettings::Required);

        assert_eq!(&*format!("{}", p2), "<pos>");
    }

    #[test]
    fn display_val_names() {
        let mut p2 = PosBuilder::new("pos", 1);
        let mut vm = VecMap::new();
        vm.insert(0, "file1");
        vm.insert(1, "file2");
        p2.val_names = Some(vm);

        assert_eq!(&*format!("{}", p2), "[file1] [file2]");
    }

    #[test]
    fn display_val_names_req() {
        let mut p2 = PosBuilder::new("pos", 1);
        p2.settings.set(ArgSettings::Required);
        let mut vm = VecMap::new();
        vm.insert(0, "file1");
        vm.insert(1, "file2");
        p2.val_names = Some(vm);

        assert_eq!(&*format!("{}", p2), "<file1> <file2>");
    }
}
