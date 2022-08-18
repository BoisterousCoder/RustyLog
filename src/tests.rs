#![allow(unused)]
use proptest::prelude::*;
use crate::*;

//const ALLOWED_CHARS:&str = "[a-zA-Z]+0123456789!@#\\$%\\^&\\*\\(\\)_\\+-=\\{\\}\\[\\]:\";\'<>\\?,\\./\\\\ ";
const ALLOWED_CHARS:&str = ".";

proptest! {
    #[test]
    fn can_create_store(user in ALLOWED_CHARS, group in ALLOWED_CHARS, pass in ALLOWED_CHARS) {
        let log = LogState::new(&user, &group, &pass);

        log.save(&pass);
        assert!(is_store_existing(&log.filename()));
        log.delete_store();
    }

    #[test]
    fn can_delete_store(user in ALLOWED_CHARS, group in ALLOWED_CHARS, pass in ALLOWED_CHARS) {
        let log = LogState::new(&user, &group, &pass);

        log.save(&pass);
        assert!(!is_store_existing(&log.filename()));
        log.delete_store();
    }

    #[test]
    fn can_read_store(user in ALLOWED_CHARS, group in ALLOWED_CHARS, pass in ALLOWED_CHARS) {
        let msg = MessageData {
            from:"Alice".to_string(),
            tag:"/s".to_string(),
            content:"Some base64 data".to_string(),
            signature:"Some base64 data".to_string(),
            signed_time_stamp:"1999 Jun 8 12:09:14.274 +0000".to_string()
        };

        let mut log_a = LogState::new(&user, &group, &pass);

        log_a.add_message(msg);

        log_a.save(&pass);

        let log_b = LogState::new(&user, &group, &pass);
        let log_c = LogState::new(&format!("Not {}", user), &group, &pass);


        assert!(log_a == log_b);
        assert!(log_a != log_c);
        log_a.delete_store();
    }

    #[test]
    fn can_read_msg(from in ALLOWED_CHARS, tag in ALLOWED_CHARS, content in ALLOWED_CHARS, signature in ALLOWED_CHARS) {
        let msg = MessageData {
            from:from,
            tag:tag,
            content:content,
            signature:signature,
            signed_time_stamp:"1999 Jun 8 12:09:14.274 +0000".to_string()
        };
        let pass = "password";

        let mut log_a = LogState::new("Alice", "test", pass);

        log_a.add_message(msg.clone());

        log_a.save(pass);

        let log_b = LogState::new("Alice", "test", pass);

        assert!(log_b.msgs()[0] == &msg);
        log_a.delete_store();
    }

    #[test]
    fn can_read_decrypt_data(from in ALLOWED_CHARS, content in ALLOWED_CHARS) {
        let msg = MessageData {
            from:from,
            tag:"/s".to_string(),
            content:"Some base64 data".to_string(),
            signature:"Some base64 data".to_string(),
            signed_time_stamp:"2022 Aug 17 1:09:01.666 +0000".to_string()
        };
        let pass = "password";

        let mut log_a = LogState::new("Alice", "test", pass);

        log_a.add_message(msg.clone());
        log_a.decrypt(&msg, &content);

        log_a.save(pass);

        let log_b = LogState::new("Alice", "test", pass);

        assert!(log_b.decrypted(&msg) == Some(content));
        log_a.delete_store();
    }
}

