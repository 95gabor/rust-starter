use model::{Model, Entity as User};
use sea_orm::{DbConn, DbErr, EntityTrait};

use crate::user::model;

pub struct UserRepository {
    pool: DbConn,
}

impl UserRepository {
    pub fn new(pool: DbConn) -> Self {
        Self { pool }
    }

    pub async fn list_users(&self) -> Result<Vec<Model>, DbErr> {
        User::find().all(&self.pool).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {
        User::find_by_id(id).one(&self.pool).await
    }
}
