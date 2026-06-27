const BASE_URL = "http://localhost:3000"

export function url(path: string) {
    return `${BASE_URL}${path}`;
}