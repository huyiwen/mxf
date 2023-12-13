use mini_moka::sync::Cache;
use mxf_entity::{MXFError, UserColumn, UserEntity, UserModel};
use sea_orm::*;
use std::time::Duration;

pub struct UserService {
    uno_cache: Cache<String, u32>,
}

impl UserService {
    pub fn init() -> Self {
        let uno_cache = Cache::builder()
            .time_to_live(Duration::from_secs(24 * 60 * 60))
            .build();
        return UserService { uno_cache };
    }

    pub async fn login(
        &self,
        db: &DbConn,
        username: &str,
        password: &str,
    ) -> Result<UserModel, MXFError> {
        if !self.uno_cache.contains_key(&String::from(username)) {
            let user = UserEntity::find()
                .filter(UserColumn::Uname.eq(username))
                .one(db)
                .await
                .map_err(|_| MXFError::user_not_found())?
                .ok_or(MXFError::user_not_found())?;
            self.uno_cache.insert(username.into(), user.uno);
        }

        let uno = self
            .uno_cache
            .get(&String::from(username))
            .ok_or(MXFError::cache_error())?;
        let user = UserEntity::find_by_id(uno)
            .one(db)
            .await
            .map_err(|_| MXFError::user_not_found())?
            .ok_or(MXFError::user_not_found())?;

        if user.upass == password {
            Ok(user)
        } else {
            println!(
                "username: {}, password: {}, user.upass: {}",
                username, password, user.upass
            );
            Err(MXFError::wrong_password())
        }
    }
}
