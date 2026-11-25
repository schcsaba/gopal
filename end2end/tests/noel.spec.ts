import { test, expect } from "@playwright/test";

test.describe("Menu de Noël", () => {
  test("banner on other pages leads to the Christmas menu", async ({ page }) => {
    await page.goto("http://localhost:3000/menu");
    await page.click("text=Menu de Noël à emporter");
    await expect(page).toHaveURL("http://localhost:3000/noel");
    await expect(page.locator("h1")).toContainText("Menu de Noël");
    // No banner on its own page
    await expect(page.locator('a[href="/noel"]')).toHaveCount(0);
  });

  test("lists every course and links to the online shop", async ({ page }) => {
    await page.goto("http://localhost:3000/noel");
    for (const course of ["Les amuse-bouches", "Les entrées", "Les plats", "Les desserts"]) {
      await expect(page.getByRole("heading", { name: course })).toBeVisible();
    }
    const order = page.getByRole("link", { name: /Commander en ligne/ }).first();
    await expect(order).toHaveAttribute("href", "https://gopal.sumupstore.com/");
  });

  test("poster opens over the page and closes again", async ({ page }) => {
    await page.goto("http://localhost:3000/noel");
    const viewer = page.getByRole("dialog", { name: "L’affiche du menu de Noël" });
    await page.click("[data-lightbox-open]");
    await expect(viewer).toBeVisible();
    await expect(page).toHaveURL("http://localhost:3000/noel");
    await viewer.getByRole("button", { name: /Fermer/ }).click();
    await expect(viewer).toBeHidden();
    await page.click("[data-lightbox-open]");
    await page.keyboard.press("Escape");
    await expect(viewer).toBeHidden();
  });
});
