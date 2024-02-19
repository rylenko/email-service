/// Structure with information about the current pagination for serialization
/// in HTML.
#[derive(serde::Serialize)]
pub(crate) struct Pagination<T> {
	current_page: std::num::NonZeroU64,
	pages: u64,
	items: Vec<T>,
	has_next_page: bool,
	has_previous_page: bool,
	next_page: u64,
	previous_page: u64,
}

impl<T> Pagination<T> {
	common::accessor!(& items -> &[T]);

	#[must_use]
	pub fn new(
		current_page: std::num::NonZeroU64,
		pages: u64,
		items: Vec<T>,
	) -> Self {
		let current_page_raw = u64::from(current_page);
		// To display on the first page that there is no `items`
		if current_page_raw != 1 {
			assert!(current_page_raw <= pages);
		}
		Self {
			current_page,
			pages,
			items,
			has_next_page: current_page_raw < pages,
			has_previous_page: current_page_raw > 1,
			next_page: current_page_raw + 1,
			previous_page: current_page_raw - 1,
		}
	}
}

#[derive(serde::Deserialize)]
pub(crate) struct Query {
	page: Option<std::num::NonZeroU64>,
}

impl Query {
	common::accessor!(copy page -> Option<std::num::NonZeroU64>);
}
