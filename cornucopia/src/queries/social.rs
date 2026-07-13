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
#[derive(Clone, Copy, Debug)]
pub struct GetMessagesParams {
    pub target_id: i32,
    pub offset: i64,
}
#[derive(Clone, Copy, Debug)]
pub struct GetSentMessagesParams {
    pub user_id: i32,
    pub offset: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub id: i32,
    pub user_id: i32,
    pub target_id: i32,
    pub subject: String,
    pub body: String,
    pub is_read: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub username: String,
}
pub struct MessageBorrowed<'a> {
    pub id: i32,
    pub user_id: i32,
    pub target_id: i32,
    pub subject: &'a str,
    pub body: &'a str,
    pub is_read: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub username: &'a str,
}
impl<'a> From<MessageBorrowed<'a>> for Message {
    fn from(
        MessageBorrowed {
            id,
            user_id,
            target_id,
            subject,
            body,
            is_read,
            created_at,
            username,
        }: MessageBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            user_id,
            target_id,
            subject: subject.into(),
            body: body.into(),
            is_read,
            created_at,
            username: username.into(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct MessageQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<MessageBorrowed, tokio_postgres::Error>,
    mapper: fn(MessageBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> MessageQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(MessageBorrowed) -> R) -> MessageQuery<'c, 'a, 's, C, R, N> {
        MessageQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
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
pub struct GetMessagesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_messages() -> GetMessagesStmt {
    GetMessagesStmt(
        "SELECT m.*, u.username FROM messages m JOIN users u ON m.user_id = u.id WHERE m.target_id = $1 ORDER BY m.created_at DESC LIMIT 10 OFFSET $2",
        None,
    )
}
impl GetMessagesStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        target_id: &'a i32,
        offset: &'a i64,
    ) -> MessageQuery<'c, 'a, 's, C, Message, 2> {
        MessageQuery {
            client,
            params: [target_id, offset],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<MessageBorrowed, tokio_postgres::Error> {
                    Ok(MessageBorrowed {
                        id: row.try_get(0)?,
                        user_id: row.try_get(1)?,
                        target_id: row.try_get(2)?,
                        subject: row.try_get(3)?,
                        body: row.try_get(4)?,
                        is_read: row.try_get(5)?,
                        created_at: row.try_get(6)?,
                        username: row.try_get(7)?,
                    })
                },
            mapper: |it| Message::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetMessagesParams,
        MessageQuery<'c, 'a, 's, C, Message, 2>,
        C,
    > for GetMessagesStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetMessagesParams,
    ) -> MessageQuery<'c, 'a, 's, C, Message, 2> {
        self.bind(client, &params.target_id, &params.offset)
    }
}
pub struct GetSentMessagesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_sent_messages() -> GetSentMessagesStmt {
    GetSentMessagesStmt(
        "SELECT m.*, u.username FROM messages m JOIN users u ON m.target_id = u.id WHERE m.user_id = $1 ORDER BY m.created_at DESC LIMIT 10 OFFSET $2",
        None,
    )
}
impl GetSentMessagesStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        user_id: &'a i32,
        offset: &'a i64,
    ) -> MessageQuery<'c, 'a, 's, C, Message, 2> {
        MessageQuery {
            client,
            params: [user_id, offset],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<MessageBorrowed, tokio_postgres::Error> {
                    Ok(MessageBorrowed {
                        id: row.try_get(0)?,
                        user_id: row.try_get(1)?,
                        target_id: row.try_get(2)?,
                        subject: row.try_get(3)?,
                        body: row.try_get(4)?,
                        is_read: row.try_get(5)?,
                        created_at: row.try_get(6)?,
                        username: row.try_get(7)?,
                    })
                },
            mapper: |it| Message::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetSentMessagesParams,
        MessageQuery<'c, 'a, 's, C, Message, 2>,
        C,
    > for GetSentMessagesStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetSentMessagesParams,
    ) -> MessageQuery<'c, 'a, 's, C, Message, 2> {
        self.bind(client, &params.user_id, &params.offset)
    }
}
