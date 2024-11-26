use crate::{
    entities::{
        user::{CreateUser, FilterUser, LoginUser, UserResponse},
        ResponseWrapper,
    },
    helpers::{
        jwt::get_jwt,
        passwords::{hash_password, verify_password},
    },
    repositories::user::{find_one_user, insert_user},
    AppState,
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use validator::Validate;

#[post("/register")]
async fn register_user(state: Data<AppState>, user: Json<CreateUser>) -> impl Responder {
    match user.validate() {
        Ok(_) => {
            let hashed_password = match hash_password(&user.password) {
                Ok(password) => password,
                Err(err) => {
                    log::error!("Password hashing error: {}", err);
                    return HttpResponse::InternalServerError().json(ResponseWrapper::<()> {
                        message: err.to_string(),
                        data: None,
                    });
                }
            };

            let user = CreateUser {
                name: user.name.clone(),
                email: user.email.clone(),
                password: hashed_password,
            };

            match insert_user(&state.db, user).await {
                Ok(user) => {
                    let token = match get_jwt(user.email.clone(), user.id) {
                        Ok(token) => token,
                        Err(err) => {
                            log::error!("JWT generation error: {}", err);
                            return HttpResponse::InternalServerError().json(
                                ResponseWrapper::<()> {
                                    message: err,
                                    data: None,
                                },
                            );
                        }
                    };

                    HttpResponse::Created().json(ResponseWrapper::<UserResponse> {
                        message: "User registered successfully".to_string(),
                        data: Some(UserResponse {
                            name: user.name,
                            email: user.email,
                            access_token: token,
                        }),
                    })
                }
                Err(err) => {
                    log::error!("Database insertion error: {}", err);
                    HttpResponse::InternalServerError().json(ResponseWrapper::<()> {
                        message: err.to_string(),
                        data: None,
                    })
                }
            }
        }
        Err(err) => {
            log::error!("User validation error: {:?}", err);
            HttpResponse::BadRequest().json(ResponseWrapper::<()> {
                message: err
                    .field_errors()
                    .iter()
                    .map(|(field, errors)| {
                        format!(
                            "Field: {}, Errors: {}",
                            field,
                            errors
                                .iter()
                                .map(|e| e.message.as_deref().unwrap_or(""))
                                .collect::<Vec<&str>>()
                                .join(", ")
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", "),
                data: None,
            })
        }
    }
}

#[post("/login")]
async fn login_user(state: Data<AppState>, user_payload: Json<LoginUser>) -> impl Responder {
    match user_payload.validate() {
        Ok(_) => {
            let user_filter = FilterUser {
                id: Some(0),
                name: None,
                email: Some(user_payload.email.clone()),
            };

            match find_one_user(&state.db, user_filter).await {
                Ok(user) => {
                    match verify_password(&user_payload.password, &user.password) {
                        Ok(_) => {}
                        Err(err) => {
                            return HttpResponse::BadRequest().json(ResponseWrapper::<()> {
                                message: err.to_string(),
                                data: None,
                            });
                        }
                    }

                    let token = match get_jwt(user.email.clone(), user.id) {
                        Ok(token) => token,
                        Err(err) => {
                            return HttpResponse::InternalServerError().json(
                                ResponseWrapper::<()> {
                                    message: err,
                                    data: None,
                                },
                            );
                        }
                    };

                    HttpResponse::Ok().json(ResponseWrapper::<UserResponse> {
                        message: "User logged in successfully".to_string(),
                        data: Some(UserResponse {
                            name: user.name,
                            email: user.email,
                            access_token: token,
                        }),
                    })
                }
                Err(err) => HttpResponse::NotFound().json(ResponseWrapper::<()> {
                    message: err.to_string(),
                    data: None,
                }),
            }
        }
        Err(err) => HttpResponse::BadRequest().json(ResponseWrapper::<()> {
            message: err
                .field_errors()
                .iter()
                .map(|(field, errors)| {
                    format!(
                        "Field: {}, Errors: {}",
                        field,
                        errors
                            .iter()
                            .map(|e| e.message.as_deref().unwrap_or(""))
                            .collect::<Vec<&str>>()
                            .join(", ")
                    )
                })
                .collect::<Vec<String>>()
                .join(", "),
            data: None,
        }),
    }
}
