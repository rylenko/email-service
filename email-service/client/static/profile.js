"use strict";

function showOrHide(block) {
	if (block.css("display") == "none") {
		block.css("display", "block");
	} else {
		block.css("display", "none");
	}
}

function showOrHidePublicKeyPemBase64() {
	showOrHide($("#public-key-pem-base64"));
}

function showOrHidePrivateKeyPemBase64() {
	showOrHide($("#private-key-pem-base64"));
}
