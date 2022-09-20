use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden)]
enum User {
    Table,
    Id,
    Avatar,
    Bot,
    Discriminator,
    Name,
    Banner,
}

#[derive(Iden)]
enum Guild {
    Table,
    Id,
    //TODO
}

#[derive(Iden)]
enum Channel {
    Table,
    Id,
    //TODO
}

#[derive(Iden)]
enum Message {
    Table,
    Id,
    Author,
    Channel,
    Content,
    EditedTimestamp,
    Guild,
    Kind,
    MentionEveryone,
    MentionRoles,
    MentionChannels,
    Mentions,
    Nonce,
    Pinned,
    CreatedTimestamp,
    Tts,
    WebhookId,

    ActivityKind,
    ActivityPartyId,

    ApplicationId,
    ApplicationCoverImage,
    ApplicationDescription,
    ApplicationIcon,
    ApplicationName,

    MessageReferenceId,

    ReferencedMessageId,

    InteractionId,
    InteractionKind,
    InteractionName,
    InteractionUserId,
}

#[derive(Iden)]
enum Attachment {
    Table,
    Id,
    MessageId,
    Filename,
    Height,
    ProxyUrl,
    Size,
    Url,
    Width,
    ContentType,
    Ephemeral,
}

#[derive(Iden)]
enum Embed {
    Table,
    Id,
    MessageId,

    AuthorName,
    AuthorUrl,
    AuthorIconUrl,
    AuthorProxyIconUrl,

    Color,
    Description,

    FooterText,
    FooterIconUrl,
    FooterProxyIconUrl,

    ImageUrl,
    ImageProxyUrl,
    ImageHeight,
    ImageWidth,

    Kind,

    ProviderName,
    ProviderUrl,

    ThumbnailUrl,
    ThumbnailProxyUrl,
    ThumbnailHeight,
    ThumbnailWidth,

    Timestamp,
    Title,
    Url,

    VideoUrl,
    VideoHeight,
    VideoWidth,
}

#[derive(Iden)]
enum EmbedField {
    Table,
    Id,
    Name,
    Value,
    Inline,
    EmbedId,
}

#[derive(Iden)]
enum Reaction {
    Table,
    Id,
    MessageId,
    Count,
    Me,
    Unicode,
    EmojiId,
    EmojiName,
    EmojiAnimated,
}

#[derive(Iden)]
enum StickerItem {
    Table,
    Id,
    MessageId,
    Name,
    StickerFormatType,
}

#[derive(Iden)]
enum ActionRow {
    Table,
    Id,
    MessageId,
    ComponentType,
}

#[derive(Iden)]
enum ActionRowComponent {
    Table,
    Id,
    ActionRowId,

    ButtonKind,
    ButtonStyle,
    ButtonLabel,
    ButtonEmoji,
    ButtonCustomId,
    ButtonUrl,
    ButtonDisabled,

    SelectMenuKind,
    SelectMenuPlaceholder,
    SelectMenuCustomId,
    SelectMenuMinValues,
    SelectMenuMaxValues,
    SelectMenuValues,

    InputTextKind,
    InputTextCustomId,
    InputTextValue,
}

#[derive(Iden)]
enum SelectMenuOption {
    Table,
    Id,
    ActionRowComponentId,

    Label,
    Value,
    Description,
    Emoji,
    Default,
}

