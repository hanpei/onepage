function init() {
  let theme = themeFromLocalStorage();
  document.querySelector('html').setAttribute('data-theme', theme);
  let addClass = theme === 'dark' ? 'icon-moon' : 'icon-sun';
  document.querySelector('#toggle-theme').classList.add(addClass);
  bindToggle();
}

function themeFromLocalStorage() {
  if (typeof window.localStorage !== 'undefined') {
    if (window.localStorage.getItem('theme') !== null) {
      return window.localStorage.getItem('theme');
    }
  }
  return 'dark'; //default
}

function setThemeToLocalStorage(theme) {
  if (typeof window.localStorage !== 'undefined') {
    window.localStorage.setItem('theme', theme);
  }
}

function bindToggle(params) {
  let btn = document.querySelector('#toggle-theme');
  btn.addEventListener('click', () => {
    let ele = document.querySelector('html');
    let theme = ele.getAttribute('data-theme');
    if (theme === 'dark') {
      ele.setAttribute('data-theme', 'light');
      btn.classList.remove('icon-moon');
      btn.classList.add('icon-sun');
      setThemeToLocalStorage('light');
    } else {
      ele.setAttribute('data-theme', 'dark');
      btn.classList.remove('icon-sun');
      btn.classList.add('icon-moon');
      setThemeToLocalStorage('dark');
    }
  });
}

init();
