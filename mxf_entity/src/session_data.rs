// use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

use crate::{user::UserType, MXFError, UserActiveModel, UserModel};

#[derive(Serialize, Deserialize)]
pub struct LoginData<'r> {
    pub username: &'r str,
    pub(crate) password: &'r str,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterData<'r> {
    pub username: &'r str,
    pub(crate) password: &'r str,
    pub(crate) pno: &'r str,
    pub(crate) email: &'r str,
    pub(crate) usertype: &'r str,
}

impl LoginData<'_> {
    pub fn validate(
        &self,
        user: UserModel,
        /* priv_key: RsaPrivateKey, */
    ) -> Result<UserModel, MXFError> {
        if self.password == user.upass {
            Ok(user)
        } else {
            Err(MXFError::WrongPassword(()))
        }
    }
}

impl RegisterData<'_> {
    pub fn into_active_model(&self, uno: u32) -> Result<UserActiveModel, MXFError> {
        let utype = match self.usertype {
            "admin" => UserType::Admin,
            "employee" => UserType::Employee,
            "tenant" => UserType::User,
            "resident" => UserType::User,
            _ => return Err(MXFError::InvalidUserType(self.usertype.to_string())),
        };
        Ok(UserActiveModel {
            uno: Set(uno),
            uname: Set(self.username.to_string()),
            upass: Set(self.password.to_string()),
            uphone: Set(self.pno.to_string()),
            uemail: Set(self.email.to_string()),
            utype: Set(utype),
        })
    }
}
