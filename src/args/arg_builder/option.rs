use std::rc::Rc;
use std::fmt::{Display, Formatter, Result};
use std::result::Result as StdResult;
use std::io;
use std::ops::Deref;

use args::{Any, Switched, HasValues, Arg};
use args::settings::ArgSettings;

#[allow(missing_debug_implementations)]
#[doc(hidden)]
pub struct Opt<'n, 'e> where 'n: 'e {
    #[doc(hidden)]
    pub a: Arg<'n, 'e>,
}

impl<'n, 'e> Opt<'n, 'e> {
    pub fn write_help<W: io::Write>(&self, w: &mut W, tab: &str, longest: usize, skip_pv: bool, nlh: bool) -> io::Result<()> {
        debugln!("fn=write_help");
        write_arg_help!(@opt self, w, tab, longest, skip_pv, nlh);
        write!(w, "\n")
    }
}

impl<'n, 'e, 'z> From<&'z Arg<'n, 'e>> for Opt<'n, 'e> {
    fn from(a: &'z Arg<'n, 'e>) -> Self {
        Opt {
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
                short: a.short,
                long: a.long,
            }
        }
    }
}

impl<'n, 'e> Display for Opt<'n, 'e> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debugln!("fn=fmt");
        // Write the name such --long or -l
        if let Some(l) = self.long {
            try!(write!(f, "--{}", l));
        } else {
            try!(write!(f, "-{}", self.short.unwrap()));
        }

        // Write the values such as <name1> <name2>
        if let Some(ref vec) = self.val_names {
            for (_, n) in vec {
                debugln!("writing val_name: {}", n);
                try!(write!(f, " <{}>", n));
            }
            let num = vec.len();
            if self.settings.is_set(ArgSettings::Multiple) && num == 1 {
                try!(write!(f, "..."));
            }
        } else {
            let num = self.num_vals.unwrap_or(1);
            for _ in 0..num {
                try!(write!(f, " <{}>", self.name));
            }
            if self.settings.is_set(ArgSettings::Multiple) && num == 1 {
                try!(write!(f, "..."));
            }
        }

        Ok(())
    }
}

impl<'n, 'e> Deref for Opt<'n, 'e> {
    type Target = Arg<'n, 'e>;
    fn deref(&self) -> &Self::Target {
        &self.a
    }
}

impl<'n, 'e> Any<'n, 'e> for Opt<'n, 'e> {
    fn name(&self) -> &'n str { self.name }
    fn is_set(&self, s: ArgSettings) -> bool { self.settings.is_set(s) }
    fn set(&mut self, s: ArgSettings) { self.a.settings.set(s) }
    fn overrides(&self) -> Option<&[&'e str]> { self.overrides.as_ref().map(|o| &o[..]) }
    fn requires(&self) -> Option<&[&'e str]> { self.requires.as_ref().map(|o| &o[..]) }
    fn blacklist(&self) -> Option<&[&'e str]> { self.blacklist.as_ref().map(|o| &o[..]) }
}

impl<'n, 'e> HasValues<'n, 'e> for Opt<'n, 'e> {
    fn max_vals(&self) -> Option<u64> { self.max_vals }
    fn num_vals(&self) -> Option<u64> { self.num_vals }
    fn possible_vals(&self) -> Option<&[&'e str]> { self.possible_vals.as_ref().map(|o| &o[..]) }
    fn validator(&self) -> Option<&Rc<Fn(String) -> StdResult<(), String>>> {
        self.validator.as_ref()
    }
    fn min_vals(&self) -> Option<u64> { self.min_vals }
    fn val_delim(&self) -> Option<char> { self.val_delim }
}

impl<'n, 'e> Switched<'n, 'e> for Opt<'n, 'e> {
    fn short(&self) -> Option<char> { self.short }
    fn long(&self) -> Option<&'e str> { self.long }
}

#[cfg(test)]
mod test {
    use super::Opt;
    use vec_map::VecMap;
    use args::settings::ArgSettings;
    use args::Arg;

    #[test]
    fn optbuilder_display1() {
        let mut a = Arg::with_name("opt");
        a.long = Some("option");
        a.settings.set(ArgSettings::Multiple);
        let o = Opt::from(&a);

        assert_eq!(&*format!("{}", o), "--option <opt>...");
    }

    #[test]
    fn optbuilder_display2() {
        let mut v_names = VecMap::new();
        v_names.insert(0, "file");
        v_names.insert(1, "name");

        let mut a2 = Arg::with_name("opt");
        a2.short = Some('o');
        a2.val_names = Some(v_names);
        let o2 = Opt::from(&a2);

        assert_eq!(&*format!("{}", o2), "-o <file> <name>");
    }

    #[test]
    fn optbuilder_display3() {
        let mut v_names = VecMap::new();
        v_names.insert(0, "file");
        v_names.insert(1, "name");

        let mut a2 = Arg::with_name("opt");
        a2.short = Some('o');
        a2.val_names = Some(v_names);
        a2.settings.set(ArgSettings::Multiple);
        let o2 = Opt::from(&a2);

        assert_eq!(&*format!("{}", o2), "-o <file> <name>");
    }
}
