// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct CreateLevelParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub name: T1,
    pub description: T2,
    pub version: i32,
    pub original_level_id: i32,
    pub length: crate::types::LevelLength,
    pub objects: i32,
    pub requested_stars: i16,
    pub coins: i16,
    pub is_ldm: bool,
    pub is_two_player: bool,
    pub is_platformer: bool,
    pub official_song_id: i32,
    pub song_id: i32,
    pub visibility: crate::types::Visibility,
    pub user_id: i32,
}
#[derive(Debug)]
pub struct SearchLevelsParams<T1: crate::StringSql> {
    pub search: T1,
    pub offset: i64,
}
#[derive(Clone, Copy, Debug)]
pub struct DeleteLevelParams {
    pub level_id: i32,
    pub user_id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct RateLevelParams {
    pub level_id: i32,
    pub rating: crate::types::Rating,
    pub stars: i16,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Level {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub user_id: i32,
    pub version: i32,
    pub original_level_id: i32,
    pub length: crate::types::LevelLength,
    pub objects: i32,
    pub requested_stars: i16,
    pub coins: i16,
    pub likes: i32,
    pub dislikes: i32,
    pub downloads: i32,
    pub is_ldm: bool,
    pub is_two_player: bool,
    pub is_platformer: bool,
    pub official_song_id: i32,
    pub song_id: i32,
    pub visibility: crate::types::Visibility,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub username: String,
    pub rating: Option<crate::types::Rating>,
    pub stars: i32,
    pub difficulty: crate::types::Difficulty,
    pub demon_difficulty: Option<crate::types::DemonDifficulty>,
    pub has_verified_coins: bool,
    pub is_auto: bool,
    pub is_demon: bool,
    pub is_featured: bool,
    pub is_rated: bool,
    pub rated_by: Option<i32>,
    pub rated_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}
pub struct LevelBorrowed<'a> {
    pub id: i32,
    pub name: &'a str,
    pub description: &'a str,
    pub user_id: i32,
    pub version: i32,
    pub original_level_id: i32,
    pub length: crate::types::LevelLength,
    pub objects: i32,
    pub requested_stars: i16,
    pub coins: i16,
    pub likes: i32,
    pub dislikes: i32,
    pub downloads: i32,
    pub is_ldm: bool,
    pub is_two_player: bool,
    pub is_platformer: bool,
    pub official_song_id: i32,
    pub song_id: i32,
    pub visibility: crate::types::Visibility,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub username: &'a str,
    pub rating: Option<crate::types::Rating>,
    pub stars: i32,
    pub difficulty: crate::types::Difficulty,
    pub demon_difficulty: Option<crate::types::DemonDifficulty>,
    pub has_verified_coins: bool,
    pub is_auto: bool,
    pub is_demon: bool,
    pub is_featured: bool,
    pub is_rated: bool,
    pub rated_by: Option<i32>,
    pub rated_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}
impl<'a> From<LevelBorrowed<'a>> for Level {
    fn from(
        LevelBorrowed {
            id,
            name,
            description,
            user_id,
            version,
            original_level_id,
            length,
            objects,
            requested_stars,
            coins,
            likes,
            dislikes,
            downloads,
            is_ldm,
            is_two_player,
            is_platformer,
            official_song_id,
            song_id,
            visibility,
            created_at,
            username,
            rating,
            stars,
            difficulty,
            demon_difficulty,
            has_verified_coins,
            is_auto,
            is_demon,
            is_featured,
            is_rated,
            rated_by,
            rated_at,
        }: LevelBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
            user_id,
            version,
            original_level_id,
            length,
            objects,
            requested_stars,
            coins,
            likes,
            dislikes,
            downloads,
            is_ldm,
            is_two_player,
            is_platformer,
            official_song_id,
            song_id,
            visibility,
            created_at,
            username: username.into(),
            rating,
            stars,
            difficulty,
            demon_difficulty,
            has_verified_coins,
            is_auto,
            is_demon,
            is_featured,
            is_rated,
            rated_by,
            rated_at,
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
pub struct LevelQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<LevelBorrowed, tokio_postgres::Error>,
    mapper: fn(LevelBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> LevelQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(LevelBorrowed) -> R) -> LevelQuery<'c, 'a, 's, C, R, N> {
        LevelQuery {
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
pub struct I64Query<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<i64, tokio_postgres::Error>,
    mapper: fn(i64) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> I64Query<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(i64) -> R) -> I64Query<'c, 'a, 's, C, R, N> {
        I64Query {
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
        "INSERT INTO levels ( name, description, user_id, version, original_level_id, length, objects, requested_stars, coins, is_ldm, is_two_player, is_platformer, official_song_id, song_id, visibility ) SELECT $1, $2, users.id, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14 FROM users WHERE users.id = $15 RETURNING id",
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
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>(
        &'s self,
        client: &'c C,
        name: &'a T1,
        description: &'a T2,
        version: &'a i32,
        original_level_id: &'a i32,
        length: &'a crate::types::LevelLength,
        objects: &'a i32,
        requested_stars: &'a i16,
        coins: &'a i16,
        is_ldm: &'a bool,
        is_two_player: &'a bool,
        is_platformer: &'a bool,
        official_song_id: &'a i32,
        song_id: &'a i32,
        visibility: &'a crate::types::Visibility,
        user_id: &'a i32,
    ) -> I32Query<'c, 'a, 's, C, i32, 15> {
        I32Query {
            client,
            params: [
                name,
                description,
                version,
                original_level_id,
                length,
                objects,
                requested_stars,
                coins,
                is_ldm,
                is_two_player,
                is_platformer,
                official_song_id,
                song_id,
                visibility,
                user_id,
            ],
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
        CreateLevelParams<T1, T2>,
        I32Query<'c, 'a, 's, C, i32, 15>,
        C,
    > for CreateLevelStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a CreateLevelParams<T1, T2>,
    ) -> I32Query<'c, 'a, 's, C, i32, 15> {
        self.bind(
            client,
            &params.name,
            &params.description,
            &params.version,
            &params.original_level_id,
            &params.length,
            &params.objects,
            &params.requested_stars,
            &params.coins,
            &params.is_ldm,
            &params.is_two_player,
            &params.is_platformer,
            &params.official_song_id,
            &params.song_id,
            &params.visibility,
            &params.user_id,
        )
    }
}
pub struct SearchLevelsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn search_levels() -> SearchLevelsStmt {
    SearchLevelsStmt(
        "SELECT * FROM level_view WHERE name ILIKE '%' || $1 || '%' LIMIT 10 OFFSET $2",
        None,
    )
}
impl SearchLevelsStmt {
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
    ) -> LevelQuery<'c, 'a, 's, C, Level, 2> {
        LevelQuery {
            client,
            params: [search, offset],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<LevelBorrowed, tokio_postgres::Error> {
                Ok(LevelBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    description: row.try_get(2)?,
                    user_id: row.try_get(3)?,
                    version: row.try_get(4)?,
                    original_level_id: row.try_get(5)?,
                    length: row.try_get(6)?,
                    objects: row.try_get(7)?,
                    requested_stars: row.try_get(8)?,
                    coins: row.try_get(9)?,
                    likes: row.try_get(10)?,
                    dislikes: row.try_get(11)?,
                    downloads: row.try_get(12)?,
                    is_ldm: row.try_get(13)?,
                    is_two_player: row.try_get(14)?,
                    is_platformer: row.try_get(15)?,
                    official_song_id: row.try_get(16)?,
                    song_id: row.try_get(17)?,
                    visibility: row.try_get(18)?,
                    created_at: row.try_get(19)?,
                    username: row.try_get(20)?,
                    rating: row.try_get(21)?,
                    stars: row.try_get(22)?,
                    difficulty: row.try_get(23)?,
                    demon_difficulty: row.try_get(24)?,
                    has_verified_coins: row.try_get(25)?,
                    is_auto: row.try_get(26)?,
                    is_demon: row.try_get(27)?,
                    is_featured: row.try_get(28)?,
                    is_rated: row.try_get(29)?,
                    rated_by: row.try_get(30)?,
                    rated_at: row.try_get(31)?,
                })
            },
            mapper: |it| Level::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        SearchLevelsParams<T1>,
        LevelQuery<'c, 'a, 's, C, Level, 2>,
        C,
    > for SearchLevelsStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a SearchLevelsParams<T1>,
    ) -> LevelQuery<'c, 'a, 's, C, Level, 2> {
        self.bind(client, &params.search, &params.offset)
    }
}
pub struct GetLevelStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_level() -> GetLevelStmt {
    GetLevelStmt("SELECT * FROM level_view WHERE id = $1", None)
}
impl GetLevelStmt {
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
    ) -> LevelQuery<'c, 'a, 's, C, Level, 1> {
        LevelQuery {
            client,
            params: [level_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<LevelBorrowed, tokio_postgres::Error> {
                Ok(LevelBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    description: row.try_get(2)?,
                    user_id: row.try_get(3)?,
                    version: row.try_get(4)?,
                    original_level_id: row.try_get(5)?,
                    length: row.try_get(6)?,
                    objects: row.try_get(7)?,
                    requested_stars: row.try_get(8)?,
                    coins: row.try_get(9)?,
                    likes: row.try_get(10)?,
                    dislikes: row.try_get(11)?,
                    downloads: row.try_get(12)?,
                    is_ldm: row.try_get(13)?,
                    is_two_player: row.try_get(14)?,
                    is_platformer: row.try_get(15)?,
                    official_song_id: row.try_get(16)?,
                    song_id: row.try_get(17)?,
                    visibility: row.try_get(18)?,
                    created_at: row.try_get(19)?,
                    username: row.try_get(20)?,
                    rating: row.try_get(21)?,
                    stars: row.try_get(22)?,
                    difficulty: row.try_get(23)?,
                    demon_difficulty: row.try_get(24)?,
                    has_verified_coins: row.try_get(25)?,
                    is_auto: row.try_get(26)?,
                    is_demon: row.try_get(27)?,
                    is_featured: row.try_get(28)?,
                    is_rated: row.try_get(29)?,
                    rated_by: row.try_get(30)?,
                    rated_at: row.try_get(31)?,
                })
            },
            mapper: |it| Level::from(it),
        }
    }
}
pub struct GetLevelsOfUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_levels_of_user() -> GetLevelsOfUserStmt {
    GetLevelsOfUserStmt(
        "SELECT level_view.* FROM level_view WHERE user_id = $1",
        None,
    )
}
impl GetLevelsOfUserStmt {
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
    ) -> LevelQuery<'c, 'a, 's, C, Level, 1> {
        LevelQuery {
            client,
            params: [user_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<LevelBorrowed, tokio_postgres::Error> {
                Ok(LevelBorrowed {
                    id: row.try_get(0)?,
                    name: row.try_get(1)?,
                    description: row.try_get(2)?,
                    user_id: row.try_get(3)?,
                    version: row.try_get(4)?,
                    original_level_id: row.try_get(5)?,
                    length: row.try_get(6)?,
                    objects: row.try_get(7)?,
                    requested_stars: row.try_get(8)?,
                    coins: row.try_get(9)?,
                    likes: row.try_get(10)?,
                    dislikes: row.try_get(11)?,
                    downloads: row.try_get(12)?,
                    is_ldm: row.try_get(13)?,
                    is_two_player: row.try_get(14)?,
                    is_platformer: row.try_get(15)?,
                    official_song_id: row.try_get(16)?,
                    song_id: row.try_get(17)?,
                    visibility: row.try_get(18)?,
                    created_at: row.try_get(19)?,
                    username: row.try_get(20)?,
                    rating: row.try_get(21)?,
                    stars: row.try_get(22)?,
                    difficulty: row.try_get(23)?,
                    demon_difficulty: row.try_get(24)?,
                    has_verified_coins: row.try_get(25)?,
                    is_auto: row.try_get(26)?,
                    is_demon: row.try_get(27)?,
                    is_featured: row.try_get(28)?,
                    is_rated: row.try_get(29)?,
                    rated_by: row.try_get(30)?,
                    rated_at: row.try_get(31)?,
                })
            },
            mapper: |it| Level::from(it),
        }
    }
}
pub struct GetLevelCountStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_level_count() -> GetLevelCountStmt {
    GetLevelCountStmt("SELECT COUNT(*) FROM levels", None)
}
impl GetLevelCountStmt {
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
    ) -> I64Query<'c, 'a, 's, C, i64, 0> {
        I64Query {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
pub struct DeleteLevelStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn delete_level() -> DeleteLevelStmt {
    DeleteLevelStmt(
        "DELETE FROM levels WHERE id = $1 AND user_id = $2 RETURNING id",
        None,
    )
}
impl DeleteLevelStmt {
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
        user_id: &'a i32,
    ) -> I32Query<'c, 'a, 's, C, i32, 2> {
        I32Query {
            client,
            params: [level_id, user_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<'c, 'a, 's, DeleteLevelParams, I32Query<'c, 'a, 's, C, i32, 2>, C>
    for DeleteLevelStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a DeleteLevelParams,
    ) -> I32Query<'c, 'a, 's, C, i32, 2> {
        self.bind(client, &params.level_id, &params.user_id)
    }
}
pub struct RateLevelStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn rate_level() -> RateLevelStmt {
    RateLevelStmt(
        "INSERT INTO rates ( level_id, rating, stars ) VALUES ( $1, $2, $3 ) ON CONFLICT (level_id) DO UPDATE SET rating = EXCLUDED.rating, stars = EXCLUDED.stars",
        None,
    )
}
impl RateLevelStmt {
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
        level_id: &'a i32,
        rating: &'a crate::types::Rating,
        stars: &'a i16,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[level_id, rating, stars]).await
    }
}
impl<'a, C: GenericClient + Send + Sync>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        RateLevelParams,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for RateLevelStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a RateLevelParams,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.level_id, &params.rating, &params.stars))
    }
}