pub enum MessageType {
    /// A regular message.
    Regular = 0,
    /// An indicator that a recipient was added by the author.
    GroupRecipientAddition = 1,
    /// An indicator that a recipient was removed by the author.
    GroupRecipientRemoval = 2,
    /// An indicator that a call was started by the author.
    GroupCallCreation = 3,
    /// An indicator that the group name was modified by the author.
    GroupNameUpdate = 4,
    /// An indicator that the group icon was modified by the author.
    GroupIconUpdate = 5,
    /// An indicator that a message was pinned by the author.
    PinsAdd = 6,
    /// An indicator that a member joined the guild.
    MemberJoin = 7,
    /// An indicator that someone has boosted the guild.
    NitroBoost = 8,
    /// An indicator that the guild has reached nitro tier 1
    NitroTier1 = 9,
    /// An indicator that the guild has reached nitro tier 2
    NitroTier2 = 10,
    /// An indicator that the guild has reached nitro tier 3
    NitroTier3 = 11,
    /// An indicator that the channel is now following a news channel
    ChannelFollowAdd = 12,
    /// An indicator that the guild is disqualified for Discovery Feature
    GuildDiscoveryDisqualified = 14,
    /// An indicator that the guild is requalified for Discovery Feature
    GuildDiscoveryRequalified = 15,
    /// The first warning before guild discovery removal.
    GuildDiscoveryGracePeriodInitialWarning = 16,
    /// The last warning before guild discovery removal.
    GuildDiscoveryGracePeriodFinalWarning = 17,
    /// Message sent to inform users that a thread was created.
    ThreadCreated = 18,
    /// A message reply.
    InlineReply = 19,
    /// A slash command.
    ChatInputCommand = 20,
    /// A thread start message.
    ThreadStarterMessage = 21,
    /// Server setup tips.
    GuildInviteReminder = 22,
    /// A context menu command.
    ContextMenuCommand = 23,
    /// A message from an auto moderation action.
    AutoModerationAction = 24,
    /// An indicator that the message is of unknown type.
    Unknown = !0,
}

