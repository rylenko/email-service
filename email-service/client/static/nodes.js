function showOrHidePassword(node_id) {
	let block = $(`#${node_id}-password`);

	if (block.css("display") == "none") {
		block.css("display", "block");
	} else {
		block.css("display", "none");
	}
}
