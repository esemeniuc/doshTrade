

export function displayStringForExpiryDate(exp: string): string {
    const options: Intl.DateTimeFormatOptions = { year: 'numeric', month: 'short', day: 'numeric' }
    const date = new Date(exp)
    const today = new Date()
    const tte = date.getTime() - today.getTime()
    const dte = Math.floor(tte / (1000 * 3600 * 24))
    return date.toLocaleDateString("en-US", options) + " " + "(" + dte + " days)"
}