pub enum MessageActivityKind {
    JOIN,
    SPECTATE,
    LISTEN,
    JOIN_REQUEST,
    Unknown,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .big_unsigned()
                            .primary_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::Avatar).text().null())
                    .col(ColumnDef::new(User::Bot).boolean().not_null())
                    .col(ColumnDef::new(User::Discriminator).text().not_null())
                    .col(ColumnDef::new(User::Name).text().null())
                    .col(ColumnDef::new(User::Banner).text().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Guild::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Guild::Id)
                            .big_unsigned()
                            .primary_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Channel::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Channel::Id)
                            .big_unsigned()
                            .primary_key()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .col(
                        ColumnDef::new(Message::Id)
                            .big_unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Message::Author).big_unsigned().not_null())
                    .col(ColumnDef::new(Message::Channel).big_unsigned().not_null())
                    .col(ColumnDef::new(Message::Content).text().not_null())
                    .col(ColumnDef::new(Message::EditedTimestamp).timestamp().null())
                    .col(ColumnDef::new(Message::Guild).big_unsigned().null())
                    // .col(ColumnDef::new(Message::Kind).enumeration("kind", MessageType).not_null())
                    .col(
                        ColumnDef::new(Message::MentionEveryone)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Message::MentionRoles)
                            .array(String::from("bigint"))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Message::MentionChannels)
                            .array(String::from("bigint"))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Message::Mentions)
                            .array(String::from("bigint"))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Message::Nonce).big_unsigned().not_null())
                    .col(ColumnDef::new(Message::Pinned).boolean().not_null())
                    .col(
                        ColumnDef::new(Message::CreatedTimestamp)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Message::Tts).boolean().not_null())
                    .col(ColumnDef::new(Message::WebhookId).big_unsigned().null())
                    // .col(ColumnDef::new(Message::ActivityKind).enumeration(name, variants))
                    .col(
                        ColumnDef::new(Message::ActivityPartyId)
                            .big_unsigned()
                            .null(),
                    )
                    .col(ColumnDef::new(Message::ApplicationId).big_unsigned().null())
                    .col(ColumnDef::new(Message::ApplicationCoverImage).text().null())
                    .col(
                        ColumnDef::new(Message::ApplicationDescription)
                            .text()
                            .null(),
                    )
                    .col(ColumnDef::new(Message::ApplicationIcon).text().null())
                    .col(ColumnDef::new(Message::ApplicationName).text().null())
                    .col(
                        ColumnDef::new(Message::MessageReferenceId)
                            .big_unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Message::ReferencedMessageId)
                            .big_unsigned()
                            .null(),
                    )
                    .col(ColumnDef::new(Message::InteractionId).big_unsigned().null())
                    // .col(ColumnDef::new(Message::InteractionKind).enumeration().null())
                    .col(ColumnDef::new(Message::InteractionName).text().null())
                    .col(
                        ColumnDef::new(Message::InteractionUserId)
                            .big_unsigned()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_message_author")
                    .from(Message::Table, Message::Author)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_message_channel")
                    .from(Message::Table, Message::Channel)
                    .to(Channel::Table, Channel::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_message_guild")
                    .from(Message::Table, Message::Guild)
                    .to(Guild::Table, Guild::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // define attachment table
        manager
            .create_table(
                Table::create()
                    .table(Attachment::Table)
                    .col(
                        ColumnDef::new(Attachment::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Attachment::MessageId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Attachment::Filename).text().not_null())
                    .col(ColumnDef::new(Attachment::Size).big_unsigned().not_null())
                    .col(ColumnDef::new(Attachment::Url).text().not_null())
                    .col(ColumnDef::new(Attachment::ProxyUrl).text().not_null())
                    .col(ColumnDef::new(Attachment::Height).big_unsigned().null())
                    .col(ColumnDef::new(Attachment::Width).big_unsigned().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_attachment_message")
                    .from(Attachment::Table, Attachment::MessageId)
                    .to(Message::Table, Message::Id)
                    .to_owned(),
            )
            .await?;

        // define embed table
        manager
            .create_table(
                Table::create()
                    .table(Embed::Table)
                    .col(
                        ColumnDef::new(Embed::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Embed::MessageId).big_unsigned().not_null())
                    .col(ColumnDef::new(Embed::Title).text().null())
                    .col(ColumnDef::new(Embed::Description).text().null())
                    .col(ColumnDef::new(Embed::Url).text().null())
                    .col(ColumnDef::new(Embed::Timestamp).timestamp().null())
                    .col(ColumnDef::new(Embed::Color).big_unsigned().null())
                    .col(ColumnDef::new(Embed::FooterText).text().null())
                    .col(ColumnDef::new(Embed::FooterIconUrl).text().null())
                    .col(ColumnDef::new(Embed::FooterProxyIconUrl).text().null())
                    .col(ColumnDef::new(Embed::ImageUrl).text().null())
                    .col(ColumnDef::new(Embed::ImageProxyUrl).text().null())
                    .col(ColumnDef::new(Embed::ImageHeight).big_unsigned().null())
                    .col(ColumnDef::new(Embed::ImageWidth).big_unsigned().null())
                    .col(ColumnDef::new(Embed::ThumbnailUrl).text().null())
                    .col(ColumnDef::new(Embed::ThumbnailProxyUrl).text().null())
                    .col(ColumnDef::new(Embed::ThumbnailHeight).big_unsigned().null())
                    .col(ColumnDef::new(Embed::ThumbnailWidth).big_unsigned().null())
                    .col(ColumnDef::new(Embed::VideoUrl).text().null())
                    .col(ColumnDef::new(Embed::VideoHeight).big_unsigned().null())
                    .col(ColumnDef::new(Embed::VideoWidth).big_unsigned().null())
                    .col(ColumnDef::new(Embed::ProviderName).text().null())
                    .col(ColumnDef::new(Embed::ProviderUrl).text().null())
                    .col(ColumnDef::new(Embed::AuthorName).text().null())
                    .col(ColumnDef::new(Embed::AuthorUrl).text().null())
                    .col(ColumnDef::new(Embed::AuthorIconUrl).text().null())
                    .col(ColumnDef::new(Embed::AuthorProxyIconUrl).text().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_embed_message")
                    .from(Embed::Table, Embed::MessageId)
                    .to(Message::Table, Message::Id)
                    .to_owned(),
            )
            .await?;

        // define embed field table
        manager
            .create_table(
                Table::create()
                    .table(EmbedField::Table)
                    .col(
                        ColumnDef::new(EmbedField::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(EmbedField::EmbedId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(EmbedField::Name).text().not_null())
                    .col(ColumnDef::new(EmbedField::Value).text().not_null())
                    .col(ColumnDef::new(EmbedField::Inline).boolean().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_embed_field_embed")
                    .from(EmbedField::Table, EmbedField::EmbedId)
                    .to(Embed::Table, Embed::Id)
                    .to_owned(),
            )
            .await?;

        // define reaction table
        manager
            .create_table(
                Table::create()
                    .table(Reaction::Table)
                    .col(
                        ColumnDef::new(Reaction::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(Reaction::MessageId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Reaction::EmojiId).big_unsigned().null())
                    .col(ColumnDef::new(Reaction::EmojiName).text().null())
                    .col(ColumnDef::new(Reaction::EmojiAnimated).boolean().null())
                    .col(ColumnDef::new(Reaction::Count).big_unsigned().not_null())
                    .col(ColumnDef::new(Reaction::Me).boolean().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_reaction_message")
                    .from(Reaction::Table, Reaction::MessageId)
                    .to(Message::Table, Message::Id)
                    .to_owned(),
            )
            .await?;

        // define sticker item table
        manager
            .create_table(
                Table::create()
                    .table(StickerItem::Table)
                    .col(
                        ColumnDef::new(StickerItem::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(StickerItem::MessageId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(StickerItem::Name).text().not_null())
                    // .col(ColumnDef::new(StickerItem::StickerFormatType).enumeration().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_sticker_item_message")
                    .from(StickerItem::Table, StickerItem::MessageId)
                    .to(Message::Table, Message::Id)
                    .to_owned(),
            )
            .await?;

        // define action row table
        manager
            .create_table(
                Table::create()
                    .table(ActionRow::Table)
                    .col(
                        ColumnDef::new(ActionRow::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(ActionRow::MessageId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_action_row_message")
                    .from(ActionRow::Table, ActionRow::MessageId)
                    .to(Message::Table, Message::Id)
                    .to_owned(),
            )
            .await?;

        // define action row component
        manager
            .create_table(
                Table::create()
                    .table(ActionRowComponent::Table)
                    .col(
                        ColumnDef::new(ActionRowComponent::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(ActionRowComponent::ActionRowId)
                            .big_unsigned()
                            .not_null(),
                    )
                    // .col(ColumnDef::new(ActionRowComponent::ButtonKind).enumeration(name, variants).null())
                    .col(
                        ColumnDef::new(ActionRowComponent::ButtonStyle)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ActionRowComponent::ButtonLabel)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ActionRowComponent::ButtonEmoji)
                            .big_unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ActionRowComponent::ButtonCustomId)
                            .text()
                            .null(),
                    )
                    .col(ColumnDef::new(ActionRowComponent::ButtonUrl).text().null())
                    .col(
                        ColumnDef::new(ActionRowComponent::ButtonDisabled)
                            .boolean()
                            .null(),
                    )
                    // .col(ColumnDef::new(ActionRowComponent::SelectMenuKind).enumeration().null())
                    .col(
                        ColumnDef::new(ActionRowComponent::SelectMenuPlaceholder)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ActionRowComponent::SelectMenuCustomId)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ActionRowComponent::SelectMenuMinValues)
                            .big_unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ActionRowComponent::SelectMenuMaxValues)
                            .big_unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ActionRowComponent::SelectMenuValues)
                            .array(String::from("text"))
                            .null(),
                    )
                    // .col(ColumnDef::new(ActionRowComponent::InputTextKind).enumeration().null())
                    .col(
                        ColumnDef::new(ActionRowComponent::InputTextCustomId)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ActionRowComponent::InputTextValue)
                            .text()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_action_row_component_action_row")
                    .from(ActionRowComponent::Table, ActionRowComponent::ActionRowId)
                    .to(ActionRow::Table, ActionRow::Id)
                    .to_owned(),
            )
            .await?;

        // define select menu option table
        manager
            .create_table(
                Table::create()
                    .table(SelectMenuOption::Table)
                    .col(
                        ColumnDef::new(SelectMenuOption::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(SelectMenuOption::ActionRowComponentId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SelectMenuOption::Label).text().not_null())
                    .col(ColumnDef::new(SelectMenuOption::Value).text().not_null())
                    .col(ColumnDef::new(SelectMenuOption::Description).text().null())
                    .col(
                        ColumnDef::new(SelectMenuOption::Emoji)
                            .big_unsigned()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(SelectMenuOption::Default)
                            .boolean()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_select_menu_option_action_row_component")
                    .from(
                        SelectMenuOption::Table,
                        SelectMenuOption::ActionRowComponentId,
                    )
                    .to(ActionRowComponent::Table, ActionRowComponent::Id)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().if_exists().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Guild::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Channel::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Message::Table).to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(Attachment::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Embed::Table).to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(EmbedField::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(Reaction::Table).to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(StickerItem::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().if_exists().table(ActionRow::Table).to_owned())
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(ActionRowComponent::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(SelectMenuOption::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
