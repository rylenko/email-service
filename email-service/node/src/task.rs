use anyhow::{Context as _, Result};

/// Every `common::consts::CHECK_OLD_EMAILS_INTERVAL` removes emails that are
/// older than `common:consts::EMAILS_MAX_AGE`.
pub(crate) async fn delete_old_emails_task(
	state: &crate::state::State,
) -> Result<()> {
	loop {
		tokio::time::sleep(common::consts::CHECK_OLD_EMAILS_INTERVAL).await;
		state
			.db()
			.delete_old_emails(common::consts::EMAILS_MAX_AGE)
			.await
			.context("Failed to delete old emails.")?;
		common::debug!(
			"Emails older than {} seconds were deleted.",
			common::consts::EMAILS_MAX_AGE.as_secs()
		);
	}
}
