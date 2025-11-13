use std::fs;
use std::path::Path;

use tiny_log_kv::kv_store::KvStore;

#[test]
fn test_set_get_delete() {
    let log = "test_store.log";
    if Path::new(log).exists() {
        fs::remove_file(log).unwrap();
    }

    {
        let mut store = KvStore::open(log).unwrap();
        store.set("a".to_string(), "1".to_string()).unwrap();
        store.set("b".to_string(), "2".to_string()).unwrap();
        assert_eq!(store.get("a"), Some(&"1".to_string()));
        assert_eq!(store.get("b"), Some(&"2".to_string()));
        store.delete("a").unwrap();
        assert_eq!(store.get("a"), None);
    }

    // Reload: "a" should be gone, "b" should remain!
    {
        let store = KvStore::open(log).unwrap();
        assert_eq!(store.get("a"), None);
        assert_eq!(store.get("b"), Some(&"2".to_string()));
    }

    fs::remove_file(log).unwrap();
}
