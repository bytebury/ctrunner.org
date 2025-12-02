function attachDropdown(anchorElement) {
	const dropdown = anchorElement.querySelector(".dropdown");
	dropdown.style.width = anchorElement.getBoundingClientRect().width + "px";
}
