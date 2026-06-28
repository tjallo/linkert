import { assertEquals } from "@std/assert";
import { operations } from "../types.gen.ts";
import { url } from "./consts.ts";

type RegisterOk =
  operations["register"]["responses"][200]["content"]["application/json"];

type RegisterDuplicateUsername =
  operations["register"]["responses"][409]["content"]["application/json"];

type RegisterInvalidBody =
  operations["register"]["responses"][422]["content"]["application/json"];

type RegisterBody =
  operations["register"]["requestBody"]["content"]["application/json"];

const TEST_USERNAME = "test_user";
const TEST_PASSWORD = "hunter2hunter2hunter2";

function registerUser(username: string, password: string) {
  const body: RegisterBody = {
    username,
    password,
  };

  return fetch(url("/auth/register"), {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(body),
  });
}

Deno.test("POST /auth/register returns 200 with created username", async () => {
  const res = await registerUser(TEST_USERNAME, TEST_PASSWORD);
  const json: RegisterOk = await res.json();

  assertEquals(res.status, 200);
  assertEquals(json.success, true);
  assertEquals(json.data.username, TEST_USERNAME);
});

Deno.test("POST /auth/register duplicate username returns 409", async () => {
  const res = await registerUser(TEST_USERNAME, TEST_PASSWORD);
  const json: RegisterDuplicateUsername = await res.json();

  assertEquals(res.status, 409);
  assertEquals(json.success, false);
});

Deno.test("POST /auth/register empty username and password returns 422", async () => {
  const res = await registerUser("", "");
  const json: RegisterInvalidBody = await res.json();

  assertEquals(res.status, 422);
  assertEquals(json.success, false);
});

Deno.test("POST /auth/register username under 3 chars returns 422", async () => {
  const res = await registerUser("as", TEST_PASSWORD);
  const json: RegisterInvalidBody = await res.json();

  assertEquals(res.status, 422);
  assertEquals(json.success, false);
});

Deno.test("POST /auth/register password under 15 chars returns 422", async () => {
  const res = await registerUser("validuser", "tooshort");
  const json: RegisterInvalidBody = await res.json();

  assertEquals(res.status, 422);
  assertEquals(json.success, false);
});

Deno.test("POST /auth/register username with invalid characters returns 422", async () => {
  const res = await registerUser("user@name!", TEST_PASSWORD);
  const json: RegisterInvalidBody = await res.json();

  assertEquals(res.status, 422);
  assertEquals(json.success, false);
});

Deno.test("POST /auth/register username at exact min length (3 chars) returns 200", async () => {
  const res = await registerUser("abc", TEST_PASSWORD);
  const json: RegisterOk = await res.json();

  assertEquals(res.status, 200);
  assertEquals(json.success, true);
});
