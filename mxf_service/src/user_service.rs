use chrono::Duration;
use lazy_static::lazy_static;
use mini_moka::sync::Cache;
use mxf_entity::{LoginData, MXFError, RegisterData, UserColumn, UserEntity, UserModel};
use rsa::pkcs8::{EncodePublicKey, LineEnding};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use sea_orm::*;
use std;

lazy_static! {
    /// Time before token expires (aka exp claim)
    static ref TOKEN_EXPIRATION: Duration = Duration::hours(24);
}

// #[derive(Clone)]
// struct TokenPair {
//     token_cache: Cache<u8, (RsaPublicKey, RsaPrivateKey)>,
// }

// impl TokenPair {
//     pub fn new() -> Self {
//         let token_cache = Cache::builder()
//             .time_to_live(std::time::Duration::from_secs(24 * 60 * 60))
//             .build();

//         return TokenPair { token_cache };
//     }

//     fn validate(&self) {
//         if !self.token_cache.contains_key(&1u8) {
//             let mut rng = rand::thread_rng();
//             let bits = 2048;
//             let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
//             let pub_key = RsaPublicKey::from(&priv_key);
//             let data = b"super";
//             let sup = pub_key
//                 .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])
//                 .unwrap_or_default();
//             println!("super: {:#?}", sup);
//             println!(
//                 "super: {}",
//                 String::from_utf8(sup).unwrap_or(String::from("from utf8 error"))
//             );
//             self.token_cache.insert(1u8, (pub_key, priv_key));
//         }
//     }

//     pub fn public_key(&self) -> Result<RsaPublicKey, MXFError> {
//         self.validate();
//         self.token_cache
//             .get(&1u8)
//             .map(|(pu, _pr)| pu)
//             .ok_or(MXFError::CacheError(None))
//     }

//     pub fn private_key(&self) -> Result<RsaPrivateKey, MXFError> {
//         self.validate();
//         self.token_cache
//             .get(&1u8)
//             .map(|(_pu, pr)| pr)
//             .ok_or(MXFError::CacheError(None))
//     }
// }

pub struct UserService {
    uno_cache: Cache<String, u32>,
    // token: TokenPair,
}

impl UserService {
    pub fn init() -> Self {
        let uno_cache = Cache::builder()
            .time_to_live(std::time::Duration::from_secs(24 * 60 * 60))
            .build();

        return UserService {
            uno_cache,
            // token: TokenPair::new(),
        };
    }

    async fn get_user_by_name(
        &self,
        db: &DbConn,
        username: &'_ str,
    ) -> Result<UserModel, MXFError> {
        if !self.uno_cache.contains_key(&String::from(username)) {
            let user = UserEntity::find()
                .column(UserColumn::Uname)
                .filter(UserColumn::Uname.eq(username))
                .one(db)
                .await?
                .ok_or(MXFError::UserNotFound(username.into()))?;
            self.uno_cache.insert(String::from(username), user.uno);
        }

        let uno = self
            .uno_cache
            .get(&String::from(username))
            .ok_or(MXFError::CacheError(None))?;

        UserEntity::find_by_id(uno)
            .one(db)
            .await?
            .ok_or(MXFError::UserNotFound(username.into()))
    }

    pub async fn login(
        &self,
        db: &DbConn,
        login_data: &LoginData<'_>,
    ) -> Result<UserModel, MXFError> {
        let user = self.get_user_by_name(db, login_data.username).await?;
        login_data.validate(user /* , self.token.private_key()? */)
    }

    pub async fn register(
        &self,
        db: &DbConn,
        register_data: &RegisterData<'_>,
    ) -> Result<UserModel, MXFError> {
        match self.get_user_by_name(db, register_data.username).await {
            Ok(_) => return Err(MXFError::UserAlreadyExists(register_data.username.into())),
            Err(MXFError::UserNotFound(_)) => {
                let uno = UserEntity::find()
                    .column(UserColumn::Uno)
                    .order_by_desc(UserColumn::Uno)
                    .one(db)
                    .await?
                    .map(|u| u.uno + 1u32)
                    .unwrap_or(1u32);
                println!("!!!!{:?}", uno);
                match register_data.into_active_model(uno) {
                    Ok(am) => am.insert(db).await.map_err(|e| e.into()),
                    Err(e) => return Err(e),
                }
            }
            Err(e) => return Err(e),
        }
    }

    // pub async fn get_public_key(&self) -> Result<String, MXFError> {
    //     self.token
    //         .public_key()?
    //         .to_public_key_pem(LineEnding::LF)
    //         .map_err(|e| MXFError::from(e))
    // }
}
