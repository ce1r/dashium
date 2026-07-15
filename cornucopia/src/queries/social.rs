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
#[derive(Clone, Copy, Debug)]
pub struct DownloadMessageParams {
    pub message_id: i32,
    pub target_id: i32,
}
#[derive(Debug)]
pub struct DeleteMessagesParams<T1: crate::ArraySql<Item = i32>> {
    pub user_id: i32,
    pub message_ids: T1,
}
#[derive(Debug)]
pub struct DeleteSentMessagesParams<T1: crate::ArraySql<Item = i32>> {
    pub user_id: i32,
    pub message_ids: T1,
}
#[derive(Debug)]
pub struct CreateFriendRequestParams<T1: crate::StringSql> {
    pub user_id: i32,
    pub target_id: i32,
    pub body: T1,
}
#[derive(Clone, Copy, Debug)]
pub struct GetFriendRequestsParams {
    pub user_id: i32,
    pub offset: i64,
}
#[derive(Clone, Copy, Debug)]
pub struct GetSentFriendRequestsParams {
    pub user_id: i32,
    pub offset: i64,
}
#[derive(Clone, Copy, Debug)]
pub struct AcceptFriendRequestParams {
    pub user_id: i32,
    pub target_id: i32,
}
#[derive(Debug)]
pub struct DeleteFriendRequestsParams<T1: crate::ArraySql<Item = i32>> {
    pub user_id: i32,
    pub target_ids: T1,
}
#[derive(Clone, Copy, Debug)]
pub struct RemoveFriendParams {
    pub user_id: i32,
    pub target_id: i32,
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
#[derive(Debug, Clone, PartialEq)]
pub struct FriendRequest {
    pub id: i32,
    pub user_id: i32,
    pub target_id: i32,
    pub body: String,
    pub is_new: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub username: String,
    pub icon: i16,
    pub color1: i16,
    pub color2: i16,
    pub icon_type: i16,
    pub glow: i16,
}
pub struct FriendRequestBorrowed<'a> {
    pub id: i32,
    pub user_id: i32,
    pub target_id: i32,
    pub body: &'a str,
    pub is_new: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub username: &'a str,
    pub icon: i16,
    pub color1: i16,
    pub color2: i16,
    pub icon_type: i16,
    pub glow: i16,
}
impl<'a> From<FriendRequestBorrowed<'a>> for FriendRequest {
    fn from(
        FriendRequestBorrowed {
            id,
            user_id,
            target_id,
            body,
            is_new,
            created_at,
            username,
            icon,
            color1,
            color2,
            icon_type,
            glow,
        }: FriendRequestBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            user_id,
            target_id,
            body: body.into(),
            is_new,
            created_at,
            username: username.into(),
            icon,
            color1,
            color2,
            icon_type,
            glow,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
    pub save_data: String,
    pub is_activated: bool,
    pub mod_level: i16,
    pub udid: String,
    pub stars: i32,
    pub demons: i32,
    pub creator_points: i32,
    pub diamonds: i32,
    pub moons: i32,
    pub secret_coins: i32,
    pub user_coins: i32,
    pub cube: i16,
    pub ship: i16,
    pub ball: i16,
    pub ufo: i16,
    pub wave: i16,
    pub robot: i16,
    pub spider: i16,
    pub swing: i16,
    pub jetpack: i16,
    pub glow: i16,
    pub explosion: i16,
    pub icon: i16,
    pub icon_type: i16,
    pub color1: i16,
    pub color2: i16,
    pub color3: i16,
    pub message_setting: i16,
    pub friend_setting: i16,
    pub comment_setting: i16,
    pub youtube: String,
    pub twitter: String,
    pub twitch: String,
    pub discord: String,
    pub instagram: String,
    pub tiktok: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct UserBorrowed<'a> {
    pub id: i32,
    pub username: &'a str,
    pub email: &'a str,
    pub hash: &'a [u8],
    pub salt: &'a [u8],
    pub save_data: &'a str,
    pub is_activated: bool,
    pub mod_level: i16,
    pub udid: &'a str,
    pub stars: i32,
    pub demons: i32,
    pub creator_points: i32,
    pub diamonds: i32,
    pub moons: i32,
    pub secret_coins: i32,
    pub user_coins: i32,
    pub cube: i16,
    pub ship: i16,
    pub ball: i16,
    pub ufo: i16,
    pub wave: i16,
    pub robot: i16,
    pub spider: i16,
    pub swing: i16,
    pub jetpack: i16,
    pub glow: i16,
    pub explosion: i16,
    pub icon: i16,
    pub icon_type: i16,
    pub color1: i16,
    pub color2: i16,
    pub color3: i16,
    pub message_setting: i16,
    pub friend_setting: i16,
    pub comment_setting: i16,
    pub youtube: &'a str,
    pub twitter: &'a str,
    pub twitch: &'a str,
    pub discord: &'a str,
    pub instagram: &'a str,
    pub tiktok: &'a str,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<UserBorrowed<'a>> for User {
    fn from(
        UserBorrowed {
            id,
            username,
            email,
            hash,
            salt,
            save_data,
            is_activated,
            mod_level,
            udid,
            stars,
            demons,
            creator_points,
            diamonds,
            moons,
            secret_coins,
            user_coins,
            cube,
            ship,
            ball,
            ufo,
            wave,
            robot,
            spider,
            swing,
            jetpack,
            glow,
            explosion,
            icon,
            icon_type,
            color1,
            color2,
            color3,
            message_setting,
            friend_setting,
            comment_setting,
            youtube,
            twitter,
            twitch,
            discord,
            instagram,
            tiktok,
            created_at,
        }: UserBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            username: username.into(),
            email: email.into(),
            hash: hash.into(),
            salt: salt.into(),
            save_data: save_data.into(),
            is_activated,
            mod_level,
            udid: udid.into(),
            stars,
            demons,
            creator_points,
            diamonds,
            moons,
            secret_coins,
            user_coins,
            cube,
            ship,
            ball,
            ufo,
            wave,
            robot,
            spider,
            swing,
            jetpack,
            glow,
            explosion,
            icon,
            icon_type,
            color1,
            color2,
            color3,
            message_setting,
            friend_setting,
            comment_setting,
            youtube: youtube.into(),
            twitter: twitter.into(),
            twitch: twitch.into(),
            discord: discord.into(),
            instagram: instagram.into(),
            tiktok: tiktok.into(),
            created_at,
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
pub struct FriendRequestQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<FriendRequestBorrowed, tokio_postgres::Error>,
    mapper: fn(FriendRequestBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> FriendRequestQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(FriendRequestBorrowed) -> R,
    ) -> FriendRequestQuery<'c, 'a, 's, C, R, N> {
        FriendRequestQuery {
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
pub struct UserQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<UserBorrowed, tokio_postgres::Error>,
    mapper: fn(UserBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UserQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(UserBorrowed) -> R) -> UserQuery<'c, 'a, 's, C, R, N> {
        UserQuery {
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
pub struct DownloadMessageStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn download_message() -> DownloadMessageStmt {
    DownloadMessageStmt(
        "WITH updated AS ( UPDATE messages SET is_read = TRUE WHERE id = $1 AND target_id = $2 RETURNING * ) SELECT updated.*, u.username FROM updated JOIN users u ON updated.user_id = u.id",
        None,
    )
}
impl DownloadMessageStmt {
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
        message_id: &'a i32,
        target_id: &'a i32,
    ) -> MessageQuery<'c, 'a, 's, C, Message, 2> {
        MessageQuery {
            client,
            params: [message_id, target_id],
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
        DownloadMessageParams,
        MessageQuery<'c, 'a, 's, C, Message, 2>,
        C,
    > for DownloadMessageStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a DownloadMessageParams,
    ) -> MessageQuery<'c, 'a, 's, C, Message, 2> {
        self.bind(client, &params.message_id, &params.target_id)
    }
}
pub struct DeleteMessagesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_messages() -> DeleteMessagesStmt {
    DeleteMessagesStmt(
        "DELETE FROM messages WHERE target_id = $1 AND id = ANY($2)",
        None,
    )
}
impl DeleteMessagesStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::ArraySql<Item = i32>>(
        &'s self,
        client: &'c C,
        user_id: &'a i32,
        message_ids: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[user_id, message_ids]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::ArraySql<Item = i32>>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        DeleteMessagesParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for DeleteMessagesStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a DeleteMessagesParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.user_id, &params.message_ids))
    }
}
pub struct DeleteSentMessagesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_sent_messages() -> DeleteSentMessagesStmt {
    DeleteSentMessagesStmt(
        "DELETE FROM messages WHERE user_id = $1 AND id = ANY($2)",
        None,
    )
}
impl DeleteSentMessagesStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::ArraySql<Item = i32>>(
        &'s self,
        client: &'c C,
        user_id: &'a i32,
        message_ids: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[user_id, message_ids]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::ArraySql<Item = i32>>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        DeleteSentMessagesParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for DeleteSentMessagesStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a DeleteSentMessagesParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.user_id, &params.message_ids))
    }
}
pub struct CreateFriendRequestStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_friend_request() -> CreateFriendRequestStmt {
    CreateFriendRequestStmt(
        "INSERT INTO friend_requests ( user_id, target_id, body ) VALUES ( $1, $2, $3 )",
        None,
    )
}
impl CreateFriendRequestStmt {
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
        body: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[user_id, target_id, body]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        CreateFriendRequestParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for CreateFriendRequestStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a CreateFriendRequestParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.user_id, &params.target_id, &params.body))
    }
}
pub struct GetFriendRequestsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_friend_requests() -> GetFriendRequestsStmt {
    GetFriendRequestsStmt(
        "SELECT fr.id, fr.user_id, fr.target_id, fr.body, fr.is_new, fr.created_at, u.username, u.icon, u.color1, u.color2, u.icon_type, u.glow FROM friend_requests fr JOIN users u on u.id = fr.user_id WHERE fr.target_id = $1 ORDER BY fr.created_at DESC LIMIT 20 OFFSET $2",
        None,
    )
}
impl GetFriendRequestsStmt {
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
    ) -> FriendRequestQuery<'c, 'a, 's, C, FriendRequest, 2> {
        FriendRequestQuery {
            client,
            params: [user_id, offset],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<FriendRequestBorrowed, tokio_postgres::Error> {
                    Ok(FriendRequestBorrowed {
                        id: row.try_get(0)?,
                        user_id: row.try_get(1)?,
                        target_id: row.try_get(2)?,
                        body: row.try_get(3)?,
                        is_new: row.try_get(4)?,
                        created_at: row.try_get(5)?,
                        username: row.try_get(6)?,
                        icon: row.try_get(7)?,
                        color1: row.try_get(8)?,
                        color2: row.try_get(9)?,
                        icon_type: row.try_get(10)?,
                        glow: row.try_get(11)?,
                    })
                },
            mapper: |it| FriendRequest::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetFriendRequestsParams,
        FriendRequestQuery<'c, 'a, 's, C, FriendRequest, 2>,
        C,
    > for GetFriendRequestsStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetFriendRequestsParams,
    ) -> FriendRequestQuery<'c, 'a, 's, C, FriendRequest, 2> {
        self.bind(client, &params.user_id, &params.offset)
    }
}
pub struct GetSentFriendRequestsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_sent_friend_requests() -> GetSentFriendRequestsStmt {
    GetSentFriendRequestsStmt(
        "SELECT fr.id, fr.user_id, fr.target_id, fr.body, fr.is_new, fr.created_at, u.username, u.icon, u.color1, u.color2, u.icon_type, u.glow FROM friend_requests fr JOIN users u on u.id = fr.target_id WHERE fr.user_id = $1 ORDER BY fr.created_at DESC LIMIT 20 OFFSET $2",
        None,
    )
}
impl GetSentFriendRequestsStmt {
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
    ) -> FriendRequestQuery<'c, 'a, 's, C, FriendRequest, 2> {
        FriendRequestQuery {
            client,
            params: [user_id, offset],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<FriendRequestBorrowed, tokio_postgres::Error> {
                    Ok(FriendRequestBorrowed {
                        id: row.try_get(0)?,
                        user_id: row.try_get(1)?,
                        target_id: row.try_get(2)?,
                        body: row.try_get(3)?,
                        is_new: row.try_get(4)?,
                        created_at: row.try_get(5)?,
                        username: row.try_get(6)?,
                        icon: row.try_get(7)?,
                        color1: row.try_get(8)?,
                        color2: row.try_get(9)?,
                        icon_type: row.try_get(10)?,
                        glow: row.try_get(11)?,
                    })
                },
            mapper: |it| FriendRequest::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetSentFriendRequestsParams,
        FriendRequestQuery<'c, 'a, 's, C, FriendRequest, 2>,
        C,
    > for GetSentFriendRequestsStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetSentFriendRequestsParams,
    ) -> FriendRequestQuery<'c, 'a, 's, C, FriendRequest, 2> {
        self.bind(client, &params.user_id, &params.offset)
    }
}
pub struct ReadFriendRequestStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn read_friend_request() -> ReadFriendRequestStmt {
    ReadFriendRequestStmt(
        "UPDATE friend_requests SET is_new = FALSE WHERE id = $1",
        None,
    )
}
impl ReadFriendRequestStmt {
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
        request_id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[request_id]).await
    }
}
pub struct AcceptFriendRequestStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn accept_friend_request() -> AcceptFriendRequestStmt {
    AcceptFriendRequestStmt(
        "WITH _ AS ( DELETE FROM friend_requests WHERE user_id = $1 AND target_id = $2 ) INSERT INTO friendships ( user1, user2 ) VALUES ( $1, $2 )",
        None,
    )
}
impl AcceptFriendRequestStmt {
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
        AcceptFriendRequestParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for AcceptFriendRequestStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a AcceptFriendRequestParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.user_id, &params.target_id))
    }
}
pub struct GetFriendListStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_friend_list() -> GetFriendListStmt {
    GetFriendListStmt(
        "SELECT u.* FROM users u WHERE u.id IN ( SELECT user2 FROM friendships WHERE user1 = $1 UNION SELECT user1 FROM friendships WHERE user2 = $1 ) ORDER BY u.username ASC",
        None,
    )
}
impl GetFriendListStmt {
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
    ) -> UserQuery<'c, 'a, 's, C, User, 1> {
        UserQuery {
            client,
            params: [user_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<UserBorrowed, tokio_postgres::Error> {
                Ok(UserBorrowed {
                    id: row.try_get(0)?,
                    username: row.try_get(1)?,
                    email: row.try_get(2)?,
                    hash: row.try_get(3)?,
                    salt: row.try_get(4)?,
                    save_data: row.try_get(5)?,
                    is_activated: row.try_get(6)?,
                    mod_level: row.try_get(7)?,
                    udid: row.try_get(8)?,
                    stars: row.try_get(9)?,
                    demons: row.try_get(10)?,
                    creator_points: row.try_get(11)?,
                    diamonds: row.try_get(12)?,
                    moons: row.try_get(13)?,
                    secret_coins: row.try_get(14)?,
                    user_coins: row.try_get(15)?,
                    cube: row.try_get(16)?,
                    ship: row.try_get(17)?,
                    ball: row.try_get(18)?,
                    ufo: row.try_get(19)?,
                    wave: row.try_get(20)?,
                    robot: row.try_get(21)?,
                    spider: row.try_get(22)?,
                    swing: row.try_get(23)?,
                    jetpack: row.try_get(24)?,
                    glow: row.try_get(25)?,
                    explosion: row.try_get(26)?,
                    icon: row.try_get(27)?,
                    icon_type: row.try_get(28)?,
                    color1: row.try_get(29)?,
                    color2: row.try_get(30)?,
                    color3: row.try_get(31)?,
                    message_setting: row.try_get(32)?,
                    friend_setting: row.try_get(33)?,
                    comment_setting: row.try_get(34)?,
                    youtube: row.try_get(35)?,
                    twitter: row.try_get(36)?,
                    twitch: row.try_get(37)?,
                    discord: row.try_get(38)?,
                    instagram: row.try_get(39)?,
                    tiktok: row.try_get(40)?,
                    created_at: row.try_get(41)?,
                })
            },
            mapper: |it| User::from(it),
        }
    }
}
pub struct GetBlockedListStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_blocked_list() -> GetBlockedListStmt {
    GetBlockedListStmt(
        "SELECT u.* FROM users u WHERE u.id IN ( SELECT target_id FROM blocks WHERE user_id = $1 ) ORDER BY u.username ASC",
        None,
    )
}
impl GetBlockedListStmt {
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
    ) -> UserQuery<'c, 'a, 's, C, User, 1> {
        UserQuery {
            client,
            params: [user_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<UserBorrowed, tokio_postgres::Error> {
                Ok(UserBorrowed {
                    id: row.try_get(0)?,
                    username: row.try_get(1)?,
                    email: row.try_get(2)?,
                    hash: row.try_get(3)?,
                    salt: row.try_get(4)?,
                    save_data: row.try_get(5)?,
                    is_activated: row.try_get(6)?,
                    mod_level: row.try_get(7)?,
                    udid: row.try_get(8)?,
                    stars: row.try_get(9)?,
                    demons: row.try_get(10)?,
                    creator_points: row.try_get(11)?,
                    diamonds: row.try_get(12)?,
                    moons: row.try_get(13)?,
                    secret_coins: row.try_get(14)?,
                    user_coins: row.try_get(15)?,
                    cube: row.try_get(16)?,
                    ship: row.try_get(17)?,
                    ball: row.try_get(18)?,
                    ufo: row.try_get(19)?,
                    wave: row.try_get(20)?,
                    robot: row.try_get(21)?,
                    spider: row.try_get(22)?,
                    swing: row.try_get(23)?,
                    jetpack: row.try_get(24)?,
                    glow: row.try_get(25)?,
                    explosion: row.try_get(26)?,
                    icon: row.try_get(27)?,
                    icon_type: row.try_get(28)?,
                    color1: row.try_get(29)?,
                    color2: row.try_get(30)?,
                    color3: row.try_get(31)?,
                    message_setting: row.try_get(32)?,
                    friend_setting: row.try_get(33)?,
                    comment_setting: row.try_get(34)?,
                    youtube: row.try_get(35)?,
                    twitter: row.try_get(36)?,
                    twitch: row.try_get(37)?,
                    discord: row.try_get(38)?,
                    instagram: row.try_get(39)?,
                    tiktok: row.try_get(40)?,
                    created_at: row.try_get(41)?,
                })
            },
            mapper: |it| User::from(it),
        }
    }
}
pub struct DeleteFriendRequestsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_friend_requests() -> DeleteFriendRequestsStmt {
    DeleteFriendRequestsStmt(
        "DELETE FROM friend_requests WHERE user_id = $1 AND target_id = ANY($2)",
        None,
    )
}
impl DeleteFriendRequestsStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<'c, 'a, 's, C: GenericClient, T1: crate::ArraySql<Item = i32>>(
        &'s self,
        client: &'c C,
        user_id: &'a i32,
        target_ids: &'a T1,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[user_id, target_ids]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::ArraySql<Item = i32>>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        DeleteFriendRequestsParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for DeleteFriendRequestsStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a DeleteFriendRequestsParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.user_id, &params.target_ids))
    }
}
pub struct RemoveFriendStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn remove_friend() -> RemoveFriendStmt {
    RemoveFriendStmt(
        "DELETE FROM friendships WHERE (user1, user2) IN (($1, $2), ($2, $1))",
        None,
    )
}
impl RemoveFriendStmt {
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
        RemoveFriendParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for RemoveFriendStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a RemoveFriendParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.user_id, &params.target_id))
    }
}
