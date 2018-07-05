#[derive(Debug)]
pub struct Task<'a> {
    pub id: i32,
    pub text: &'a str,
    pub created: i64,
}
