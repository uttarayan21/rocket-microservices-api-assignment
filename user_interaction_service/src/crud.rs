use super::models::{InsertableUserInteraction, UserInteraction};

use super::schema::user_interactions;

use diesel::prelude::*;

pub fn get(c_id: i32, u_id: i32, connection: &PgConnection) -> QueryResult<UserInteraction> {
    use super::schema::user_interactions::dsl::{content_id, user_id};
    user_interactions::table
        .filter(content_id.eq(c_id))
        .filter(user_id.eq(u_id))
        .get_result::<UserInteraction>(connection)
}

pub fn get_content_likes(c_id: i32, connection: &PgConnection) -> QueryResult<i64> {
    use super::schema::user_interactions::dsl::{content_id, user_like};
    user_interactions::table
        .filter(content_id.eq(c_id))
        .filter(user_like.eq(true))
        .count()
        .get_result::<i64>(connection)
}

pub fn get_content_reads(c_id: i32, connection: &PgConnection) -> QueryResult<i64> {
    use super::schema::user_interactions::dsl::{content_id, user_read};
    user_interactions::table
        .filter(content_id.eq(c_id))
        .filter(user_read.eq(true))
        .count()
        .get_result::<i64>(connection)
}

pub fn get_content_top_reads(
    limit: usize,
    connection: &PgConnection,
) -> QueryResult<Vec<super::models::ContentFrequency>> {

    diesel::sql_query(format!(
        "select content_id, COUNT(content_id) as frequency
    from user_interactions where user_read = true
    group by content_id
    order by count(*) desc
    limit {}",
        limit
    ))
    .load::<super::models::ContentFrequency>(connection)
}

pub fn get_content_top_likes(
    limit: usize,
    connection: &PgConnection,
) -> QueryResult<Vec<super::models::ContentFrequency>> {
    diesel::sql_query(format!(
        "select content_id, COUNT(content_id) as frequency
    from user_interactions where user_like = true
    group by content_id
    order by count(*) desc
    limit {}",
        limit
    ))
    .load::<super::models::ContentFrequency>(connection)
}

pub fn read_and_like(
    content_id: i32,
    user_id: i32,
    connection: &PgConnection,
) -> QueryResult<UserInteraction> {
    if let Ok(user_interaction) = get(content_id, user_id, connection) {
        update(
            user_interaction.id,
            user_interaction.read().liked(),
            connection,
        )
    } else {
        insert(
            InsertableUserInteraction {
                content_id,
                user_id,
                user_like: true,
                user_read: true,
            },
            connection,
        )
    }
}

/// Read the content

pub fn read(
    content_id: i32,
    user_id: i32,
    connection: &PgConnection,
) -> QueryResult<UserInteraction> {
    if let Ok(user_interaction) = get(content_id, user_id, connection) {
        update(user_interaction.id, user_interaction.read(), connection)
    } else {
        insert(
            InsertableUserInteraction {
                content_id,
                user_id,
                user_like: false,
                user_read: true,
            },
            connection,
        )
    }
}

/// If this is called the content is not read but liked which shouldn't happen

pub fn _like(
    content_id: i32,
    user_id: i32,
    connection: &PgConnection,
) -> QueryResult<UserInteraction> {
    if let Ok(user_interaction) = get(content_id, user_id, connection) {
        update(user_interaction.id, user_interaction.liked(), connection)
    } else {
        insert(
            InsertableUserInteraction {
                content_id,
                user_id,
                user_like: true,
                user_read: false,
            },
            connection,
        )
    }
}

/// Unlike the content previously liked.
/// This function expects the value to already exist and then update it to unlike the story
pub fn unlike(
    content_id: i32,
    user_id: i32,
    connection: &PgConnection,
) -> QueryResult<UserInteraction> {
    if let Ok(user_interaction) = get(content_id, user_id, connection) {
        update(user_interaction.id, user_interaction.unliked(), connection)
    } else {
        Err(diesel::result::Error::NotFound)
    }
}

pub fn insert(
    content: InsertableUserInteraction,
    connection: &PgConnection,
) -> QueryResult<UserInteraction> {
    diesel::insert_into(user_interactions::table)
        .values(content)
        .get_result(connection)
}

pub fn update(
    id: i32,
    user_interaction: UserInteraction,
    connection: &PgConnection,
) -> QueryResult<UserInteraction> {
    diesel::update(user_interactions::table.find(id))
        .set(&user_interaction)
        .get_result(connection)
}

/// Since we are updating we already have the interaction id so this is redundant
pub fn _update_with_content_id(
    c_id: i32,
    u_id: i32,
    user_interaction: UserInteraction,
    connection: &PgConnection,
) -> QueryResult<UserInteraction> {
    use super::schema::user_interactions::dsl::{content_id, user_id};
    diesel::update(
        user_interactions::table
            .filter(content_id.eq(c_id))
            .filter(user_id.eq(u_id)),
    )
    .set(&user_interaction)
    .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(user_interactions::table.find(id)).execute(connection)
}
