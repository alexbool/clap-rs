use std::ops::Deref;

use args::Arg;

impl_arg!(Opt);

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
        let o = Opt(a);

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
        let o2 = Opt(a2);

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
        let o2 = Opt(a2);

        assert_eq!(&*format!("{}", o2), "-o <file> <name>");
    }
}
