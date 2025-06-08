use std::collections::HashMap;
use std::sync::LazyLock;

pub static COLORS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("red", "\x1b[0;31m"),
        ("green", "\x1b[0;32m"),
        ("yellow", "\x1b[0;33m"),
        ("blue", "\x1b[0;34m"),
        ("magenta", "\x1b[0;35m"),
        ("cyan", "\x1b[0;36m"),
        ("grey", "\x1b[0;37m"),

        ("lightred", "\x1b[0;91m"),
        ("lightgreen", "\x1b[0;92m"),
        ("lightyellow", "\x1b[0;93m"),
        ("lightblue", "\x1b[0;94m"),
        ("lightmagenta", "\x1b[0;95m"),
        ("lightcyan", "\x1b[0;96m"),
        ("white", "\x1b[0;97m"),

        ("bold", "\x1b[1m"),
        ("soft", "\x1b[2m"),
        ("reset", "\x1b[0m"),
    ])
});