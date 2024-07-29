/// Enum of color types.
#[allow(dead_code)]
pub enum Colors {
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    End,
}

/// Get a color based on the enum member.
/// e.g. Colors::Red -> \x1b[1;91m
pub fn color<'a>(colors: Colors) -> &'a str {
    match colors {
        Colors::Red => "\x1b[1;91m",
        Colors::Green => "\x1b[1;92m",
        Colors::Yellow => "\x1b[1;93m",
        Colors::Blue => "\x1b[1;94m",
        Colors::Purple => "\x1b[1;95m",
        Colors::Cyan => "\x1b[1;96m",
        Colors::White => "\x1b[1;97m",
        Colors::End => "\x1b[0m",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        assert_eq!(color(Colors::Red) == "\x1b[1;91m", true)
    }
}
