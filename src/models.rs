#[derive(Queryable)]
pub struct users {
    pub id: i32,
    pub name: String,
    pub offset: i8,
}
