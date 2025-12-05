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

function addMapInteractivity() {
	document.querySelectorAll("#ct_towns path").forEach(function (path) {
		path.addEventListener("mouseenter", (e) => {
			tooltip.textContent = path.id;
			tooltip.style.opacity = 1;
			updateTooltipPosition(e);
		});

		path.addEventListener("mousemove", (e) => {
			updateTooltipPosition(e);
		});

		path.addEventListener("mouseleave", () => {
			tooltip.style.opacity = 0;
		});
	});
}

function updateTooltipPosition(e) {
	const padding = 10;
	const tooltipRect = tooltip.getBoundingClientRect();
	let x = e.pageX + padding;
	let y = e.pageY + padding;

	if (x + tooltipRect.width > window.pageXOffset + window.innerWidth) {
		x = e.pageX - tooltipRect.width - padding;
	}

	if (y + tooltipRect.height > window.pageYOffset + window.innerHeight) {
		y = e.pageY - tooltipRect.height - padding;
	}

	tooltip.style.left = x + "px";
	tooltip.style.top = y + "px";
}
