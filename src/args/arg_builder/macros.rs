macro_rules! impl_arg {
    ($name:ident) => {
        pub struct $name<'a, 'b>(pub Arg<'a, 'b>) where 'a: 'b;

        impl<'a, 'b> Deref for $name<'a, 'b> {
            type Target = Arg<'a, 'b>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'a, 'b> ::std::fmt::Display for $name<'a, 'b> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}
