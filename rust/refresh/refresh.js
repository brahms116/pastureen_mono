
function focusGlobalSearch() {
  const searchInput = document.getElementById('global-search-input');
  if (!(searchInput instanceof HTMLInputElement)) {
    return
  }
  if (searchInput) {
    setTimeout(() => {
      searchInput.focus();
      const value = searchInput.value
      searchInput.value = ''
      searchInput.value = value
    },100)
  }
}

function clearGlobalSearch() {
  const searchInput = document.getElementById('global-search-input');
  if (!(searchInput instanceof HTMLInputElement)) {
    return
  }
  searchInput.value = '';
}
