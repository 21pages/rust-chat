use crate::internal::model::user::User;
use sqlx::{self, MySqlPool};
use std::sync::Arc;
use tracing::trace;

pub async fn login(
    user: &mut User,
    pool: Arc<MySqlPool>,
) -> Result<(), Box<dyn std::error::Error>> {
    *user = sqlx::query_as::<_, User>("select * from users where username=? and password=?")
        .bind(&user.username)
        .bind(&user.password)
        .fetch_one(&*pool)
        .await?;
    trace!("{:?}", user);
    Ok(())
}
