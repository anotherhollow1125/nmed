use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

use clap::Parser;
use color_eyre::{eyre::anyhow, Result};
use fancy_regex::Regex;
use ratatui::crossterm::event::{KeyCode, ModifierKeyCode};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Prompt Key
    #[arg(short, long, default_value_t = KeyCode::Char('/'), value_parser = str2key)]
    pub prompt_key: KeyCode,
    /// Asist Key
    #[arg(short, long, default_value_t = KeyCode::Char('@'), value_parser = str2key)]
    pub assist_key: KeyCode,
}

fn str2key(s: &str) -> Result<KeyCode> {
    let s_lowered = s.to_lowercase().replace("_", "");

    // Specified Key
    if let Some(&key) = STR2KEYMAP.get(s_lowered.as_str()) {
        return Ok(key);
    }

    static RE_FANC: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^(f|F|func|Func)(\d+)$").unwrap());

    let f_wrapped = RE_FANC
        .captures(&s_lowered)
        .ok()
        .flatten()
        .and_then(|caps| caps.get(2))
        .and_then(|mch| mch.as_str().parse::<u8>().ok());

    // Function Key
    if let Some(func_num) = f_wrapped {
        return Ok(KeyCode::F(func_num));
    }

    // Char Key
    if s.len() == 1 {
        Ok(KeyCode::Char(s.chars().next().unwrap()))
    } else {
        Err(anyhow!("Invalid Key: {}", s))
    }
}

static STR2KEYMAP: LazyLock<HashMap<&str, KeyCode>> = LazyLock::new(|| {
    [
        ("backspace", KeyCode::Backspace),
        ("enter", KeyCode::Enter),
        ("left", KeyCode::Left),
        ("right", KeyCode::Right),
        ("up", KeyCode::Up),
        ("down", KeyCode::Down),
        ("home", KeyCode::Home),
        ("end", KeyCode::End),
        ("pageup", KeyCode::PageUp),
        ("pagedown", KeyCode::PageDown),
        ("tab", KeyCode::Tab),
        ("backtab", KeyCode::BackTab),
        ("delete", KeyCode::Delete),
        ("insert", KeyCode::Insert),
        ("null", KeyCode::Null),
        ("esc", KeyCode::Esc),
        ("escape", KeyCode::Esc),
        ("capslock", KeyCode::CapsLock),
        ("scrolllock", KeyCode::ScrollLock),
        ("numlock", KeyCode::NumLock),
        ("printscreen", KeyCode::PrintScreen),
        ("pause", KeyCode::Pause),
        ("menu", KeyCode::Menu),
        ("keypadbegin", KeyCode::KeypadBegin),
        /* // Manually Settings
        Media(_media_key_code),
        Modifier(_modifier_key_code),
        F(_),
        Char(_),
        */
    ]
    .into_iter()
    .collect()
});

#[allow(unused)]
fn str2modkeys(s: &str) -> Result<HashSet<KeyCode>> {
    todo!()
}

#[allow(unused)]
static STR2METAKEYMAP: LazyLock<HashMap<&str, HashSet<ModifierKeyCode>>> = LazyLock::new(|| {
    [
        ("left_shift", vec![ModifierKeyCode::LeftShift]),
        ("right_shift", vec![ModifierKeyCode::RightShift]),
        (
            "shift",
            vec![ModifierKeyCode::LeftShift, ModifierKeyCode::RightShift],
        ),
        ("left_control", vec![ModifierKeyCode::LeftControl]),
        ("right_control", vec![ModifierKeyCode::RightControl]),
        (
            "control",
            vec![ModifierKeyCode::LeftControl, ModifierKeyCode::RightControl],
        ),
        ("left_alt", vec![ModifierKeyCode::LeftAlt]),
        ("right_alt", vec![ModifierKeyCode::RightAlt]),
        (
            "alt",
            vec![ModifierKeyCode::LeftAlt, ModifierKeyCode::RightAlt],
        ),
        ("left_super", vec![ModifierKeyCode::LeftSuper]),
        ("right_super", vec![ModifierKeyCode::RightSuper]),
        (
            "super",
            vec![ModifierKeyCode::LeftSuper, ModifierKeyCode::RightSuper],
        ),
        ("left_hyper", vec![ModifierKeyCode::LeftHyper]),
        ("right_hyper", vec![ModifierKeyCode::RightHyper]),
        (
            "hyper",
            vec![ModifierKeyCode::LeftHyper, ModifierKeyCode::RightHyper],
        ),
        ("left_meta", vec![ModifierKeyCode::LeftMeta]),
        ("right_meta", vec![ModifierKeyCode::RightMeta]),
        (
            "meta",
            vec![ModifierKeyCode::LeftMeta, ModifierKeyCode::RightMeta],
        ),
        ("iso_level3_shift", vec![ModifierKeyCode::IsoLevel3Shift]),
        ("iso_level5_shift", vec![ModifierKeyCode::IsoLevel5Shift]),
    ]
    .into_iter()
    .map(|(s, keys)| (s, keys.into_iter().collect()))
    .collect()
});

#[cfg(test)]
mod tests {
    use ratatui::crossterm::event::KeyCode;

    use super::str2key;

    #[test]
    fn str2key_test_ok() {
        // Specified Keys
        assert_eq!(str2key("backspace").unwrap(), KeyCode::Backspace);
        assert_eq!(str2key("enter").unwrap(), KeyCode::Enter);
        assert_eq!(str2key("left").unwrap(), KeyCode::Left);
        assert_eq!(str2key("Backspace").unwrap(), KeyCode::Backspace);
        assert_eq!(str2key("Enter").unwrap(), KeyCode::Enter);
        assert_eq!(str2key("Left").unwrap(), KeyCode::Left);
        assert_eq!(str2key("back_space").unwrap(), KeyCode::Backspace);
        assert_eq!(str2key("e_n_t_e_r").unwrap(), KeyCode::Enter);
        assert_eq!(str2key("l_e_f_t").unwrap(), KeyCode::Left);

        // Function Keys
        assert_eq!(str2key("f1").unwrap(), KeyCode::F(1));
        assert_eq!(str2key("F2").unwrap(), KeyCode::F(2));
        assert_eq!(str2key("func11").unwrap(), KeyCode::F(11));
        assert_eq!(str2key("Func12").unwrap(), KeyCode::F(12));

        // Char Keys
        assert_eq!(str2key("a").unwrap(), KeyCode::Char('a'));
        assert_eq!(str2key("A").unwrap(), KeyCode::Char('A'));
        assert_eq!(str2key("f").unwrap(), KeyCode::Char('f'));
        assert_eq!(str2key("F").unwrap(), KeyCode::Char('F'));
        assert_eq!(str2key("1").unwrap(), KeyCode::Char('1'));
        assert_eq!(str2key("9").unwrap(), KeyCode::Char('9'));
        assert_eq!(str2key("0").unwrap(), KeyCode::Char('0'));
        assert_eq!(str2key("/").unwrap(), KeyCode::Char('/'));
    }

    #[test]
    fn str2key_test_invalid() {
        assert!(str2key("invalid").is_err());
        assert!(str2key("func").is_err());
        assert!(str2key("Back Space").is_err());
    }
}
