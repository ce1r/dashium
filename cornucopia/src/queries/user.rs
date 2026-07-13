// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct CreateUserParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::BytesSql,
    T4: crate::BytesSql,
> {
    pub username: T1,
    pub email: T2,
    pub hash: T3,
    pub salt: T4,
}
#[derive(Debug)]
pub struct LoginUserParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub udid: T1,
    pub username: T2,
}
#[derive(Debug)]
pub struct SaveDataParams<T1: crate::StringSql> {
    pub save_data: T1,
    pub user_id: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct SaveStatsParams {
    pub stars: i32,
    pub demons: i32,
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
    pub user_id: i32,
}
#[derive(Debug)]
pub struct UpdateSettingsParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
    T6: crate::StringSql,
> {
    pub message_setting: i16,
    pub friend_setting: i16,
    pub comment_setting: i16,
    pub youtube: T1,
    pub twitter: T2,
    pub twitch: T3,
    pub discord: T4,
    pub instagram: T5,
    pub tiktok: T6,
    pub user_id: i32,
}
#[derive(Debug)]
pub struct SearchUsersParams<T1: crate::StringSql> {
    pub search: T1,
    pub user_id: i32,
    pub offset: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetHashAndSalt {
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
}
pub struct GetHashAndSaltBorrowed<'a> {
    pub hash: &'a [u8],
    pub salt: &'a [u8],
}
impl<'a> From<GetHashAndSaltBorrowed<'a>> for GetHashAndSalt {
    fn from(GetHashAndSaltBorrowed { hash, salt }: GetHashAndSaltBorrowed<'a>) -> Self {
        Self {
            hash: hash.into(),
            salt: salt.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetUser {
    pub id: i32,
    pub username: String,
    pub is_activated: bool,
    pub mod_level: i16,
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
}
pub struct GetUserBorrowed<'a> {
    pub id: i32,
    pub username: &'a str,
    pub is_activated: bool,
    pub mod_level: i16,
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
}
impl<'a> From<GetUserBorrowed<'a>> for GetUser {
    fn from(
        GetUserBorrowed {
            id,
            username,
            is_activated,
            mod_level,
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
        }: GetUserBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            username: username.into(),
            is_activated,
            mod_level,
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
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct LoginUser {
    pub id: i32,
    pub hash: Vec<u8>,
    pub salt: Vec<u8>,
}
pub struct LoginUserBorrowed<'a> {
    pub id: i32,
    pub hash: &'a [u8],
    pub salt: &'a [u8],
}
impl<'a> From<LoginUserBorrowed<'a>> for LoginUser {
    fn from(LoginUserBorrowed { id, hash, salt }: LoginUserBorrowed<'a>) -> Self {
        Self {
            id,
            hash: hash.into(),
            salt: salt.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SearchUsers {
    pub id: i32,
    pub username: String,
    pub stars: i32,
    pub demons: i32,
    pub creator_points: i32,
    pub diamonds: i32,
    pub moons: i32,
    pub secret_coins: i32,
    pub user_coins: i32,
    pub glow: i16,
    pub icon: i16,
    pub icon_type: i16,
    pub color1: i16,
    pub color2: i16,
    pub color3: i16,
}
pub struct SearchUsersBorrowed<'a> {
    pub id: i32,
    pub username: &'a str,
    pub stars: i32,
    pub demons: i32,
    pub creator_points: i32,
    pub diamonds: i32,
    pub moons: i32,
    pub secret_coins: i32,
    pub user_coins: i32,
    pub glow: i16,
    pub icon: i16,
    pub icon_type: i16,
    pub color1: i16,
    pub color2: i16,
    pub color3: i16,
}
impl<'a> From<SearchUsersBorrowed<'a>> for SearchUsers {
    fn from(
        SearchUsersBorrowed {
            id,
            username,
            stars,
            demons,
            creator_points,
            diamonds,
            moons,
            secret_coins,
            user_coins,
            glow,
            icon,
            icon_type,
            color1,
            color2,
            color3,
        }: SearchUsersBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            username: username.into(),
            stars,
            demons,
            creator_points,
            diamonds,
            moons,
            secret_coins,
            user_coins,
            glow,
            icon,
            icon_type,
            color1,
            color2,
            color3,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct GetHashAndSaltQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetHashAndSaltBorrowed, tokio_postgres::Error>,
    mapper: fn(GetHashAndSaltBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetHashAndSaltQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(GetHashAndSaltBorrowed) -> R,
    ) -> GetHashAndSaltQuery<'c, 'a, 's, C, R, N> {
        GetHashAndSaltQuery {
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
pub struct GetUserQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetUserBorrowed, tokio_postgres::Error>,
    mapper: fn(GetUserBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetUserQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(GetUserBorrowed) -> R) -> GetUserQuery<'c, 'a, 's, C, R, N> {
        GetUserQuery {
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
pub struct LoginUserQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<LoginUserBorrowed, tokio_postgres::Error>,
    mapper: fn(LoginUserBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> LoginUserQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(LoginUserBorrowed) -> R) -> LoginUserQuery<'c, 'a, 's, C, R, N> {
        LoginUserQuery {
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
pub struct StringQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<&str, tokio_postgres::Error>,
    mapper: fn(&str) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> StringQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(&str) -> R) -> StringQuery<'c, 'a, 's, C, R, N> {
        StringQuery {
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
pub struct I16Query<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<i16, tokio_postgres::Error>,
    mapper: fn(i16) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> I16Query<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(i16) -> R) -> I16Query<'c, 'a, 's, C, R, N> {
        I16Query {
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
pub struct SearchUsersQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<SearchUsersBorrowed, tokio_postgres::Error>,
    mapper: fn(SearchUsersBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> SearchUsersQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(SearchUsersBorrowed) -> R,
    ) -> SearchUsersQuery<'c, 'a, 's, C, R, N> {
        SearchUsersQuery {
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
pub struct GetHashAndSaltStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_hash_and_salt() -> GetHashAndSaltStmt {
    GetHashAndSaltStmt("SELECT hash, salt FROM users WHERE id = $1", None)
}
impl GetHashAndSaltStmt {
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
    ) -> GetHashAndSaltQuery<'c, 'a, 's, C, GetHashAndSalt, 1> {
        GetHashAndSaltQuery {
            client,
            params: [user_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<GetHashAndSaltBorrowed, tokio_postgres::Error> {
                Ok(GetHashAndSaltBorrowed {
                    hash: row.try_get(0)?,
                    salt: row.try_get(1)?,
                })
            },
            mapper: |it| GetHashAndSalt::from(it),
        }
    }
}
pub struct CreateUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_user() -> CreateUserStmt {
    CreateUserStmt(
        "INSERT INTO users ( username, email, hash, salt ) VALUES ( $1, $2, $3, $4 )",
        None,
    )
}
impl CreateUserStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::BytesSql,
        T4: crate::BytesSql,
    >(
        &'s self,
        client: &'c C,
        username: &'a T1,
        email: &'a T2,
        hash: &'a T3,
        salt: &'a T4,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[username, email, hash, salt]).await
    }
}
impl<
    'a,
    C: GenericClient + Send + Sync,
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::BytesSql,
    T4: crate::BytesSql,
>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        CreateUserParams<T1, T2, T3, T4>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for CreateUserStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a CreateUserParams<T1, T2, T3, T4>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.username,
            &params.email,
            &params.hash,
            &params.salt,
        ))
    }
}
pub struct GetUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_user() -> GetUserStmt {
    GetUserStmt(
        "SELECT id, username, is_activated, mod_level, stars, demons, creator_points, diamonds, moons, secret_coins, user_coins, cube, ship, ball, ufo, wave, robot, spider, swing, jetpack, glow, explosion, icon, icon_type, color1, color2, color3, message_setting, friend_setting, comment_setting, youtube, twitter, twitch, discord, instagram, tiktok FROM users WHERE id = $1",
        None,
    )
}
impl GetUserStmt {
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
        id: &'a i32,
    ) -> GetUserQuery<'c, 'a, 's, C, GetUser, 1> {
        GetUserQuery {
            client,
            params: [id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<GetUserBorrowed, tokio_postgres::Error> {
                    Ok(GetUserBorrowed {
                        id: row.try_get(0)?,
                        username: row.try_get(1)?,
                        is_activated: row.try_get(2)?,
                        mod_level: row.try_get(3)?,
                        stars: row.try_get(4)?,
                        demons: row.try_get(5)?,
                        creator_points: row.try_get(6)?,
                        diamonds: row.try_get(7)?,
                        moons: row.try_get(8)?,
                        secret_coins: row.try_get(9)?,
                        user_coins: row.try_get(10)?,
                        cube: row.try_get(11)?,
                        ship: row.try_get(12)?,
                        ball: row.try_get(13)?,
                        ufo: row.try_get(14)?,
                        wave: row.try_get(15)?,
                        robot: row.try_get(16)?,
                        spider: row.try_get(17)?,
                        swing: row.try_get(18)?,
                        jetpack: row.try_get(19)?,
                        glow: row.try_get(20)?,
                        explosion: row.try_get(21)?,
                        icon: row.try_get(22)?,
                        icon_type: row.try_get(23)?,
                        color1: row.try_get(24)?,
                        color2: row.try_get(25)?,
                        color3: row.try_get(26)?,
                        message_setting: row.try_get(27)?,
                        friend_setting: row.try_get(28)?,
                        comment_setting: row.try_get(29)?,
                        youtube: row.try_get(30)?,
                        twitter: row.try_get(31)?,
                        twitch: row.try_get(32)?,
                        discord: row.try_get(33)?,
                        instagram: row.try_get(34)?,
                        tiktok: row.try_get(35)?,
                    })
                },
            mapper: |it| GetUser::from(it),
        }
    }
}
pub struct LoginUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn login_user() -> LoginUserStmt {
    LoginUserStmt(
        "UPDATE users SET udid = $1 WHERE username = $2 RETURNING id, hash, salt",
        None,
    )
}
impl LoginUserStmt {
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
        udid: &'a T1,
        username: &'a T2,
    ) -> LoginUserQuery<'c, 'a, 's, C, LoginUser, 2> {
        LoginUserQuery {
            client,
            params: [udid, username],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<LoginUserBorrowed, tokio_postgres::Error> {
                    Ok(LoginUserBorrowed {
                        id: row.try_get(0)?,
                        hash: row.try_get(1)?,
                        salt: row.try_get(2)?,
                    })
                },
            mapper: |it| LoginUser::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        LoginUserParams<T1, T2>,
        LoginUserQuery<'c, 'a, 's, C, LoginUser, 2>,
        C,
    > for LoginUserStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a LoginUserParams<T1, T2>,
    ) -> LoginUserQuery<'c, 'a, 's, C, LoginUser, 2> {
        self.bind(client, &params.udid, &params.username)
    }
}
pub struct SaveDataStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn save_data() -> SaveDataStmt {
    SaveDataStmt("UPDATE users SET save_data = $1 WHERE id = $2", None)
}
impl SaveDataStmt {
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
        save_data: &'a T1,
        user_id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[save_data, user_id]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        SaveDataParams<T1>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for SaveDataStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a SaveDataParams<T1>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.save_data, &params.user_id))
    }
}
pub struct LoadDataStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn load_data() -> LoadDataStmt {
    LoadDataStmt("SELECT save_data FROM users WHERE id = $1", None)
}
impl LoadDataStmt {
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
    ) -> StringQuery<'c, 'a, 's, C, String, 1> {
        StringQuery {
            client,
            params: [user_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
pub struct SaveStatsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn save_stats() -> SaveStatsStmt {
    SaveStatsStmt(
        "UPDATE users SET stars = $1, demons = $2, diamonds = $3, moons = $4, secret_coins = $5, user_coins = $6, cube = $7, ship = $8, ball = $9, ufo = $10, wave = $11, robot = $12, spider = $13, swing = $14, jetpack = $15, glow = $16, explosion = $17, icon = $18, icon_type = $19, color1 = $20, color2 = $21, color3 = $22 WHERE id = $23 RETURNING id",
        None,
    )
}
impl SaveStatsStmt {
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
        stars: &'a i32,
        demons: &'a i32,
        diamonds: &'a i32,
        moons: &'a i32,
        secret_coins: &'a i32,
        user_coins: &'a i32,
        cube: &'a i16,
        ship: &'a i16,
        ball: &'a i16,
        ufo: &'a i16,
        wave: &'a i16,
        robot: &'a i16,
        spider: &'a i16,
        swing: &'a i16,
        jetpack: &'a i16,
        glow: &'a i16,
        explosion: &'a i16,
        icon: &'a i16,
        icon_type: &'a i16,
        color1: &'a i16,
        color2: &'a i16,
        color3: &'a i16,
        user_id: &'a i32,
    ) -> I32Query<'c, 'a, 's, C, i32, 23> {
        I32Query {
            client,
            params: [
                stars,
                demons,
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
                user_id,
            ],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<'c, 'a, 's, SaveStatsParams, I32Query<'c, 'a, 's, C, i32, 23>, C>
    for SaveStatsStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a SaveStatsParams,
    ) -> I32Query<'c, 'a, 's, C, i32, 23> {
        self.bind(
            client,
            &params.stars,
            &params.demons,
            &params.diamonds,
            &params.moons,
            &params.secret_coins,
            &params.user_coins,
            &params.cube,
            &params.ship,
            &params.ball,
            &params.ufo,
            &params.wave,
            &params.robot,
            &params.spider,
            &params.swing,
            &params.jetpack,
            &params.glow,
            &params.explosion,
            &params.icon,
            &params.icon_type,
            &params.color1,
            &params.color2,
            &params.color3,
            &params.user_id,
        )
    }
}
pub struct GetModLevelStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_mod_level() -> GetModLevelStmt {
    GetModLevelStmt("SELECT mod_level FROM users WHERE id = $1", None)
}
impl GetModLevelStmt {
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
    ) -> I16Query<'c, 'a, 's, C, i16, 1> {
        I16Query {
            client,
            params: [user_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
pub struct UpdateSettingsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn update_settings() -> UpdateSettingsStmt {
    UpdateSettingsStmt(
        "UPDATE users SET message_setting = $1, friend_setting = $2, comment_setting = $3, youtube = $4, twitter = $5, twitch = $6, discord = $7, instagram = $8, tiktok = $9 WHERE id = $10",
        None,
    )
}
impl UpdateSettingsStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub async fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
        T6: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        message_setting: &'a i16,
        friend_setting: &'a i16,
        comment_setting: &'a i16,
        youtube: &'a T1,
        twitter: &'a T2,
        twitch: &'a T3,
        discord: &'a T4,
        instagram: &'a T5,
        tiktok: &'a T6,
        user_id: &'a i32,
    ) -> Result<u64, tokio_postgres::Error> {
        client
            .execute(
                self.0,
                &[
                    message_setting,
                    friend_setting,
                    comment_setting,
                    youtube,
                    twitter,
                    twitch,
                    discord,
                    instagram,
                    tiktok,
                    user_id,
                ],
            )
            .await
    }
}
impl<
    'a,
    C: GenericClient + Send + Sync,
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
    T6: crate::StringSql,
