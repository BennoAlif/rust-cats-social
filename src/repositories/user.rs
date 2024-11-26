use crate::entities::user::{CreateUser, FilterUser, User};
use sqlx::{PgPool, QueryBuilder, Row};

pub async fn insert_user(pool: &PgPool, user: CreateUser) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email, password",
    )
    .bind(user.name.to_string())
    .bind(user.email.to_string())
    .bind(user.password.to_string())
    .fetch_one(pool)
    .await
}

pub async fn find_one_user(pool: &PgPool, filter: FilterUser) -> Result<User, sqlx::Error> {
    let mut query =
        QueryBuilder::<sqlx::Postgres>::new("SELECT id, name, email, password FROM users WHERE ");
    let mut has_condition = false;

    if let Some(id) = filter.id {
        if id > 0 {
            if has_condition {
                query.push(" AND ");
            }
            query.push("id = ");
            query.push_bind(id);
            has_condition = true;
        }
    }

    if let Some(email) = filter.email {
        if has_condition {
            query.push(" AND ");
        }
        query.push("email = ");
        query.push_bind(email);
        has_condition = true;
    }

    if let Some(name) = filter.name {
        if has_condition {
            query.push(" AND ");
        }
        query.push("name = ");
        query.push_bind(name);
    }

    query.push(" LIMIT 1");

    let query = query.build();

    query.fetch_one(pool).await.map(|row| User {
        id: row.get("id"),
        name: row.get("name"),
        email: row.get("email"),
        password: row.get("password"),
    })
}
