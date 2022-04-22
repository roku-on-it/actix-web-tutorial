use diesel::result::DatabaseErrorKind::ForeignKeyViolation;
use diesel::result::Error;
use diesel::{insert_into, QueryDsl, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::core::exception::Exception;
use crate::schema::posts;
use crate::schema::posts::dsl::*;
#[allow(unused_imports)]
use crate::user::model::User;
use crate::Pool;

#[derive(Identifiable, Associations, Queryable, Debug, Serialize, Deserialize)]
#[diesel(belongs_to(User))]
pub struct Post {
  pub id: i32,
  pub content: String,
  pub title: String,
  pub user_id: i32,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
  pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct CreatePost {
  pub content: String,
  pub title: String,
  pub user_id: i32,
}

impl Post {
  pub fn find_one(post_id: i32, pool: &Pool) -> Result<Post, Exception> {
    let conn = pool.get().unwrap();
    let post: QueryResult<Post> = posts.find(post_id).first(&conn);

    match post {
      Ok(post) => Ok(post),
      Err(why) => match why {
        Error::NotFound => Err(Exception::NotFound),
        _ => Err(Exception::InternalServerError),
      },
    }
  }

  pub fn find(pool: &Pool) -> Result<(Vec<Post>, i64), diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items: Vec<Post> = posts.load(&conn).unwrap();
    let count: i64 = posts.count().first(&conn).unwrap();

    Ok((items, count))
  }

  pub fn save(create_post_input: CreatePost, pool: &Pool) -> Result<Post, Exception> {
    let conn = pool.get().unwrap();
    let post = CreatePost {
      ..create_post_input
    };
    let query_result: QueryResult<Post> = insert_into(posts).values(&post).get_result(&conn);

    match query_result {
      Ok(saved_post) => Ok(saved_post),
      Err(why) => match why {
        Error::DatabaseError(db_error, _) => match db_error {
          ForeignKeyViolation => Err(Exception::NotFound),
          _ => Err(Exception::InternalServerError),
        },
        _ => Err(Exception::InternalServerError),
      },
    }
  }
}
