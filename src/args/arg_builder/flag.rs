use std::ops::Deref;

use args::Arg;

impl_arg!(Flag);

#[cfg(test)]
mod test {
    use super::Flag;
    use args::settings::ArgSettings;
    use args::Arg;

    #[test]
    fn flagbuilder_display() {
        let mut a = Arg::with_name("flg");
        a.settings.set(ArgSettings::Multiple);
        a.long = Some("flag");
        let f = Flag(a);

        assert_eq!(&*format!("{}", f), "--flag");

        let mut a2 = Arg::with_name("flg");
        a2.short = Some('f');
        let f2 = Flag(a2);

        assert_eq!(&*format!("{}", f2), "-f");
    }
}
