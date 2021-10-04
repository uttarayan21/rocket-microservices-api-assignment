use super::models::{Content, InsertableContent};

use super::schema::contents;

use diesel::prelude::*;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Content>> {
    contents::table.load::<Content>(connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Content> {
    contents::table.find(id).get_result::<Content>(connection)
}

pub fn get_list(ids: Vec<i32>, connection: &PgConnection) -> QueryResult<Vec<Content>> {
    // contents::table.filter(contents::id.in(ids)).get_results::<Content>(&*connection)
    if ids.is_empty() {
        return Err(diesel::result::Error::NotFound);
    }
    let mut idstring: String = String::new();
    println!("{:?}", &ids);
    for id in ids {
        idstring.push_str(&id.to_string());
        idstring.push(',');
    }
    let (idstring, _) = idstring.split_at(idstring.len() - 1);
    let query: String = format!("SELECT * FROM contents WHERE id IN ({})", idstring);
    diesel::sql_query(query).get_results::<Content>(connection)
}

pub fn insert(content: InsertableContent, connection: &PgConnection) -> QueryResult<Content> {
    diesel::insert_into(contents::table)
        .values(content)
        .get_result(connection)
}

pub fn update(id: i32, content: Content, connection: &PgConnection) -> QueryResult<Content> {
    diesel::update(contents::table.find(id))
        .set(&content)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(contents::table.find(id)).execute(connection)
}

pub fn ingest(
    contents: Vec<InsertableContent>,
    connection: &PgConnection,
) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(contents::table)
        .values(&contents)
        .execute(connection)
}

pub fn new(count: i64, connection: &PgConnection) -> QueryResult<Vec<Content>> {
    use super::schema::contents::dsl::published;
    contents::table
        .order(published.desc())
        .limit(count)
        .load::<Content>(&*connection)
}
