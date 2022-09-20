use crate::database::DatabaseHandle;

pub trait DiscordDatabase {}

impl DiscordDatabase for DatabaseHandle {}
