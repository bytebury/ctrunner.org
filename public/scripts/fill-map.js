function markTownComplete(townName) {
	document
		.getElementById("ct_towns")
		.querySelectorAll("path")
		.forEach(function (path) {
			if (townName.includes(path.id)) {
				path.classList.add("completed");
			}
		});
}
