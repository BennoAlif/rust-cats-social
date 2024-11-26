use crate::entities::cat::{Cat, CatResponse, CreateCatPayload, CreateCatResponse, FilterCat};
use sqlx::{PgPool, QueryBuilder, Row};

pub async fn insert_cat(
    pool: &PgPool,
    cat: CreateCatPayload,
) -> Result<CreateCatResponse, sqlx::Error> {
    sqlx::query_as::<_, CreateCatResponse>(
        "INSERT INTO cats (name, race, sex, age_in_month, description, img_urls, user_id) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, created_at",
    )
    .bind(cat.name.to_string())
    .bind(cat.race.to_string())
    .bind(cat.sex.to_string())
    .bind(cat.age_in_month)
    .bind(cat.description.to_string())
    .bind(cat.img_urls)
    .bind(cat.user_id)
    .fetch_one(pool)
    .await
}

pub async fn find_many_cats(
    pool: &PgPool,
    filter: FilterCat,
) -> Result<Vec<CatResponse>, sqlx::Error> {
    let mut query = QueryBuilder::<sqlx::Postgres>::new("SELECT id, name, race, sex, age_in_month, description, img_urls, created_at, user_id FROM cats WHERE 1=1 ");
    let mut has_condition = true;

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

    if let Some(search) = filter.search {
        if has_condition {
            query.push(" AND ");
        }
        query.push("name = ");
        query.push_bind(search);
        has_condition = true;
    }

    if let Some(race) = filter.race {
        if has_condition {
            query.push(" AND ");
        }
        query.push("race = ");
        query.push_bind(race);
        has_condition = true;
    }

    if let Some(sex) = filter.sex {
        if has_condition {
            query.push(" AND ");
        }
        query.push("sex = ");
        query.push_bind(sex);
        has_condition = true;
    }

    if let Some(age_in_month) = filter.age_in_month {
        if has_condition {
            query.push(" AND ");
        }
        if age_in_month.starts_with('>') {
            query.push("age_in_month > ");
            query.push_bind(age_in_month[1..].parse::<i32>().unwrap());
        } else if age_in_month.starts_with('<') {
            query.push("age_in_month < ");
            query.push_bind(age_in_month[1..].parse::<i32>().unwrap());
        } else {
            query.push("age_in_month = ");
            query.push_bind(age_in_month[1..].parse::<i32>().unwrap());
        }
        has_condition = true;
    }

    if let Some(_) = filter.owned {
        if has_condition {
            query.push(" AND ");
        }
        query.push("user_id = ");
        query.push_bind(filter.user_id);
    }

    query.push(" ORDER BY created_at DESC");

    query.push(" LIMIT ");
    query.push_bind(filter.limit);
    query.push(" OFFSET ");
    query.push_bind(filter.offset);

    let query = query.build();

    query.fetch_all(pool).await.map(|rows| {
        rows.iter()
            .map(|row| CatResponse {
                id: row.get("id"),
                name: row.get("name"),
                race: row.get("race"),
                sex: row.get("sex"),
                age_in_month: row.get("age_in_month"),
                description: row.get("description"),
                img_urls: row.get("img_urls"),
                created_at: row.get("created_at"),
                has_matched: false,
            })
            .collect()
    })
}

pub async fn find_one_cat(pool: &PgPool, id: i32) -> Result<Cat, sqlx::Error> {
    sqlx::query_as::<_, Cat>(
        "SELECT id, name, race, sex, age_in_month, description, img_urls, created_at, user_id FROM cats WHERE id = $1",
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn update_cat(
    pool: &PgPool,
    id: i32,
    cat: CreateCatPayload,
) -> Result<CreateCatResponse, sqlx::Error> {
    sqlx::query_as::<_, CreateCatResponse>(
        "UPDATE cats SET name = $2, race = $3, sex = $4, age_in_month = $5, description = $6, img_urls = $7 WHERE id = $1 RETURNING id, created_at",
    )
    .bind(id)
    .bind(cat.name.to_string())
    .bind(cat.race.to_string())
    .bind(cat.sex.to_string())
    .bind(cat.age_in_month)
    .bind(cat.description.to_string())
    .bind(cat.img_urls)
    .fetch_one(pool)
    .await
}

pub async fn delete_cat(pool: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM cats WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map(|_| ())
}
