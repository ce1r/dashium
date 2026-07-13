// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug, Clone, PartialEq)]
pub struct GetQuests {
    pub id: i32,
    pub item_type: crate::types::ItemType,
    pub amount: i16,
    pub reward: i16,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
pub struct GetQuestsBorrowed<'a> {
    pub id: i32,
    pub item_type: crate::types::ItemType,
    pub amount: i16,
    pub reward: i16,
    pub name: &'a str,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
impl<'a> From<GetQuestsBorrowed<'a>> for GetQuests {
    fn from(
        GetQuestsBorrowed {
            id,
            item_type,
            amount,
            reward,
            name,
            created_at,
        }: GetQuestsBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            item_type,
            amount,
            reward,
            name: name.into(),
            created_at,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct GetQuestsQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<GetQuestsBorrowed, tokio_postgres::Error>,
    mapper: fn(GetQuestsBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> GetQuestsQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(GetQuestsBorrowed) -> R) -> GetQuestsQuery<'c, 'a, 's, C, R, N> {
        GetQuestsQuery {
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
pub struct GetQuestsStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_quests() -> GetQuestsStmt {
    GetQuestsStmt(
        "SELECT * FROM quests ORDER BY created_at DESC LIMIT 3",
        None,
    )
}
impl GetQuestsStmt {
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
    ) -> GetQuestsQuery<'c, 'a, 's, C, GetQuests, 0> {
        GetQuestsQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<GetQuestsBorrowed, tokio_postgres::Error> {
                    Ok(GetQuestsBorrowed {
                        id: row.try_get(0)?,
                        item_type: row.try_get(1)?,
                        amount: row.try_get(2)?,
                        reward: row.try_get(3)?,
                        name: row.try_get(4)?,
                        created_at: row.try_get(5)?,
                    })
                },
            mapper: |it| GetQuests::from(it),
        }
    }
}
