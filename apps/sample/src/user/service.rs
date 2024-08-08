use crate::user::repository::UserRepository;
use crate::user::model::{Model as User};
use sea_orm::{DbConn, DbErr};

pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(pool: DbConn) -> Self {
        Self {
            user_repository: UserRepository::new(pool),
        }
    }

    pub async fn get_user_info(&self, id: i32) -> Result<Option<User>, DbErr> {
        self.user_repository.find_by_id(id).await
    }

    pub async fn list_users(&self) -> Result<Vec<User>, DbErr> {
        self.user_repository.list_users().await
    }
}
