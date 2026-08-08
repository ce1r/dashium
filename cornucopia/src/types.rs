// This file was generated with `cornucopia`. Do not modify.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
#[allow(non_camel_case_types)]
pub enum Role {
    User,
    Moderator,
    ElderModerator,
    LeaderboardModerator,
    Administrator,
}
impl<'a> postgres_types::ToSql for Role {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let s = match *self {
            Role::User => "User",
            Role::Moderator => "Moderator",
            Role::ElderModerator => "ElderModerator",
            Role::LeaderboardModerator => "LeaderboardModerator",
            Role::Administrator => "Administrator",
        };
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "role" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 5 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "User" => true,
                    "Moderator" => true,
                    "ElderModerator" => true,
                    "LeaderboardModerator" => true,
                    "Administrator" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
    fn to_sql_checked(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
impl<'a> postgres_types::FromSql<'a> for Role {
    fn from_sql(
        ty: &postgres_types::Type,
        buf: &'a [u8],
    ) -> Result<Role, Box<dyn std::error::Error + Sync + Send>> {
        match std::str::from_utf8(buf)? {
            "User" => Ok(Role::User),
            "Moderator" => Ok(Role::Moderator),
            "ElderModerator" => Ok(Role::ElderModerator),
            "LeaderboardModerator" => Ok(Role::LeaderboardModerator),
            "Administrator" => Ok(Role::Administrator),
            s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "role" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 5 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "User" => true,
                    "Moderator" => true,
                    "ElderModerator" => true,
                    "LeaderboardModerator" => true,
                    "Administrator" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[allow(non_camel_case_types)]
pub enum LevelLength {
    Tiny,
    Short,
    Medium,
    Long,
    XL,
}
impl<'a> postgres_types::ToSql for LevelLength {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let s = match *self {
            LevelLength::Tiny => "Tiny",
            LevelLength::Short => "Short",
            LevelLength::Medium => "Medium",
            LevelLength::Long => "Long",
            LevelLength::XL => "XL",
        };
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "level_length" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 5 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "Tiny" => true,
                    "Short" => true,
                    "Medium" => true,
                    "Long" => true,
                    "XL" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
    fn to_sql_checked(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
impl<'a> postgres_types::FromSql<'a> for LevelLength {
    fn from_sql(
        ty: &postgres_types::Type,
        buf: &'a [u8],
    ) -> Result<LevelLength, Box<dyn std::error::Error + Sync + Send>> {
        match std::str::from_utf8(buf)? {
            "Tiny" => Ok(LevelLength::Tiny),
            "Short" => Ok(LevelLength::Short),
            "Medium" => Ok(LevelLength::Medium),
            "Long" => Ok(LevelLength::Long),
            "XL" => Ok(LevelLength::XL),
            s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "level_length" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 5 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "Tiny" => true,
                    "Short" => true,
                    "Medium" => true,
                    "Long" => true,
                    "XL" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[allow(non_camel_case_types)]
pub enum Visibility {
    Public,
    FriendsOnly,
    Private,
}
impl<'a> postgres_types::ToSql for Visibility {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let s = match *self {
            Visibility::Public => "Public",
            Visibility::FriendsOnly => "FriendsOnly",
            Visibility::Private => "Private",
        };
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "visibility" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 3 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "Public" => true,
                    "FriendsOnly" => true,
                    "Private" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
    fn to_sql_checked(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
impl<'a> postgres_types::FromSql<'a> for Visibility {
    fn from_sql(
        ty: &postgres_types::Type,
        buf: &'a [u8],
    ) -> Result<Visibility, Box<dyn std::error::Error + Sync + Send>> {
        match std::str::from_utf8(buf)? {
            "Public" => Ok(Visibility::Public),
            "FriendsOnly" => Ok(Visibility::FriendsOnly),
            "Private" => Ok(Visibility::Private),
            s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "visibility" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 3 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "Public" => true,
                    "FriendsOnly" => true,
                    "Private" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[allow(non_camel_case_types)]
pub enum Rating {
    Star,
    Feature,
    Epic,
    Legendary,
    Mythic,
}
impl<'a> postgres_types::ToSql for Rating {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let s = match *self {
            Rating::Star => "Star",
            Rating::Feature => "Feature",
            Rating::Epic => "Epic",
            Rating::Legendary => "Legendary",
            Rating::Mythic => "Mythic",
        };
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "rating" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 5 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "Star" => true,
                    "Feature" => true,
                    "Epic" => true,
                    "Legendary" => true,
                    "Mythic" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
    fn to_sql_checked(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
impl<'a> postgres_types::FromSql<'a> for Rating {
    fn from_sql(
        ty: &postgres_types::Type,
        buf: &'a [u8],
    ) -> Result<Rating, Box<dyn std::error::Error + Sync + Send>> {
        match std::str::from_utf8(buf)? {
            "Star" => Ok(Rating::Star),
            "Feature" => Ok(Rating::Feature),
            "Epic" => Ok(Rating::Epic),
            "Legendary" => Ok(Rating::Legendary),
            "Mythic" => Ok(Rating::Mythic),
            s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "rating" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 5 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "Star" => true,
                    "Feature" => true,
                    "Epic" => true,
                    "Legendary" => true,
                    "Mythic" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[allow(non_camel_case_types)]
pub enum Difficulty {
    NA,
    Auto,
    Easy,
    Normal,
    Hard,
    Harder,
    Insane,
    Demon,
}
impl<'a> postgres_types::ToSql for Difficulty {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let s = match *self {
            Difficulty::NA => "NA",
            Difficulty::Auto => "Auto",
            Difficulty::Easy => "Easy",
            Difficulty::Normal => "Normal",
            Difficulty::Hard => "Hard",
            Difficulty::Harder => "Harder",
            Difficulty::Insane => "Insane",
            Difficulty::Demon => "Demon",
        };
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "difficulty" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 8 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "NA" => true,
                    "Auto" => true,
                    "Easy" => true,
                    "Normal" => true,
                    "Hard" => true,
                    "Harder" => true,
                    "Insane" => true,
                    "Demon" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
    fn to_sql_checked(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
impl<'a> postgres_types::FromSql<'a> for Difficulty {
    fn from_sql(
        ty: &postgres_types::Type,
        buf: &'a [u8],
    ) -> Result<Difficulty, Box<dyn std::error::Error + Sync + Send>> {
        match std::str::from_utf8(buf)? {
            "NA" => Ok(Difficulty::NA),
            "Auto" => Ok(Difficulty::Auto),
            "Easy" => Ok(Difficulty::Easy),
            "Normal" => Ok(Difficulty::Normal),
            "Hard" => Ok(Difficulty::Hard),
            "Harder" => Ok(Difficulty::Harder),
            "Insane" => Ok(Difficulty::Insane),
            "Demon" => Ok(Difficulty::Demon),
            s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "difficulty" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 8 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "NA" => true,
                    "Auto" => true,
                    "Easy" => true,
                    "Normal" => true,
                    "Hard" => true,
                    "Harder" => true,
                    "Insane" => true,
                    "Demon" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[allow(non_camel_case_types)]
pub enum DemonDifficulty {
    Easy,
    Medium,
    Hard,
    Insane,
    Extreme,
}
impl<'a> postgres_types::ToSql for DemonDifficulty {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let s = match *self {
            DemonDifficulty::Easy => "Easy",
            DemonDifficulty::Medium => "Medium",
            DemonDifficulty::Hard => "Hard",
            DemonDifficulty::Insane => "Insane",
            DemonDifficulty::Extreme => "Extreme",
        };
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "demon_difficulty" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 5 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "Easy" => true,
                    "Medium" => true,
                    "Hard" => true,
                    "Insane" => true,
                    "Extreme" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
    fn to_sql_checked(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
impl<'a> postgres_types::FromSql<'a> for DemonDifficulty {
    fn from_sql(
        ty: &postgres_types::Type,
        buf: &'a [u8],
    ) -> Result<DemonDifficulty, Box<dyn std::error::Error + Sync + Send>> {
        match std::str::from_utf8(buf)? {
            "Easy" => Ok(DemonDifficulty::Easy),
            "Medium" => Ok(DemonDifficulty::Medium),
            "Hard" => Ok(DemonDifficulty::Hard),
            "Insane" => Ok(DemonDifficulty::Insane),
            "Extreme" => Ok(DemonDifficulty::Extreme),
            s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "demon_difficulty" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 5 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "Easy" => true,
                    "Medium" => true,
                    "Hard" => true,
                    "Insane" => true,
                    "Extreme" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ItemType {
    Orbs,
    Coins,
    Stars,
}
impl<'a> postgres_types::ToSql for ItemType {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let s = match *self {
            ItemType::Orbs => "Orbs",
            ItemType::Coins => "Coins",
            ItemType::Stars => "Stars",
        };
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "item_type" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 3 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "Orbs" => true,
                    "Coins" => true,
                    "Stars" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
    fn to_sql_checked(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
impl<'a> postgres_types::FromSql<'a> for ItemType {
    fn from_sql(
        ty: &postgres_types::Type,
        buf: &'a [u8],
    ) -> Result<ItemType, Box<dyn std::error::Error + Sync + Send>> {
        match std::str::from_utf8(buf)? {
            "Orbs" => Ok(ItemType::Orbs),
            "Coins" => Ok(ItemType::Coins),
            "Stars" => Ok(ItemType::Stars),
            s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "item_type" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 3 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "Orbs" => true,
                    "Coins" => true,
                    "Stars" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[allow(non_camel_case_types)]
pub enum MessageSetting {
    All,
    FriendsOnly,
    None,
}
impl<'a> postgres_types::ToSql for MessageSetting {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let s = match *self {
            MessageSetting::All => "All",
            MessageSetting::FriendsOnly => "FriendsOnly",
            MessageSetting::None => "None",
        };
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "message_setting" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 3 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "All" => true,
                    "FriendsOnly" => true,
                    "None" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
    fn to_sql_checked(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
impl<'a> postgres_types::FromSql<'a> for MessageSetting {
    fn from_sql(
        ty: &postgres_types::Type,
        buf: &'a [u8],
    ) -> Result<MessageSetting, Box<dyn std::error::Error + Sync + Send>> {
        match std::str::from_utf8(buf)? {
            "All" => Ok(MessageSetting::All),
            "FriendsOnly" => Ok(MessageSetting::FriendsOnly),
            "None" => Ok(MessageSetting::None),
            s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "message_setting" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 3 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "All" => true,
                    "FriendsOnly" => true,
                    "None" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[allow(non_camel_case_types)]
pub enum CommentSetting {
    All,
    FriendsOnly,
    None,
}
impl<'a> postgres_types::ToSql for CommentSetting {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        buf: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let s = match *self {
            CommentSetting::All => "All",
            CommentSetting::FriendsOnly => "FriendsOnly",
            CommentSetting::None => "None",
        };
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "comment_setting" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 3 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "All" => true,
                    "FriendsOnly" => true,
                    "None" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
    fn to_sql_checked(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        postgres_types::__to_sql_checked(self, ty, out)
    }
}
impl<'a> postgres_types::FromSql<'a> for CommentSetting {
    fn from_sql(
        ty: &postgres_types::Type,
        buf: &'a [u8],
    ) -> Result<CommentSetting, Box<dyn std::error::Error + Sync + Send>> {
        match std::str::from_utf8(buf)? {
            "All" => Ok(CommentSetting::All),
            "FriendsOnly" => Ok(CommentSetting::FriendsOnly),
            "None" => Ok(CommentSetting::None),
            s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    }
    fn accepts(ty: &postgres_types::Type) -> bool {
        if ty.name() != "comment_setting" {
            return false;
        }
        match *ty.kind() {
            postgres_types::Kind::Enum(ref variants) => {
                if variants.len() != 3 {
                    return false;
                }
                variants.iter().all(|v| match &**v {
                    "All" => true,
                    "FriendsOnly" => true,
                    "None" => true,
                    _ => false,
                })
            }
            _ => false,
        }
    }
}
