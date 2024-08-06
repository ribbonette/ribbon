use dashmap::{
	mapref::one::Ref,
	DashMap, DashSet
};
use ribbon_models::roblox::group::MembershipModel;

use crate::Result;

#[derive(Default)]
pub struct RobloxCache {
	memberships: DashMap<(u64, u64), MembershipModel>,
	user_memberships: DashMap<u64, DashSet<u64>>
}

impl RobloxCache {
	pub fn membership(&self, group_id: u64, user_id: u64) -> Option<Ref<'_, (u64, u64), MembershipModel>> {
		self.memberships.get(&(group_id, user_id))
	}

	pub async fn user_memberships(&self, user_id: u64) -> Result<Vec<u64>> {
		Ok(match self.user_memberships.get(&user_id) {
			Some(model) => model
				.iter()
				.map(|x| *x)
				.collect(),
			None => {
				let new_models = MembershipModel::get_user_many(user_id)
					.await?;
				let model_ids: Vec<_> = new_models
					.iter()
					.map(|x| x.group_id())
					.collect();
				for model in new_models {
					self.memberships.insert((model.group_id(), user_id), model);
				}

				self
					.user_memberships
					.entry(user_id)
					.or_default()
					.extend(model_ids.iter().copied());
				model_ids
			}
		})
	}
}