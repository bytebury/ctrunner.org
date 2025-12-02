function selectRace(race) {
	showTertiaryQuestions();

	race_id.value = race.id;
	race_name.value = race.name;
	distance_val.value = race.miles;
	distance_unit.value = "miles";
	race_date.value = race.start_date;

	results.classList.add("hidden");
}

function noRace() {
	showTertiaryQuestions();
	results.classList.add("hidden");
}

function enableRaceSearch() {
	secondary_questions.classList.remove("hidden");
	secondary_questions.classList.add("flex-col");

	secondary_questions.addEventListener("focusout", function (event) {
		if (secondary_questions.classList.contains("flex-col"))
			if (!secondary_questions.contains(event.relatedTarget)) {
				noRace();
			}
	});
}

function showTertiaryQuestions() {
	tertiary_questions.classList.remove("hidden");
	tertiary_questions.classList.add("flex-col");
}
