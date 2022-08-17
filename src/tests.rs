#![allow(unused)]
use proptest::prelude::*;
use crate::*;

proptest! {
    #[test]
    fn can_create(user in "\\PC*", group in "\\PC*", pass in "\\PC*") {
        let log = LogState::new(&user, &group, &pass);

        log.save(&pass);
        assert!(is_store_existing(&log.get_filename()));
        log.delete_store();
    }

    #[test]
    fn can_delete(user in "\\PC*", group in "\\PC*", pass in "\\PC*") {
        let log = LogState::new(&user, &group, &pass);

        log.save(&pass);
        log.delete_store();
        assert!(!is_store_existing(&log.get_filename()));
    }

    #[test]
    fn can_read(user in "\\PC*", group in "\\PC*", pass in "\\PC*") {
        let log_a = LogState::new(&user, &group, &pass);

        log_a.save(&pass);

        let log_b = LogState::new(&user, &group, &pass);

        assert!(log_a == log_b);

        log_a.delete_store();
    }
}

