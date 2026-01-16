use mini_redis_tls::Db;
use bytes::Bytes;

#[tokio::test]
async fn test_concurrent_access() {
    let db = Db::new();

    // Seed data
    db.set("key".to_string(), Bytes::from("value"));

    let mut handles = Vec::new();

    // Spawn 100 readers
    for _ in 0..100 {
        let db = db.clone();
        handles.push(tokio::spawn(async move {
            let val = db.get("key");
            assert_eq!(val, Some(Bytes::from("value")));
        }));
    }

    // Spawn a writer
    let db_writer = db.clone();
    handles.push(tokio::spawn(async move {
        db_writer.set("key2".to_string(), Bytes::from("value2"));
    }));

    // Wait for all
    for handle in handles {
        handle.await.unwrap();
    }

    assert_eq!(db.get("key2"), Some(Bytes::from("value2")));
}
