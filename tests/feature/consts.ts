import type { paths } from "../types.gen.ts";

const BASE_URL = "http://localhost:3000";

export function url(path: keyof paths) {
    return `${BASE_URL}${path}`;
}
