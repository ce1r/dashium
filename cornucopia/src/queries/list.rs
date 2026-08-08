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
