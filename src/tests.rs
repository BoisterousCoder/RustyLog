#![allow(unused)]
use proptest::prelude::*;
use crate::*;
use serial_test::serial;

const ALLOWED_CHARS:&str = ".";

proptest! {
    #[test]
    #[serial]
    fn can_create_store(user in ALLOWED_CHARS, group in ALLOWED_CHARS, pass in ALLOWED_CHARS) {
        let log = LogState::new(&user, &group, &pass);

        log.save(&pass);

        assert!(is_store_existing(&log.filename()));
        log.delete_store();
    }

    #[test]
    #[serial]
    fn can_delete_store(user in ALLOWED_CHARS, group in ALLOWED_CHARS, pass in ALLOWED_CHARS) {
        let log = LogState::new(&user, &group, &pass);

        log.save(&pass);
        log.delete_store();
        assert!(!is_store_existing(&log.filename()));
    }

    #[test]
    #[serial]
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
    #[serial]
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

        assert!(log_b.messages()[0] == &msg);
        log_a.delete_store();
    }

    #[test]
    #[serial]
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
    #[test]
    #[serial]
    fn can_stop_duplicates(from in ALLOWED_CHARS, tag in ALLOWED_CHARS, content in ALLOWED_CHARS, signature in ALLOWED_CHARS) {
        let msg = MessageData {
            from:from,
            tag:tag,
            content:content,
            signature:signature,
            signed_time_stamp:"1999 Jun 8 12:09:14.274 +0000".to_string()
        };
        let pass = "password";

        let mut log = LogState::new("Alice", "test", pass);

        log.add_message(msg.clone());

        let log_clone = log.clone();

        log.add_message(msg.clone());

        assert!(log == log_clone);
    }
    #[test]
    #[serial]
    fn can_stop_duplicates_two(from in ALLOWED_CHARS, tag in ALLOWED_CHARS, content in ALLOWED_CHARS, signature in ALLOWED_CHARS) {
        let msg = MessageData {
            from:from,
            tag:tag,
            content:content,
            signature:signature,
            signed_time_stamp:"1999 Jun 8 12:09:14.274 +0000".to_string()
        };
        let pass = "password";

        let mut log = LogState::new("Alice", "test", pass);

        let op = log.add_message(msg).unwrap();

        let log_clone = log.clone();

        log.apply_op(&op);

        assert!(log == log_clone);
    }
}

