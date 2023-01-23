extern crate once_cell;

use once_cell::sync::OnceCell;

pub fn is_unicode_supported() -> bool {
    static CACHED_RESULT: OnceCell<bool> = OnceCell::new();

    *CACHED_RESULT.get_or_init(|| _is_unicode_supported())
}

fn _is_unicode_supported() -> bool {
    if std::env::consts::OS != "windows" {
        return !is_linux_console_kernel();
    }

    is_ci_like()
        || is_windows_terminal()
        || is_older_terminus()
        || is_con_emu_cmder()
        || is_terminus()
        || is_vscode()
        || is_xterm_256_color()
        || is_alacritty()
        || is_jetbrains_jediterm()
}

fn is_linux_console_kernel() -> bool {
    match std::env::var("TERM") {
        Ok(string) => string == "linux",
        Err(_error) => false,
    }
}

fn is_ci_like() -> bool {
    std::env::var_os("CI").is_some()
}

fn is_windows_terminal() -> bool {
    std::env::var_os("WT_SESSION").is_some()
}

fn is_older_terminus() -> bool {
    std::env::var_os("TERMINUS_SUBLIME").is_some()
}

fn is_con_emu_cmder() -> bool {
    match std::env::var("ConEmuTask") {
        Ok(string) => string == "{cmd::Cmder}",
        Err(_error) => false,
    }
}

fn is_terminus() -> bool {
    match std::env::var("TERM_PROGRAM") {
        Ok(string) => string == "Terminus-Sublime",
        Err(_error) => false,
    }
}

fn is_vscode() -> bool {
    match std::env::var("TERM_PROGRAM") {
        Ok(string) => string == "vscode",
        Err(_error) => false,
    }
}

fn is_xterm_256_color() -> bool {
    match std::env::var("TERM") {
        Ok(string) => string == "xterm-256color",
        Err(_error) => false,
    }
}

fn is_alacritty() -> bool {
    match std::env::var("TERM") {
        Ok(string) => string == "alacritty",
        Err(_error) => false,
    }
}

fn is_jetbrains_jediterm() -> bool {
    match std::env::var("TERMINAL_EMULATOR") {
        Ok(string) => string == "JetBrains-JediTerm",
        Err(_error) => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::is_unicode_supported;

    #[test]
    fn should_work_on_ci() {
        assert_eq!(is_unicode_supported(), true);
    }
}
