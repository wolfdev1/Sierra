use mongodb::bson::{doc, Document};
use salvo::{prelude::*, http::StatusCode};
use serde::{Serialize, Deserialize};
use salvo::macros::Extractible;
use crate::db;

#[handler]
pub async fn get_user(res: &mut Response, req: &mut Request) {

    println!("GET /user");
    
    #[derive(Serialize, Debug, Deserialize)]
    struct User<T> {
        name: T,
        email: T,
        nickname: T,
        age: u8,
    }

    #[derive(Serialize, Debug)]
    struct Msg {
        code: u32,
        msg: String,
    }

    if req.queries().contains_key("user") {
        
        let db = db::get_database().await.unwrap();
        let collection = db.collection::<Document>("users");

        let filter = doc! {"nickname": req.queries().get("user").unwrap()};
        let us: Option<Document> = collection.find_one( filter, None).await.unwrap();
        
        if let Some(doc) = us {
            let user = User {
                name: doc.get("name").unwrap(),
                email: doc.get("email").unwrap(),
                nickname: doc.get("nickname").unwrap(),
                age: doc.get("age").unwrap().to_string().parse::<u8>().unwrap(),
            };

            res.set_status_code(StatusCode::OK);
            res.render(Json(user));
        } else {
            let msg = Msg {
                code: 404 as u32,
                msg: "User not found.".to_string(),
            };
            res.set_status_code(StatusCode::NOT_FOUND);
            res.render(Json(msg));
        }
        res.set_status_code(StatusCode::OK);
    
    } else {
        let msg = Msg {
            code: 404 as u32,
            msg: "Please specify a valid user. Example: <addr>/user?user=johndoe".to_string(),
        };
        res.set_status_code(StatusCode::BAD_REQUEST);
        res.render(Json(msg));
    }
}    

#[handler]
pub async fn post_user(res: &mut Response, req: &mut Request) {
    println!("POST /user");

    #[derive(Serialize, Debug)]
    struct Msg {
        code: u32,
        msg: String,
    }

    #[derive(Serialize, Debug, Deserialize, Extractible)]
    #[extract(
        default_source(from = "body")
    )]
    struct User {
        name: String,
        email: String,
        nickname: String,
        age: u8,
    }

        let db = db::get_database().await.unwrap();
        let collection = db.collection::<Document>("users");

        let u = req.parse_body().await;

        if u.is_err() {

            let msg = Msg {
                code: 400 as u32,
                msg: "Could not obtain a value of those requested in the request body".to_string(),
            };
            res.set_status_code(StatusCode::BAD_REQUEST);
            res.render(Json(msg));
       
        } else {

            let user: User = u.unwrap();
     
        let filter = doc! {"nickname": user.nickname.clone()};
        let result: Option<Document> = collection.find_one( filter, None).await.unwrap();

        if let Some(_doc) = result {
            let msg = Msg {
                code: 409 as u32,
                msg: "User already exists.".to_string(),
            };
            res.set_status_code(StatusCode::CONFLICT);
            res.render(Json(msg));
        } else {
            let filter_mail = doc! {"email": user.email.clone()};
            let result2: Option<Document> = collection.find_one( filter_mail, None).await.unwrap();

            if let Some(_doc) = result2 {
                let msg = Msg {
                    code: 409 as u32,
                    msg: "Email already exists.".to_string(),
                };
                res.set_status_code(StatusCode::CONFLICT);
                res.render(Json(msg));

            } else {
                let user = doc! {
                    "name": user.name,
                    "email": user.email,
                    "nickname": user.nickname,
                    "age": user.age as i64
                };
                collection.insert_one(user, None).await.unwrap();
                let msg = Msg {
                    code: 201 as u32,
                    msg: "User created.".to_string(),
                };
                res.set_status_code(StatusCode::CREATED);
                res.render(Json(msg));
            }
        }
    }
    res.set_status_code(StatusCode::OK);
}

#[handler]
pub async fn delete_user(res: &mut Response, req: &mut Request) {
    println!("DELETE /user");

    #[derive(Serialize, Debug)]
    struct Msg {
        code: u32,
        msg: String,
    }

    if req.queries().contains_key("user") {
        
        let db = db::get_database().await.unwrap();
        let collection = db.collection::<Document>("users");
    
        let filter = doc! {"nickname": req.queries().get("user").unwrap()};
        let us: Option<Document> = collection.find_one( filter.clone(), None).await.unwrap();
            
        if let Some(_doc) = us {
            collection.delete_one(filter, None).await.expect("Failed to delete user");
    
            res.set_status_code(StatusCode::OK);
            res.render(Json(Msg {code: 200 as u32, msg: "User deleted.".to_string()} ));
        } else {
            let msg = Msg {
                code: 404 as u32,
                msg: "User not found.".to_string(),
            };
            res.set_status_code(StatusCode::NOT_FOUND);
            res.render(Json(msg));
        }
    } else {
        let msg = Msg {
            code: 404 as u32,
            msg: "Please specify a valid user. Example: <addr>/user?user=johndoe".to_string(),
        };
        res.set_status_code(StatusCode::BAD_REQUEST);
        res.render(Json(msg));
    }
    

}