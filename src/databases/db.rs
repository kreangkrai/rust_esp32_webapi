pub struct DB{
    pub url: &'static str
}
impl DB{
    pub fn url() -> Self{
        DB {url:"postgres://postgres:Meeci500000@localhost:5432/postgres"}
    }
}
