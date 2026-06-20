// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug)]
pub struct CreateUserParams<T1: crate::StringSql, T2: crate::StringSql, T3: crate::StringSql> {
    pub username: T1,
    pub email: T2,
    pub gjp2: T3,
}
#[derive(Debug)]
pub struct VerifyGjp2Params<T1: crate::StringSql, T2: crate::StringSql> {
    pub username: T1,
    pub gjp2: T2,
}
#[derive(Debug)]
pub struct SaveDataParams<T1: crate::StringSql, T2: crate::StringSql> {
    pub save_data: T1,
    pub user_id: i32,
    pub gjp2: T2,
}
#[derive(Debug)]
pub struct LoadDataParams<T1: crate::StringSql> {
    pub user_id: i32,
    pub gjp2: T1,
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
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
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
pub struct BoolQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<bool, tokio_postgres::Error>,
    mapper: fn(bool) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> BoolQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(bool) -> R) -> BoolQuery<'c, 'a, 's, C, R, N> {
        BoolQuery {
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
pub struct CreateUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn create_user() -> CreateUserStmt {
    CreateUserStmt(
        "INSERT INTO users ( username, email, gjp2 ) VALUES ( $1, $2, $3 )",
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
        T3: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        username: &'a T1,
        email: &'a T2,
        gjp2: &'a T3,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[username, email, gjp2]).await
    }
}
impl<
    'a,
    C: GenericClient + Send + Sync,
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        CreateUserParams<T1, T2, T3>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for CreateUserStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a CreateUserParams<T1, T2, T3>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.username, &params.email, &params.gjp2))
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
pub struct IsUsernameTakenStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn is_username_taken() -> IsUsernameTakenStmt {
    IsUsernameTakenStmt(
        "SELECT EXISTS ( SELECT 1 FROM users WHERE username ILIKE $1 )",
        None,
    )
}
impl IsUsernameTakenStmt {
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
        username: &'a T1,
    ) -> BoolQuery<'c, 'a, 's, C, bool, 1> {
        BoolQuery {
            client,
            params: [username],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
pub struct IsEmailTakenStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn is_email_taken() -> IsEmailTakenStmt {
    IsEmailTakenStmt(
        "SELECT EXISTS ( SELECT 1 FROM users WHERE email ILIKE $1 )",
        None,
    )
}
impl IsEmailTakenStmt {
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
        email: &'a T1,
    ) -> BoolQuery<'c, 'a, 's, C, bool, 1> {
        BoolQuery {
            client,
            params: [email],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it,
        }
    }
}
pub struct VerifyGjp2Stmt(&'static str, Option<tokio_postgres::Statement>);
pub fn verify_gjp2() -> VerifyGjp2Stmt {
    VerifyGjp2Stmt(
        "SELECT id FROM users WHERE username = $1 AND gjp2 = $2",
        None,
    )
}
impl VerifyGjp2Stmt {
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
        username: &'a T1,
        gjp2: &'a T2,
    ) -> I32Query<'c, 'a, 's, C, i32, 2> {
        I32Query {
            client,
            params: [username, gjp2],
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
        VerifyGjp2Params<T1, T2>,
        I32Query<'c, 'a, 's, C, i32, 2>,
        C,
    > for VerifyGjp2Stmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a VerifyGjp2Params<T1, T2>,
    ) -> I32Query<'c, 'a, 's, C, i32, 2> {
        self.bind(client, &params.username, &params.gjp2)
    }
}
pub struct SaveDataStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn save_data() -> SaveDataStmt {
    SaveDataStmt(
        "UPDATE users SET save_data = $1 WHERE id = $2 AND gjp2 = $3",
        None,
    )
}
impl SaveDataStmt {
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
        save_data: &'a T1,
        user_id: &'a i32,
        gjp2: &'a T2,
    ) -> Result<u64, tokio_postgres::Error> {
        client.execute(self.0, &[save_data, user_id, gjp2]).await
    }
}
impl<'a, C: GenericClient + Send + Sync, T1: crate::StringSql, T2: crate::StringSql>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        SaveDataParams<T1, T2>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for SaveDataStmt
{
    fn params(
        &'a self,
        client: &'a C,
        params: &'a SaveDataParams<T1, T2>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(client, &params.save_data, &params.user_id, &params.gjp2))
    }
}
pub struct LoadDataStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn load_data() -> LoadDataStmt {
    LoadDataStmt(
        "SELECT save_data FROM users WHERE id = $1 AND gjp2 = $2",
        None,
    )
}
impl LoadDataStmt {
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
        gjp2: &'a T1,
    ) -> StringQuery<'c, 'a, 's, C, String, 2> {
        StringQuery {
            client,
            params: [user_id, gjp2],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        LoadDataParams<T1>,
        StringQuery<'c, 'a, 's, C, String, 2>,
        C,
    > for LoadDataStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a LoadDataParams<T1>,
    ) -> StringQuery<'c, 'a, 's, C, String, 2> {
        self.bind(client, &params.user_id, &params.gjp2)
    }
}
