use ribbon_cache::CACHE;
use ribbon_commands_core::{ Context, Error, Result, command };
use ribbon_emojis::Emoji;
use ribbon_models::ribbon::user::WebsiteQuickLinkModel;
use ribbon_syncing::SyncOperation;
use ribbon_util::WEBSITE_URL;
use twilight_model::channel::message::component::{ ActionRow, Button, ButtonStyle };

#[command(slash, context = "guild", description = "Acquire a quick-link to this server's Ribbon Dashboard.", default_member_permissions = "32")]
pub async fn dashboard(context: Context) -> Result<()> {
	let new_quick_link = WebsiteQuickLinkModel::new(context.author_id().unwrap(), context.guild_id());
	let website_url = format!("{WEBSITE_URL}/auth/quick_link#{}", new_quick_link.id);
	CACHE
		.ribbon
		.website_quick_links
		.insert(new_quick_link.id.clone(), new_quick_link);

	context.reply("")
		.components([ActionRow {
			components: vec![
				Button {
					custom_id: None,
					disabled: false,
					emoji: Some(Emoji::ArrowClockwise.into()),
					label: Some({
						let guild = CACHE
							.discord
							.guild(context.guild_id().unwrap())
							.unwrap();
						format!("Visit Dashboard for {}", guild.name)
					}),
					sku_id: None,
					style: ButtonStyle::Link,
					url: Some(website_url)
				}.into()
			]
		}.into()])
		.ephemeral()
		.await
}

#[command(slash, context = "guild", description = "Sync your server profile with the Roblox platform.")]
pub async fn sync(context: Context) -> Result<()> {
	SyncOperation::from_interaction(&context, false)
		.await
		.map_err(|x| {
			println!("{x}");
			Error::Unknown // temporary because fiofdasfosajfdlaefe.fqlergiqpteqw[twqkewqt]
		})?;

	Ok(())
}