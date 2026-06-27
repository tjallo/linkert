import { assertEquals } from "@std/assert";
import { operations } from "../types.gen.ts";
import { url } from "./consts.ts";

type HealthOk =
  operations["get_health"]["responses"][200]["content"]["application/json"];

Deno.test("GET /health returns 200 with healthy: true", async () => {
  const res = await fetch(url("/health"));
  const json: HealthOk = await res.json();

  assertEquals(res.status, 200);
  assertEquals(json.success, true);
  assertEquals(json.data.healthy, true);
});
