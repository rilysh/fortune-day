/// Get the size of the current terminal window, via ioctl() call.
/// e,g. get_termsize()
/// *********************
/// This function returns two u16 values. First one is the row size
/// and the second one is the column size.
pub fn get_termsize() -> (u16, u16) {
    let mut wsz = libc::winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    unsafe {
        if libc::ioctl(0, libc::TIOCGWINSZ, &mut wsz) == -1 {
            panic!(
                "panic invoked, as {} returned -1.",
                "libc::ioctl(0, libc::TIOCGWINSZ, &mut wsz)"
            );
        }
        (wsz.ws_row, wsz.ws_col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_termsize() {
        let (row, col) = get_termsize();
        assert_eq!(row > 0, true);
        assert_eq!(col > 0, true);
    }
}
