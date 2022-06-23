macro_rules! pretty {
    ($s:expr, $($flag:tt)|* + fg@$fg:tt + bg@$bg:tt) => {{
        let style = ansi_term::Style::new();

        let style = pretty!(__$($flag)|*, style);
        let style = style.fg(pretty!(__col $fg));
        let style = style.bg(pretty!(__col $bg));

        style.paint($s)
    }};

    ($s:expr, $($flag:tt)|* + fg@$fg:tt) => {{
        let style = ansi_term::Style::new();

        let style = pretty!(__$($flag)|*, style);
        let style = style.fg(pretty!(__col $fg));

        style.paint($s)
    }};

    ($s:expr, $($flag:tt)|* + bg@$bg:tt) => {{
        let mut style = ansi_term::Style::new();

        let style = pretty!(__$($flag)|*, style);
        let style = style.bg(pretty!(__col $bg));

        style.paint($s)
    }};

    ($s:expr, $($flag:tt)|*) => {{
        let style = ansi_term::Style::new();

        let style = pretty!(__$($flag)|*, style);

        style.paint($s)
    }};

    ($s:expr, fg@$fg:tt + bg@$bg:tt) => {{
        let style = ansi_term::Style::new();

        let style = style.fg(pretty!(__col $fg));
        let style = style.bg(pretty!(__col $bg));

        style.paint($s)
    }};

    ($s:expr, fg@$fg:tt) => {{
        let style = ansi_term::Style::new();

        let style = style.fg(pretty!(__col $fg));

        style.paint($s)
    }};

    ($s:expr, bg@$bg:tt) => {{
        let style = ansi_term::Style::new();

        let style = style.bg(pretty!(__col $bg));

        style.paint($s)
    }};

    (__$($flag:tt)|*, $style:expr) => {{
        let mut style = $style;
        for f in [$(
            stringify!($flag)
        ),+] {
            style = match f.to_lowercase().as_str() {
                "b" => style.bold(),
                "_" => style.underline(),
                "i" => style.italic(),
                "~" => style.strikethrough(),
                "blink" => style.blink(),
                "dim" => style.dimmed(),
                "hidden" => style.hidden(),
                "reverse" => style.reverse(),
                _ => panic!("Unknown flag: {}", f),
            }
        }
        style
    }};

    (__col $col:tt) => {{
        match stringify!($col).to_lowercase().as_str() {
            "black" => ansi_term::Colour::Black,
            "red" => ansi_term::Colour::Red,
            "green" => ansi_term::Colour::Green,
            "yellow" => ansi_term::Colour::Yellow,
            "blue" => ansi_term::Colour::Blue,
            "purple" => ansi_term::Colour::Purple,
            "cyan" => ansi_term::Colour::Cyan,
            "white" => ansi_term::Colour::White,
            s if s.len() == 6 => {
                // Convert the RGB hex string to 3 bytes.
                let r = u8::from_str_radix(&s[0..2], 16).unwrap();
                let g = u8::from_str_radix(&s[2..4], 16).unwrap();
                let b = u8::from_str_radix(&s[4..6], 16).unwrap();
                ansi_term::Colour::RGB(r, g, b)
            },
            s => {
                ansi_term::Colour::Fixed(s.parse::<u8>().unwrap())
            }
        }
    }}
}

// https://stackoverflow.com/a/31749071/12057036
pub(crate) use pretty;
