use anyhow::{Context as _, Result};

/// The configuration of [`Node`](node::Node).
#[derive(serde::Deserialize)]
#[non_exhaustive]
pub(crate) struct Config {
	password: Option<String>,
	other_nodes: Option<std::collections::HashSet<OtherNode>>,
}

impl Config {
	common::accessor!(as_deref password -> Option<&str>);

	common::accessor!(
		as_ref other_nodes -> Option<&std::collections::HashSet<OtherNode>>
	);

	pub async fn load() -> Result<Self> {
		common::helpers::deserialize_json_from_file(
			crate::consts::CONFIG_PATH.as_path(),
		)
		.await
		.context("Failed to deserialize config from file.")
	}
}

#[derive(Clone, Eq, Hash, PartialEq, serde::Deserialize)]
pub(crate) struct OtherNode {
	address: std::net::SocketAddr,
	password: Option<String>,
}

impl OtherNode {
	common::accessor!(copy address -> std::net::SocketAddr);

	common::accessor!(as_deref password -> Option<&str>);
}

impl From<OtherNode> for (std::net::SocketAddr, Option<String>) {
	#[inline]
	#[must_use]
	fn from(n: OtherNode) -> (std::net::SocketAddr, Option<String>) {
		(n.address, n.password)
	}
}
