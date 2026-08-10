// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct CreateCommentParams<T1: crate::StringSql> {
    pub user_id: i32,
    pub level_id: i32,
    pub body: T1,
    pub percent: i16,
}
#[derive(Clone, Copy, Debug)]
pub struct GetCommentsByDateParams {
    pub level_id: i32,
    pub offset: i64,
}
#[derive(Clone, Copy, Debug)]
pub struct GetCommentsByLikesParams {
    pub level_id: i32,
    pub offset: i64,
}
#[derive(Clone, Copy, Debug)]
pub struct GetCommentHistoryParams {
    pub user_id: i32,
    pub offset: i64,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Comment {
    pub id: i32,
    pub level_id: i32,
    pub user_id: i32,
    pub body: String,
    pub likes: i32,
    pub is_spam: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub percent: i16,
    pub chat_color: String,
    pub username: String,
    pub role: crate::types::Role,
    pub color1: i16,
    pub color2: i16,
    pub color3: i16,
    pub icon: i16,
    pub icon_type: i16,
    pub glow: i16,
}
pub struct CommentBorrowed<'a> {
    pub id: i32,
    pub level_id: i32,
    pub user_id: i32,
    pub body: &'a str,
    pub likes: i32,
    pub is_spam: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub percent: i16,
    pub chat_color: &'a str,
    pub username: &'a str,
    pub role: crate::types::Role,
    pub color1: i16,
    pub color2: i16,
    pub color3: i16,
    pub icon: i16,
    pub icon_type: i16,
    pub glow: i16,
}
impl<'a> From<CommentBorrowed<'a>> for Comment {
    fn from(
        CommentBorrowed {
            id,
            level_id,
            user_id,
            body,
            likes,
            is_spam,
            created_at,
            percent,
            chat_color,
            username,
            role,
            color1,
            color2,
            color3,
            icon,
            icon_type,
            glow,
        }: CommentBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            level_id,
            user_id,
            body: body.into(),
            likes,
            is_spam,
            created_at,
            percent,
            chat_color: chat_color.into(),
            username: username.into(),
            role,
            color1,
            color2,
            color3,
            icon,
            icon_type,
            glow,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetCommentHistory {
    pub id: i32,
    pub level_id: i32,
    pub user_id: i32,
    pub body: String,
    pub likes: i32,
    pub is_spam: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub percent: i16,
    pub chat_color: String,
    pub username: String,
    pub role: crate::types::Role,
    pub color1: i16,
    pub color2: i16,
    pub color3: i16,
    pub icon: i16,
    pub icon_type: i16,
    pub glow: i16,
}
pub struct GetCommentHistoryBorrowed<'a> {
    pub id: i32,
    pub level_id: i32,
    pub user_id: i32,
    pub body: &'a str,
    pub likes: i32,
    pub is_spam: bool,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub percent: i16,
    pub chat_color: &'a str,
    pub username: &'a str,
    pub role: crate::types::Role,
    pub color1: i16,
    pub color2: i16,
    pub color3: i16,
    pub icon: i16,
    pub icon_type: i16,
    pub glow: i16,
}
impl<'a> From<GetCommentHistoryBorrowed<'a>> for GetCommentHistory {
    fn from(
        GetCommentHistoryBorrowed {
            id,
            level_id,
            user_id,
            body,
            likes,
            is_spam,
            created_at,
            percent,
            chat_color,
            username,
            role,
            color1,
            color2,
            color3,
            icon,
            icon_type,
            glow,
        }: GetCommentHistoryBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            level_id,
            user_id,
            body: body.into(),
            likes,
            is_spam,
            created_at,
            percent,
            chat_color: chat_color.into(),
            username: username.into(),
            role,
            color1,
            color2,
            color3,
            icon,
            icon_type,
            glow,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct I32Query<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<i32, tokio_postgres::Error>,
    mapper: fn(i32) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> I32Query<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(i32) -> R) -> I32Query<'c, 'a, 's, C, R, N> {
        I32Query {
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
pub struct CommentQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<CommentBorrowed, tokio_postgres::Error>,
    mapper: fn(CommentBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> CommentQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(CommentBorrowed) -> R) -> CommentQuery<'c, 'a, 's, C, R, N> {
        CommentQuery {
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
pub struct GetCommentHistoryQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetCommentHistoryBorrowed, tokio_postgres::Error>,
    mapper: fn(GetCommentHistoryBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetCommentHistoryQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetCommentHistoryBorrowed) -> R,
    ) -> GetCommentHistoryQuery<'c, 'a, 's, C, R, N> {
        GetCommentHistoryQuery {
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
pub struct CreateCommentStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_comment() -> CreateCommentStmt {
    CreateCommentStmt(
        "INSERT INTO comments ( user_id, level_id, body, percent ) VALUES ( $1, $2, $3, $4 ) RETURNING id",
        None,
    )
}
impl CreateCommentStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        user_id: &'a i32,
        level_id: &'a i32,
        body: &'a T1,
        percent: &'a i16,
    ) -> I32Query<'c, 'a, 's, C, i32, 4> {
        I32Query {
            client,
            params: [user_id, level_id, body, percent],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateCommentParams<T1>,
        I32Query<'c, 'a, 's, C, i32, 4>,
        C,
    > for CreateCommentStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateCommentParams<T1>,
    ) -> I32Query<'c, 'a, 's, C, i32, 4> {
        self.bind(
            client,
            &params.user_id,
            &params.level_id,
            &params.body,
            &params.percent,
        )
    }
}
pub struct GetCommentsByDateStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_comments_by_date() -> GetCommentsByDateStmt {
    GetCommentsByDateStmt(
        "SELECT * FROM comment_view WHERE level_id = $1 ORDER BY created_at DESC LIMIT 20 OFFSET $2",
        None,
    )
}
impl GetCommentsByDateStmt {
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
        level_id: &'a i32,
        offset: &'a i64,
    ) -> CommentQuery<'c, 'a, 's, C, Comment, 2> {
        CommentQuery {
            client,
            params: [level_id, offset],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<CommentBorrowed, tokio_postgres::Error> {
                    Ok(CommentBorrowed {
                        id: row.try_get(0)?,
                        level_id: row.try_get(1)?,
                        user_id: row.try_get(2)?,
                        body: row.try_get(3)?,
                        likes: row.try_get(4)?,
                        is_spam: row.try_get(5)?,
                        created_at: row.try_get(6)?,
                        percent: row.try_get(7)?,
                        chat_color: row.try_get(8)?,
                        username: row.try_get(9)?,
                        role: row.try_get(10)?,
                        color1: row.try_get(11)?,
                        color2: row.try_get(12)?,
                        color3: row.try_get(13)?,
                        icon: row.try_get(14)?,
                        icon_type: row.try_get(15)?,
                        glow: row.try_get(16)?,
                    })
                },
            mapper: |it| Comment::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetCommentsByDateParams,
        CommentQuery<'c, 'a, 's, C, Comment, 2>,
        C,
    > for GetCommentsByDateStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetCommentsByDateParams,
    ) -> CommentQuery<'c, 'a, 's, C, Comment, 2> {
        self.bind(client, &params.level_id, &params.offset)
    }
}
pub struct GetCommentsByLikesStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_comments_by_likes() -> GetCommentsByLikesStmt {
    GetCommentsByLikesStmt(
        "SELECT * FROM comment_view WHERE level_id = $1 ORDER BY likes DESC LIMIT 20 OFFSET $2",
        None,
    )
}
impl GetCommentsByLikesStmt {
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
        level_id: &'a i32,
        offset: &'a i64,
    ) -> CommentQuery<'c, 'a, 's, C, Comment, 2> {
        CommentQuery {
            client,
            params: [level_id, offset],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<CommentBorrowed, tokio_postgres::Error> {
                    Ok(CommentBorrowed {
                        id: row.try_get(0)?,
                        level_id: row.try_get(1)?,
                        user_id: row.try_get(2)?,
                        body: row.try_get(3)?,
                        likes: row.try_get(4)?,
                        is_spam: row.try_get(5)?,
                        created_at: row.try_get(6)?,
                        percent: row.try_get(7)?,
                        chat_color: row.try_get(8)?,
                        username: row.try_get(9)?,
                        role: row.try_get(10)?,
                        color1: row.try_get(11)?,
                        color2: row.try_get(12)?,
                        color3: row.try_get(13)?,
                        icon: row.try_get(14)?,
                        icon_type: row.try_get(15)?,
                        glow: row.try_get(16)?,
                    })
                },
            mapper: |it| Comment::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetCommentsByLikesParams,
        CommentQuery<'c, 'a, 's, C, Comment, 2>,
        C,
    > for GetCommentsByLikesStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetCommentsByLikesParams,
    ) -> CommentQuery<'c, 'a, 's, C, Comment, 2> {
        self.bind(client, &params.level_id, &params.offset)
    }
}
pub struct GetCommentHistoryStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_comment_history() -> GetCommentHistoryStmt {
    GetCommentHistoryStmt(
        "SELECT * FROM comment_view WHERE user_id = $1 ORDER BY created_at DESC LIMIT 10 OFFSET $2",
        None,
    )
}
impl GetCommentHistoryStmt {
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
    ) -> GetCommentHistoryQuery<'c, 'a, 's, C, GetCommentHistory, 2> {
        GetCommentHistoryQuery {
            client,
            params: [user_id, offset],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<GetCommentHistoryBorrowed, tokio_postgres::Error> {
                Ok(GetCommentHistoryBorrowed {
                    id: row.try_get(0)?,
                    level_id: row.try_get(1)?,
                    user_id: row.try_get(2)?,
                    body: row.try_get(3)?,
                    likes: row.try_get(4)?,
                    is_spam: row.try_get(5)?,
                    created_at: row.try_get(6)?,
                    percent: row.try_get(7)?,
                    chat_color: row.try_get(8)?,
                    username: row.try_get(9)?,
                    role: row.try_get(10)?,
                    color1: row.try_get(11)?,
                    color2: row.try_get(12)?,
                    color3: row.try_get(13)?,
                    icon: row.try_get(14)?,
                    icon_type: row.try_get(15)?,
                    glow: row.try_get(16)?,
                })
            },
            mapper: |it| GetCommentHistory::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetCommentHistoryParams,
        GetCommentHistoryQuery<'c, 'a, 's, C, GetCommentHistory, 2>,
        C,
    > for GetCommentHistoryStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetCommentHistoryParams,
    ) -> GetCommentHistoryQuery<'c, 'a, 's, C, GetCommentHistory, 2> {
        self.bind(client, &params.user_id, &params.offset)
    }
}
