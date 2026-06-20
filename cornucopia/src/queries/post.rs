// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct CreatePostParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub user_id: i32,
    pub body: T1,
    pub gjp2: T2,
}
#[derive(Clone, Copy, Debug)]
pub struct GetPostsParams {
    pub user_id: i32,
    pub offset: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetPosts {
    pub id: i32,
    pub user_id: i32,
    pub body: String,
    pub likes: i32,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct GetPostsBorrowed<'a> {
    pub id: i32,
    pub user_id: i32,
    pub body: &'a str,
    pub likes: i32,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<GetPostsBorrowed<'a>> for GetPosts {
    fn from(
        GetPostsBorrowed {
            id,
            user_id,
            body,
            likes,
            created_at,
        }: GetPostsBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            user_id,
            body: body.into(),
            likes,
            created_at,
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
pub struct GetPostsQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetPostsBorrowed, tokio_postgres::Error>,
    mapper: fn(GetPostsBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetPostsQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(GetPostsBorrowed) -> R) -> GetPostsQuery<'c, 'a, 's, C, R, N> {
        GetPostsQuery {
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
pub struct CreatePostStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_post() -> CreatePostStmt {
    CreatePostStmt(
        "INSERT INTO posts ( user_id, body ) SELECT $1, $2 FROM users WHERE id = $1 AND gjp2 = $3 RETURNING id",
        None,
    )
}
impl CreatePostStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        user_id: &'a i32,
        body: &'a T1,
        gjp2: &'a T2,
    ) -> I32Query<'c, 'a, 's, C, i32, 3> {
        I32Query {
            client,
            params: [user_id, body, gjp2],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreatePostParams<T1, T2>,
        I32Query<'c, 'a, 's, C, i32, 3>,
        C,
    > for CreatePostStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreatePostParams<T1, T2>,
    ) -> I32Query<'c, 'a, 's, C, i32, 3> {
        self.bind(client, &params.user_id, &params.body, &params.gjp2)
    }
}
pub struct GetPostsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_posts() -> GetPostsStmt {
    GetPostsStmt(
        "SELECT * FROM posts WHERE user_id = $1 ORDER BY created_at DESC LIMIT 10 OFFSET $2",
        None,
    )
}
impl GetPostsStmt {
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
    ) -> GetPostsQuery<'c, 'a, 's, C, GetPosts, 2> {
        GetPostsQuery {
            client,
            params: [user_id, offset],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<GetPostsBorrowed, tokio_postgres::Error> {
                    Ok(GetPostsBorrowed {
                        id: row.try_get(0)?,
                        user_id: row.try_get(1)?,
                        body: row.try_get(2)?,
                        likes: row.try_get(3)?,
                        created_at: row.try_get(4)?,
                    })
                },
            mapper: |it| GetPosts::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetPostsParams,
        GetPostsQuery<'c, 'a, 's, C, GetPosts, 2>,
        C,
    > for GetPostsStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a GetPostsParams,
    ) -> GetPostsQuery<'c, 'a, 's, C, GetPosts, 2> {
        self.bind(client, &params.user_id, &params.offset)
    }
}
