function toggleTheme() {
	const theme = document.documentElement.getAttribute("data-theme");

	if (theme === "light") {
		document.documentElement.setAttribute("data-theme", "dark");
		localStorage.setItem("theme", "dark");
	} else {
		document.documentElement.setAttribute("data-theme", "light");
		localStorage.setItem("theme", "light");
	}
}

function loadThemeForGuests() {
	const theme = document.documentElement.getAttribute("data-theme");

	if (!theme) {
		document.documentElement.setAttribute("data-theme", localStorage.getItem("theme") || "light");
	}
}

loadThemeForGuests();
