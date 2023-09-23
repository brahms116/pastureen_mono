function main() {
  const searchInput = document.getElementById("search");
  const queryParams = new URLSearchParams(window.location.search);
  const search = queryParams.get("search");

  if (search) {
    searchInput.value = search;
    htmx.trigger('#search', 'customLoad');
  }
}

window.addEventListener("load", main);
