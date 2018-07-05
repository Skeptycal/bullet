use std::time;

pub fn timestamp() -> i64 {
    let now = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap();
    now.as_secs() as i64
}
