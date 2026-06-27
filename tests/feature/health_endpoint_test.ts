import { assertExists } from "@std/assert/exists";
import { url } from "./consts.ts";

Deno.test("Check health endpoint", async () => {
  const res = await fetch(url("/health"));
  const json = await res.json();

  assertExists(json);
});
