function selectRace(race) {
	showTertiaryQuestions();

	race_id.value = race.id;
	race_name.value = titleCase(race.name);
	distance_val.value = race.miles;
	distance_unit.value = "miles";
	race_date.value = race.start_date;
}

function selectedNoRace() {
	showTertiaryQuestions();
}

function enableRaceSearch() {
	showSecondaryQuestions();

	race_name.addEventListener("focus", function (event) {
		race_name.classList.add("opened");
	});

	secondary_questions.addEventListener("focusout", function (event) {
		if (secondary_questions.classList.contains("flex-col")) {
			if (!secondary_questions.contains(event.relatedTarget)) {
				selectedNoRace();
				race_name.classList.remove("opened");
			}
		}
	});
}

function showSecondaryQuestions() {
	secondary_questions.classList.remove("hidden");
	secondary_questions.classList.add("flex-col");
}

function showTertiaryQuestions() {
	tertiary_questions.classList.remove("hidden");
	tertiary_questions.classList.add("flex-col");
	results.classList.add("hidden");
}

function titleCase(str) {
	return str
		.toLowerCase()
		.split(" ")
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(" ");
}
