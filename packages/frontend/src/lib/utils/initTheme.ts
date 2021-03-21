export function initTheme() {
  if (
    localStorage.theme === 'dark' ||
    (!('theme' in localStorage) &&
      window.matchMedia('(prefers-color-scheme: dark)').matches)
  ) {
    document.querySelector('html')?.classList.add('dark');
    localStorage.theme = 'dark';
  } else {
    document.querySelector('html')?.classList.remove('dark');
    localStorage.theme = 'light';
  }
}

export function toggleTheme() {
  if (localStorage.theme === 'dark') {
    localStorage.theme = 'light';
    document.querySelector('html')?.classList.remove('dark');

    return 'light';
  } else {
    localStorage.theme = 'dark';
    document.querySelector('html')?.classList.add('dark');

    return 'dark';
  }
}
