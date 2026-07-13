// This file was generated with `cornucopia`. Do not modify.

#[derive(Clone, Copy, Debug)]
pub struct BlockUserParams {
    pub user_id: i32,
    pub target_id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct UnblockUserParams {
    pub user_id: i32,
    pub target_id: i32,
}
#[derive(Debug)]
pub struct CreateMessageParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub user_id: i32,
    pub target_id: i32,
    pub subject: T1,
    pub body: T2,
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct BlockUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn block_user() -> BlockUserStmt {
    BlockUserStmt(
        "INSERT INTO blocks ( user_id, target_id ) SELECT $1, $2 WHERE EXISTS ( SELECT 1 FROM users WHERE id = $1 )",
        None,
    )
}
impl BlockUserStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        user_id: &'a i32,
        target_id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[user_id, target_id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        BlockUserParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for BlockUserStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a BlockUserParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.user_id, &params.target_id))
    }
}
pub struct UnblockUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn unblock_user() -> UnblockUserStmt {
    UnblockUserStmt(
        "DELETE FROM blocks WHERE user_id = $1 AND target_id = $2",
        None,
    )
}
impl UnblockUserStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        user_id: &'a i32,
        target_id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[user_id, target_id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UnblockUserParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UnblockUserStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UnblockUserParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.user_id, &params.target_id))
    }
}
pub struct CreateMessageStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_message() -> CreateMessageStmt {
    CreateMessageStmt(
        "INSERT INTO messages ( user_id, target_id, subject, body ) SELECT $1, $2, $3, $4 FROM users WHERE id = $1",
        None,
    )
}
impl CreateMessageStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        user_id: &'a i32,
        target_id: &'a i32,
        subject: &'a T1,
        body: &'a T2,
    ) -> Result<u64, tokio_postgres::Error> {
        client
            .execute(self.0, &[user_id, target_id, subject, body])
            .await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        CreateMessageParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for CreateMessageStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a CreateMessageParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.user_id,
            &params.target_id,
            &params.subject,
            &params.body,
        ))
    }
}
