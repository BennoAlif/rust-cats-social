use crate::{
    entities::{
        cat::{CatResponse, CreateCatPayload, CreateCatRequest, CreateCatResponse, FilterCat},
        ResponseWrapper,
    },
    middlewares::auth::Auth,
    repositories::cat::{find_many_cats, find_one_cat, insert_cat, update_cat},
    AppState,
};
use actix_web::{
    delete, get, post, put, web,
    web::{Data, Json},
    HttpResponse, Responder,
};
use validator::Validate;

#[get("")]
async fn get_cats(
    state: Data<AppState>,
    Auth(user): Auth,
    query: web::Query<FilterCat>,
) -> impl Responder {
    let filter = FilterCat {
        id: query.id,
        search: query.search.clone(),
        limit: query.limit,
        offset: query.offset,
        race: query.race.clone(),
        sex: query.sex.clone(),
        age_in_month: query.age_in_month.clone(),
        has_matched: query.has_matched,
        owned: query.owned,
        user_id: Some(user.id),
    };

    match find_many_cats(&state.db, filter).await {
        Ok(cats) => {
            let cats: Vec<CatResponse> = cats
                .into_iter()
                .map(|cat| {
                    let has_matched = false;
                    CatResponse {
                        id: cat.id,
                        name: cat.name,
                        race: cat.race,
                        sex: cat.sex,
                        age_in_month: cat.age_in_month,
                        description: cat.description,
                        img_urls: cat.img_urls,
                        created_at: cat.created_at,
                        has_matched,
                    }
                })
                .collect();

            HttpResponse::Ok().json(ResponseWrapper::<Vec<CatResponse>> {
                message: "Cats fetched successfully".to_string(),
                data: Some(cats),
            })
        }
        Err(err) => {
            return HttpResponse::InternalServerError().json(ResponseWrapper::<()> {
                message: err.to_string(),
                data: None,
            });
        }
    }
}

#[post("")]
async fn create_cat(
    state: Data<AppState>,
    Auth(user): Auth,
    cat_payload: Json<CreateCatRequest>,
) -> impl Responder {
    match cat_payload.validate() {
        Ok(_) => {
            let cat = CreateCatPayload {
                user_id: user.id,
                name: cat_payload.name.clone(),
                race: cat_payload.race.clone(),
                sex: cat_payload.sex.clone(),
                age_in_month: cat_payload.age_in_month.clone(),
                description: cat_payload.description.clone(),
                img_urls: cat_payload.img_urls.clone(),
            };

            match insert_cat(&state.db, cat).await {
                Ok(cat) => HttpResponse::Created().json(ResponseWrapper::<CreateCatResponse> {
                    message: "Cat created successfully".to_string(),
                    data: Some(CreateCatResponse {
                        id: cat.id,
                        created_at: cat.created_at,
                    }),
                }),
                Err(err) => {
                    return HttpResponse::InternalServerError().json(ResponseWrapper::<()> {
                        message: err.to_string(),
                        data: None,
                    });
                }
            }
        }

        Err(err) => HttpResponse::BadRequest().json(ResponseWrapper::<()> {
            message: err.to_string(),
            data: None,
        }),
    }
}

#[put("/{id}")]
async fn modify_cat(
    state: Data<AppState>,
    Auth(user): Auth,
    id: web::Path<i32>,
    cat_payload: Json<CreateCatRequest>,
) -> impl Responder {
    match cat_payload.validate() {
        Ok(_) => {
            let update_cat_payload = CreateCatPayload {
                user_id: user.id,
                name: cat_payload.name.clone(),
                race: cat_payload.race.clone(),
                sex: cat_payload.sex.clone(),
                age_in_month: cat_payload.age_in_month.clone(),
                description: cat_payload.description.clone(),
                img_urls: cat_payload.img_urls.clone(),
            };

            match find_one_cat(&state.db, *id).await {
                Ok(_) => match update_cat(&state.db, id.into_inner(), update_cat_payload).await {
                    Ok(cat) => HttpResponse::Ok().json(ResponseWrapper::<CreateCatResponse> {
                        message: "Cat updated successfully".to_string(),
                        data: Some(CreateCatResponse {
                            id: cat.id,
                            created_at: cat.created_at,
                        }),
                    }),
                    Err(err) => {
                        return HttpResponse::InternalServerError().json(ResponseWrapper::<()> {
                            message: err.to_string(),
                            data: None,
                        });
                    }
                },
                Err(err) => {
                    return HttpResponse::NotFound().json(ResponseWrapper::<()> {
                        message: err.to_string(),
                        data: None,
                    });
                }
            }
        }
        Err(err) => {
            return HttpResponse::BadRequest().json(ResponseWrapper::<()> {
                message: err.to_string(),
                data: None,
            });
        }
    }
}

#[delete("/{id}")]
async fn remove_cat(state: Data<AppState>, id: web::Path<i32>) -> impl Responder {
    match find_one_cat(&state.db, *id).await {
        Ok(_) => match crate::repositories::cat::delete_cat(&state.db, id.into_inner()).await {
            Ok(_) => HttpResponse::Ok().json(ResponseWrapper::<()> {
                message: "Cat deleted successfully".to_string(),
                data: None,
            }),
            Err(err) => {
                return HttpResponse::InternalServerError().json(ResponseWrapper::<()> {
                    message: err.to_string(),
                    data: None,
                });
            }
        },
        Err(err) => {
            return HttpResponse::NotFound().json(ResponseWrapper::<()> {
                message: err.to_string(),
                data: None,
            });
        }
    }
}
