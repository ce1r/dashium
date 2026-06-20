// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct BlockUserParams<T1: crate::StringSql> {
    pub user_id: i32,
    pub target_id: i32,
    pub gjp2: T1,
}
#[derive(Debug)]
pub struct UnblockUserParams<T1: crate::StringSql> {
    pub user_id: i32,
    pub target_id: i32,
    pub gjp2: T1,
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct BlockUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn block_user() -> BlockUserStmt {
    BlockUserStmt(
        "INSERT INTO blocks ( user_id, target_id ) SELECT $1, $2 WHERE EXISTS ( SELECT 1 FROM users WHERE id = $1 AND gjp2 = $3 ) AND NOT EXISTS ( SELECT 1 FROM blocks WHERE user_id = $1 AND target_id = $2 )",
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
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        user_id: &'a i32,
        target_id: &'a i32,
        gjp2: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[user_id, target_id, gjp2]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        BlockUserParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for BlockUserStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a BlockUserParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.user_id, &params.target_id, &params.gjp2))
    }
}
pub struct UnblockUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn unblock_user() -> UnblockUserStmt {
    UnblockUserStmt(
        "DELETE FROM blocks USING users WHERE blocks.user_id = users.id AND blocks.user_id = $1 AND blocks.target_id = $2 AND users.gjp2 = $3",
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
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        user_id: &'a i32,
        target_id: &'a i32,
        gjp2: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[user_id, target_id, gjp2]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UnblockUserParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UnblockUserStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UnblockUserParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.user_id, &params.target_id, &params.gjp2))
    }
}
