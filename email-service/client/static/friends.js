function showOrHidePublicKey(friend_id) {
	let block = $(`#${friend_id}-public-key`);

	if (block.css("display") == "none") {
		block.css("display", "block");
	} else {
		block.css("display", "none");
	}
}
