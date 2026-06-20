// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct CreateLevelParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
> {
    pub name: T1,
    pub description: T2,
    pub level_data: T3,
    pub version: i32,
    pub original_level_id: i32,
    pub length: i16,
    pub objects: i32,
    pub requested_stars: i16,
    pub coins: i16,
    pub is_auto: bool,
    pub is_ldm: bool,
    pub is_two_player: bool,
    pub official_song_id: i32,
    pub song_id: i32,
    pub visibility: i16,
    pub user_id: i32,
    pub gjp2: T4,
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
pub struct CreateLevelStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_level() -> CreateLevelStmt {
    CreateLevelStmt(
        "INSERT INTO levels ( name, description, user_id, level_data, version, original_level_id, length, objects, requested_stars, coins, is_auto, is_ldm, is_two_player, official_song_id, song_id, visibility ) SELECT $1, $2, users.id, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15 FROM users WHERE id = $16 AND gjp2 = $17 RETURNING id",
        None,
    )
}
impl CreateLevelStmt {
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
        T3: crate::StringSql,
        T4: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        name: &'a T1,
        description: &'a T2,
        level_data: &'a T3,
        version: &'a i32,
        original_level_id: &'a i32,
        length: &'a i16,
        objects: &'a i32,
        requested_stars: &'a i16,
        coins: &'a i16,
        is_auto: &'a bool,
        is_ldm: &'a bool,
        is_two_player: &'a bool,
        official_song_id: &'a i32,
        song_id: &'a i32,
        visibility: &'a i16,
        user_id: &'a i32,
        gjp2: &'a T4,
    ) -> I32Query<'c, 'a, 's, C, i32, 17> {
        I32Query {
            client,
            params: [
                name,
                description,
                level_data,
                version,
                original_level_id,
                length,
                objects,
                requested_stars,
                coins,
                is_auto,
                is_ldm,
                is_two_player,
                official_song_id,
                song_id,
                visibility,
                user_id,
                gjp2,
            ],
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
    T3: crate::StringSql,
    T4: crate::StringSql,
>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        CreateLevelParams<T1, T2, T3, T4>,
        I32Query<'c, 'a, 's, C, i32, 17>,
        C,
    > for CreateLevelStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateLevelParams<T1, T2, T3, T4>,
    ) -> I32Query<'c, 'a, 's, C, i32, 17> {
        self.bind(
            client,
            &params.name,
            &params.description,
            &params.level_data,
            &params.version,
            &params.original_level_id,
            &params.length,
            &params.objects,
            &params.requested_stars,
            &params.coins,
            &params.is_auto,
            &params.is_ldm,
            &params.is_two_player,
            &params.official_song_id,
            &params.song_id,
            &params.visibility,
            &params.user_id,
            &params.gjp2,
        )
    }
}