>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        UpdateSettingsParams<T1, T2, T3, T4, T5, T6>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for UpdateSettingsStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a UpdateSettingsParams<T1, T2, T3, T4, T5, T6>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.message_setting,
            &params.friend_setting,
            &params.comment_setting,
            &params.youtube,
            &params.twitter,
            &params.twitch,
            &params.discord,
            &params.instagram,
            &params.tiktok,
            &params.user_id,
        ))
    }
}
pub struct SearchUsersStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn search_users() -> SearchUsersStmt {
    SearchUsersStmt(
        "SELECT id, username, stars, demons, creator_points, diamonds, moons, secret_coins, user_coins, glow, icon, icon_type, color1, color2, color3 FROM users WHERE username ILIKE '%' || $1 || '%' AND id != $2 LIMIT 10 OFFSET $3",
        None,
    )
}
impl SearchUsersStmt {
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
        user_id: &'a i32,
        offset: &'a i64,
    ) -> SearchUsersQuery<'c, 'a, 's, C, SearchUsers, 3> {
        SearchUsersQuery {
            client,
            params: [search, user_id, offset],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<SearchUsersBorrowed, tokio_postgres::Error> {
                    Ok(SearchUsersBorrowed {
                        id: row.try_get(0)?,
                        username: row.try_get(1)?,
                        stars: row.try_get(2)?,
                        demons: row.try_get(3)?,
                        creator_points: row.try_get(4)?,
                        diamonds: row.try_get(5)?,
                        moons: row.try_get(6)?,
                        secret_coins: row.try_get(7)?,
                        user_coins: row.try_get(8)?,
                        glow: row.try_get(9)?,
                        icon: row.try_get(10)?,
                        icon_type: row.try_get(11)?,
                        color1: row.try_get(12)?,
                        color2: row.try_get(13)?,
                        color3: row.try_get(14)?,
                    })
                },
            mapper: |it| SearchUsers::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        SearchUsersParams<T1>,
        SearchUsersQuery<'c, 'a, 's, C, SearchUsers, 3>,
        C,
    > for SearchUsersStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a SearchUsersParams<T1>,
    ) -> SearchUsersQuery<'c, 'a, 's, C, SearchUsers, 3> {
        self.bind(client, &params.search, &params.user_id, &params.offset)
    }
}
pub struct GetUdidStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_udid() -> GetUdidStmt {
    GetUdidStmt("SELECT udid FROM users WHERE id = $1", None)
}
impl GetUdidStmt {
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
    ) -> StringQuery<'c, 'a, 's, C, String, 1> {
        StringQuery {
            client,
            params: [user_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
