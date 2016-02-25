use std::ops::Deref;

use args::Arg;

impl_arg!(Pos);

#[cfg(test)]
mod test {
    use vec_map::VecMap;

    use super::Pos;
    use args::settings::ArgSettings;
    use args::Arg;

    #[test]
    fn display_mult() {
        let mut a = Arg::with_name("pos");
        a.settings.set(ArgSettings::Multiple);
        let p = Pos(a);

        assert_eq!(&*format!("{}", p), "[pos]...");
    }

    #[test]
    fn display_required() {
        let mut a2 = Arg::with_name("pos");
        a2.settings.set(ArgSettings::Required);
        let p2 = Pos(a2);

        assert_eq!(&*format!("{}", p2), "<pos>");
    }

    #[test]
    fn display_val_names() {
        let mut a2 = Arg::with_name("pos");
        let mut vm = VecMap::new();
        vm.insert(0, "file1");
        vm.insert(1, "file2");
        a2.val_names = Some(vm);
        let p2 = Pos(a2);

        assert_eq!(&*format!("{}", p2), "[file1] [file2]");
    }

    #[test]
    fn display_val_names_req() {
        let mut a2 = Arg::with_name("pos");
        a2.settings.set(ArgSettings::Required);
        let mut vm = VecMap::new();
        vm.insert(0, "file1");
        vm.insert(1, "file2");
        a2.val_names = Some(vm);
        let p2 = Pos(a2);

        assert_eq!(&*format!("{}", p2), "<file1> <file2>");
    }
}
