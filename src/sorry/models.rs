use super::schema::sorry_errors;


#[derive(Queryable)]
pub struct SorryError {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "sorry_errors"]
pub struct SorryErrorForm<'a> {
    pub title: &'a str,
    pub body: Option<&'a str>,
}
