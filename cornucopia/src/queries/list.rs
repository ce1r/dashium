// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct CreateListParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::ArraySql<Item = i32>,
> {
    pub name: T1,
    pub description: T2,
    pub user_id: i32,
    pub levels: T3,
    pub difficulty: i16,
}
#[derive(Debug)]
pub struct SearchListsParams<T1: crate::StringSql> {
    pub search: T1,
    pub offset: i64,
}
#[derive(Clone, Copy, Debug)]
pub struct DeleteListParams {
    pub list_id: i32,
    pub user_id: i32,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct List {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub user_id: i32,
    pub downloads: i32,
    pub likes: i32,
    pub difficulty: i16,
    pub rated: bool,
    pub levels: Vec<i32>,
    pub reward: i32,
    pub requirement: i32,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub username: String,
}
pub struct ListBorrowed<'a> {
    pub id: i32,
    pub name: &'a str,
    pub description: &'a str,
    pub user_id: i32,
    pub downloads: i32,
    pub likes: i32,
    pub difficulty: i16,
    pub rated: bool,
    pub levels: crate::ArrayIterator<'a, i32>,
    pub reward: i32,
    pub requirement: i32,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub username: &'a str,
}
impl<'a> From<ListBorrowed<'a>> for List {
    fn from(
        ListBorrowed {
            id,
            name,
            description,
            user_id,
            downloads,
            likes,
            difficulty,
            rated,
            levels,
            reward,
            requirement,
            created_at,
            username,
        }: ListBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
            user_id,
            downloads,
            likes,
            difficulty,
            rated,
            levels: levels.map(|v| v).collect(),
            reward,
            requirement,
            created_at,
            username: username.into(),
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
pub struct ListQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<ListBorrowed, tokio_postgres::Error>,
    mapper: fn(ListBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> ListQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(ListBorrowed) -> R) -> ListQuery<'c, 'a, 's, C, R, N> {
        ListQuery {
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
pub struct CreateListStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_list() -> CreateListStmt {
    CreateListStmt(
        "INSERT INTO lists ( name, description, user_id, levels, difficulty ) SELECT $1, $2, $3, $4, $5 FROM users WHERE id = $3 RETURNING id",
        None,
    )
}
impl CreateListStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::ArraySql<Item = i32>,
    >(
        &'s self,
        client: &'c C,
        name: &'a T1,
        description: &'a T2,
        user_id: &'a i32,
        levels: &'a T3,
        difficulty: &'a i16,
    ) -> I32Query<'c, 'a, 's, C, i32, 5> {
        I32Query {
            client,
            params: [name, description, user_id, levels, difficulty],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
impl<
    'c,
    'a,
    's,
    C: GenericClient,
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::ArraySql<Item = i32>,
>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateListParams<T1, T2, T3>,
        I32Query<'c, 'a, 's, C, i32, 5>,
        C,
    > for CreateListStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateListParams<T1, T2, T3>,
    ) -> I32Query<'c, 'a, 's, C, i32, 5> {
        self.bind(
            client,
            &params.name,
            &params.description,
            &params.user_id,
            &params.levels,
            &params.difficulty,
        )
    }
}
pub struct SearchListsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn search_lists() -> SearchListsStmt {
    SearchListsStmt(
        "SELECT * FROM list_view WHERE name ILIKE '%' || $1 || '%' LIMIT 10 OFFSET $2",
        None,
    )
}
impl SearchListsStmt {
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
        search: &'a T1,
        offset: &'a i64,
    ) -> ListQuery<'c, 'a, 's, C, List, 2> {
        ListQuery {
            client,
            params: [search, offset],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<ListBorrowed, tokio_postgres::Error> {
                Ok(ListBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    description: row.try_get(2)?,
                    user_id: row.try_get(3)?,
                    downloads: row.try_get(4)?,
                    likes: row.try_get(5)?,
                    difficulty: row.try_get(6)?,
                    rated: row.try_get(7)?,
                    levels: row.try_get(8)?,
                    reward: row.try_get(9)?,
                    requirement: row.try_get(10)?,
                    created_at: row.try_get(11)?,
                    username: row.try_get(12)?,
                })
            },
            mapper: |it| List::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        SearchListsParams<T1>,
        ListQuery<'c, 'a, 's, C, List, 2>,
        C,
    > for SearchListsStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a SearchListsParams<T1>,
    ) -> ListQuery<'c, 'a, 's, C, List, 2> {
        self.bind(client, &params.search, &params.offset)
    }
}
pub struct DeleteListStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_list() -> DeleteListStmt {
    DeleteListStmt("DELETE FROM lists WHERE id = $1 AND user_id = $2", None)
}
impl DeleteListStmt {
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
        list_id: &'a i32,
        user_id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[list_id, user_id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        DeleteListParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for DeleteListStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a DeleteListParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.list_id, &params.user_id))
    }
}
