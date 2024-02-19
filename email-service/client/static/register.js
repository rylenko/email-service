"use strict";

function showOrHidePrivateKeyInput() {
	let block = $("#private-key-pem-base64-block");
	let input = $("#private-key-pem-base64");

	if (block.css("display") == "none") {
		block.css("display", "block");
		input.attr("disabled", false);
	} else {
		block.css("display", "none");
		input.attr("disabled", true);
	}
}
