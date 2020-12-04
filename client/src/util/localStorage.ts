export function getLocalItem<T>(key: string): T | null {
  const item = window.localStorage.getItem(key);
  let parsed = null;
  try {
    parsed = item ? JSON.parse(item) : null;
  } catch (err) {
    console.log("Local storage could not parse stuff inside! CLEARING")
    window.localStorage.clear();
  }
  return parsed;
}

export function setLocalItem<T>(key: string, value: T | null) {
  window.localStorage.setItem(key, JSON.stringify(value));
}
