use std::io;

use args::{Arg, ArgKind};
use args::settings::ArgSettings;

pub struct HelpWriter {
    pub tab: &'static str,
    pub l: usize,
    pub nlh: bool,
    pub skip_pv: bool,
}

impl HelpWriter {
    pub fn write<'a, 'b, W: io::Write>(&self, a: &Arg<'a, 'b>, w: &mut W) -> io::Result<()> {
        self.write_help(a, w)
    }

    pub fn writeln<'a, 'b, W: io::Write>(&self, a: &Arg<'a, 'b>, w: &mut W) -> io::Result<()> {
        try!(self.write_help(a, w));
        write!(w, "\n")
    }

    fn write_help<'a, 'b, W>(&self, a: &Arg<'a, 'b>, w: &mut W) -> io::Result<()>
        where W: io::Write
    {
        match a.kind {
            ArgKind::Flag => {
                try!(self.short(a, w));
                try!(self.long(a, w));
                if let Some(_) = a.help {
                    try!(self.help(a, w));
                }
            },
            ArgKind::Opt => {
                try!(self.short(a, w));
                try!(self.long(a, w));
                try!(self.val(a, w));
                if !(self.nlh || a.is_set(ArgSettings::NextLineHelp)) {
                    write_spaces!(if a.long.is_some() { self.l + 4 } else { self.l + 8 } - (a.to_string().len()), w);
                }
                if let Some(_) = a.help {
                    try!(self.help(a, w));
                    try!(self.spec_vals(a, w));
                }
            },
            ArgKind::Pos => {
                try!(write!(w, "{}", self.tab));
                try!(self.val(a, w));
                if !(self.nlh || a.is_set(ArgSettings::NextLineHelp)) {
                    write_spaces!(self.l + 4 - (a.to_string().len()), w);
                }
                if a.help.is_some() {
                    try!(self.help(a, w));
                    try!(self.spec_vals(a, w));
                }
            }
        }
        Ok(())
    }

    fn short<'a, 'b, W>(&self, a: &Arg<'a, 'b>, w: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(write!(w, "{}", self.tab));
        if let Some(s) = a.short {
            write!(w, "-{}", s)
        } else {
            write!(w, "{}", self.tab)
        }
    }

    fn long<'a, 'b, W>(&self, a: &Arg<'a, 'b>, w: &mut W) -> io::Result<()>
        where W: io::Write
    {
        match a.kind {
            ArgKind::Flag => {
                if let Some(l) = a.long {
                    try!(write!(w, "{}--{}", if a.short.is_some() { ", " } else { "" }, l));
                }
                try!(write!(w, " "))
            },
            ArgKind::Opt => {
                if let Some(l) = a.long {
                    try!(write!(w, "{}--{}", if a.short.is_some() { ", " } else { "" }, self.l));
                    if !self.nlh || !a.is_set(ArgSettings::NextLineHelp) {
                        write_spaces!((self.l + 4) - (l.len() + 2), w);
                    }
                } else {
                    if !self.nlh || !a.is_set(ArgSettings::NextLineHelp) {
                        // 6 is tab (4) + -- (2)
                        write_spaces!((self.l + 6), w);
                    }
                }
            },
            _ => unreachable!()
        }
        Ok(())
    }

    fn val<'a, 'b, W>(&self, a: &Arg<'a, 'b>, w: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if let Some(ref vec) = a.val_names {
            let mut it = vec.iter().peekable();
            while let Some((_, val)) = it.next() {
                try!(write!(w, "<{}>", val));
                if it.peek().is_some() { try!(write!(w, " ")); }
            }
            let num = vec.len();
            if a.is_set(ArgSettings::Multiple) && num == 1 {
                try!(write!(w, "..."));
            }
        } else if let Some(num) = a.num_vals {
            for _ in 0..num {
                try!(write!(w, "<{}>", a.name));
            }
        } else {
            try!(write!(w, "<{}>{}", a.name, if a.is_set(ArgSettings::Multiple) { "..." } else { "" }));
        }
        Ok(())
    }

    fn spec_vals<'a, 'b, W>(&self, a: &Arg<'a, 'b>, w: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if let Some(ref pv) = a.default_val {
            try!(write!(w, " [default: {}]", pv));
        }
        if !self.skip_pv {
            if let Some(ref pv) = a.possible_vals {
                try!(write!(w, " [values: {}]", pv.join(", ")));
            }
        }
        Ok(())
    }

    fn help<'a, 'b, W>(&self, a: &Arg<'a, 'b>, w: &mut W) -> io::Result<()>
        where W: io::Write
    {
        use term;
        let mut help = String::new();
        let h = a.help.unwrap_or("");
        let spcs = if self.nlh || a.is_set(ArgSettings::NextLineHelp) {
            8 // "tab" + "tab"
        } else {
            self.l + 12
        };
        // get terminal width
        let term_w = term::dimensions().map(|(w, _)| w);

        // determine if our help fits or needs to wrap
        let too_long = term_w.is_some() && spcs + h.len() >= term_w.unwrap_or(0);

        // Is help on next line, if so newline + 2x tab
        if self.nlh || a.is_set(ArgSettings::NextLineHelp) {
            try!(write!(w, "\n{}{}", self.tab, self.tab));
        }

        if too_long {
            if let Some(width) = term_w {
                help.push_str(h);
                debugln!("width: {}", width);
                // Determine how many newlines we need to insert
                let avail_chars = width - spcs;
                debugln!("avail_chars: {}", avail_chars);
                let mut num_parts = h.len() / avail_chars;
                if h.len() % avail_chars != 0 {
                    num_parts += 1;
                }
                debugln!("num_parts: {}", num_parts);
                for i in 1..num_parts {
                    debugln!("i: {}", i);
                    let idx = if i != num_parts {
                        i * avail_chars
                    } else {
                        help.len() - 1
                    };
                    debugln!("idx: {}", idx);
                    help.insert(idx, '{');
                    help.insert(idx + 1, 'n');
                    help.insert(idx + 2, '}');
                }
            }
        }
        let help = if !h.is_empty() {
            &*help
        } else {
            h
        };
        if help.contains("{n}") {
            if let Some(part) = help.split("{n}").next() {
                try!(write!(w, "{}", part));
            }
            for part in help.split("{n}").skip(1) {
                try!(write!(w, "\n"));
                if self.nlh || a.is_set(ArgSettings::NextLineHelp) {
                    try!(write!(w, "{}{}", self.tab, self.tab));
                } else {
                    write_spaces!(self.l + 12, w);
                }
                try!(write!(w, "{}", part));
            }
        } else {
            try!(write!(w, "{}", help));
        }
        Ok(())
    }
}
