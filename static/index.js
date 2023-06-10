const inputQuery = document.getElementById("input-query");
const inputLanguage = document.getElementById("input-language");
const searchButton = document.getElementById("button-search");

const search = () => {
  const limit = null;
  const query = inputQuery.value;
  const language = inputLanguage.value.length > 0 ? inputLanguage.value : undefined;

  const params = new URLSearchParams();
  if (query) params.append("query", query);
  if (limit) params.append("limit", limit);
  if (language) params.append("language", language);

  window.location.assign("/?" + params.toString());
};

searchButton.onclick = () => {
  search();
};

inputQuery.onkeyup = (event) => {
  switch (event.key) {
    case "Enter":
      search();
      break;
    case "Escape":
      inputQuery.value = "";
      break;
  }
};

inputLanguage.onkeyup = (event) => {
  switch (event.key) {
    case "Enter":
      search();
      break;
    case "Escape":
      inputLanguage.value = "";
      break;
  }
};

window.onkeyup = (event) => {
  if (!inputQuery.value && !inputLanguage.value) return;
  if (document.activeElement === inputQuery || document.activeElement === inputLanguage) return;
  switch (event.key) {
    case "Escape":
      inputQuery.value = "";
      inputLanguage.value = "";
      search();
      break;
  }
};
