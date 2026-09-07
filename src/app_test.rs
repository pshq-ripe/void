#[cfg(test)]
mod tests {
    use crate::app::{Buffer, MessageType, WindowLevel};

    #[test]
    fn test_buffer_new() {
        let buf = Buffer::new("#test");
        assert_eq!(buf.name, "#test");
        assert!(buf.messages.is_empty());
        assert!(buf.nicks.is_empty());
        assert_eq!(buf.topic, "");
        assert_eq!(buf.scroll_offset, 0);
        assert_eq!(buf.unread_count, 0);
        assert!(!buf.has_activity);
        assert_eq!(buf.level, WindowLevel::All);
    }

    #[test]
    fn test_buffer_push_message() {
        let mut buf = Buffer::new("#test");
        buf.push_message("hello".to_string(), MessageType::Normal, 500);
        assert_eq!(buf.messages.len(), 1);
        assert_eq!(buf.messages[0].text, "hello");
    }

    #[test]
    fn test_buffer_scrollback_limit() {
        let mut buf = Buffer::new("#test");
        for i in 0..200 {
            buf.push_message(format!("msg {}", i), MessageType::Normal, 100);
        }
        assert_eq!(buf.messages.len(), 100);
        assert_eq!(buf.messages[0].text, "msg 100");
    }

    #[test]
    fn test_buffer_add_remove_nick() {
        let mut buf = Buffer::new("#test");
        buf.add_nick("user1");
        buf.add_nick("user2");
        assert_eq!(buf.nicks.len(), 2);
        assert!(buf.nicks.iter().any(|n| n.nick == "user1"));

        buf.remove_nick("user1");
        assert_eq!(buf.nicks.len(), 1);
        assert!(!buf.nicks.iter().any(|n| n.nick == "user1"));
    }

    #[test]
    fn test_buffer_rename_nick() {
        let mut buf = Buffer::new("#test");
        buf.add_nick("oldnick");
        buf.rename_nick("oldnick", "newnick");
        assert!(buf.nicks.iter().any(|n| n.nick == "newnick"));
        assert!(!buf.nicks.iter().any(|n| n.nick == "oldnick"));
    }

    #[test]
    fn test_buffer_set_nick_prefix() {
        let mut buf = Buffer::new("#test");
        buf.add_nick("user1");
        buf.set_nick_prefix("user1", '@', true);
        let nick = buf.nicks.iter().find(|n| n.nick == "user1").unwrap();
        assert_eq!(nick.prefix, "@");
    }

    #[test]
    fn test_window_level_default() {
        let buf = Buffer::new("#test");
        assert_eq!(buf.level, WindowLevel::All);
    }

    #[test]
    fn test_buffer_scroll() {
        let mut buf = Buffer::new("#test");
        for i in 0..20 {
            buf.push_message(format!("msg {}", i), MessageType::Normal, 500);
        }
        buf.scroll_offset = 5;
        assert_eq!(buf.scroll_offset, 5);
    }

    #[test]
    fn test_buffer_topic() {
        let mut buf = Buffer::new("#test");
        buf.topic = "Test topic".to_string();
        assert_eq!(buf.topic, "Test topic");
    }
}
