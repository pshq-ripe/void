#[cfg(test)]
mod tests {
    use crate::app::ThemeColors;

    #[test]
    fn test_theme_colors_default() {
        let tc = ThemeColors::default();
        assert_eq!(tc.name, "Default");
        assert!(tc.is_dark);
        assert!(!tc.status_format.is_empty());
        assert!(!tc.input_prompt.is_empty());
        assert!(!tc.fmt_join.is_empty());
        assert!(!tc.fmt_quit.is_empty());
        assert!(!tc.ctcp_version.is_empty());
        assert!(!tc.default_kick_reason.is_empty());
    }

    #[test]
    fn test_theme_colors_nick_palette() {
        let tc = ThemeColors::default();
        assert!(!tc.nick_colors.is_empty());
        assert!(tc.nick_colors.len() >= 6);
    }

    #[test]
    fn test_theme_format_strings() {
        let tc = ThemeColors::default();
        assert!(tc.fmt_join.contains("$0"));
        assert!(tc.fmt_join.contains("$1"));
        assert!(tc.fmt_kick.contains("$0"));
        assert!(tc.fmt_kick.contains("$2"));
        assert!(tc.fmt_msg.contains("$0"));
        assert!(tc.fmt_msg.contains("$1"));
    }

    #[test]
    fn test_theme_ctcp_defaults() {
        let tc = ThemeColors::default();
        assert!(tc.ctcp_version.contains("Void"));
        assert!(!tc.ctcp_userinfo.is_empty());
        assert!(!tc.ctcp_source.is_empty());
    }

    #[test]
    fn test_theme_reasons_defaults() {
        let tc = ThemeColors::default();
        assert_eq!(tc.default_kick_reason, "Requested");
        assert_eq!(tc.default_part_reason, "Leaving");
        assert_eq!(tc.default_quit_reason, "Leaving");
    }
}
