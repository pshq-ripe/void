#[cfg(test)]
mod tests {
    use crate::app::Settings;

    #[test]
    fn test_settings_defaults() {
        let settings = Settings::new();
        assert_eq!(settings.get("SCROLL_LINES"), "1");
        assert_eq!(settings.get("SCROLLBACK"), "500");
        assert_eq!(settings.get("SHOW_TIMESTAMPS"), "ON");
        assert_eq!(settings.get("TIMESTAMP_FORMAT"), "%H:%M:%S");
        assert_eq!(settings.get("MOUSE"), "OFF");
        assert_eq!(settings.get("SHOW_NICKLIST"), "ON");
        assert_eq!(settings.get("SHOW_STATUSBAR"), "ON");
        assert_eq!(settings.get("SHOW_USER_COUNT"), "ON");
        assert_eq!(settings.get("SHOW_BUFFER_LIST"), "OFF");
    }

    #[test]
    fn test_settings_set_get() {
        let mut settings = Settings::new();
        settings.set("MY_KEY", "my_value");
        assert_eq!(settings.get("MY_KEY"), "my_value");
    }

    #[test]
    fn test_settings_get_bool() {
        let mut settings = Settings::new();
        assert!(settings.get_bool("SHOW_TIMESTAMPS"));
        assert!(!settings.get_bool("MOUSE"));

        settings.set("MOUSE", "ON");
        assert!(settings.get_bool("MOUSE"));

        settings.set("MOUSE", "off");
        assert!(!settings.get_bool("MOUSE"));
    }

    #[test]
    fn test_settings_get_int() {
        let mut settings = Settings::new();
        assert_eq!(settings.get_int("SCROLL_LINES"), 1);
        assert_eq!(settings.get_int("SCROLLBACK"), 500);

        settings.set("SCROLL_LINES", "5");
        assert_eq!(settings.get_int("SCROLL_LINES"), 5);
    }

    #[test]
    fn test_settings_case_insensitive_get() {
        let mut settings = Settings::new();
        settings.set("MY_KEY", "value");
        assert_eq!(settings.get("my_key"), "value");
        assert_eq!(settings.get("MY_KEY"), "value");
    }

    #[test]
    fn test_settings_flood_protection() {
        let mut settings = Settings::new();
        settings.set("FLOOD_PROTECTION", "ON");
        assert!(settings.get_bool("FLOOD_PROTECTION"));
    }
}
