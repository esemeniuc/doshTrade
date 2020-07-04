
export function getLocalItem<T>(key: string): T | null {
    let item = window.localStorage.getItem(key)
    return item ? JSON.parse(item) : null
}

export function setLocalItem<T>(key: string, value: T) {
    window.localStorage.setItem(key, JSON.stringify(value));
} 