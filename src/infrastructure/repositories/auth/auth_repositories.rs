use crate::infrastructure::db::databases::Databases;
use crate::infrastructure::repository_impl::auth::repository::AuthRepositoryImpl;
use crate::infrastructure::repository_impl::auth::refresh_token_repository_impl::RefreshTokenRepositoryImpl;

pub struct AuthRepositories {
    pub auth: AuthRepositoryImpl,
    pub refresh_token: RefreshTokenRepositoryImpl,
}

impl AuthRepositories {
    pub fn new(dbs: &Databases) -> Self {
        Self {
            auth: AuthRepositoryImpl::new(dbs.mysql.clone()),
            refresh_token: RefreshTokenRepositoryImpl::new(dbs.mysql.clone()),
        }
    }
}
