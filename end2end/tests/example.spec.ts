import { test, expect } from "@playwright/test";

test("basic app smoke test", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  // Basic smoke test to ensure app loads
  await expect(page).toHaveTitle("Le Gopal Tours");
  await expect(page.locator("body")).toBeVisible();
});
