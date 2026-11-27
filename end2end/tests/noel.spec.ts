import { test, expect } from "@playwright/test";

test.describe("Noël à emporter", () => {
  test("banner on other pages leads to the Christmas menu", async ({ page }) => {
    await page.goto("http://localhost:3000/menu");
    await page.click("text=Menu & bûches de Noël à emporter");
    await expect(page).toHaveURL("http://localhost:3000/noel");
    await expect(page.locator("h1")).toContainText("Noël à emporter");
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

  test("lists both bûches with their price", async ({ page }) => {
    await page.goto("http://localhost:3000/noel");
    const buches = page.locator("#buches");
    for (const name of ["Bûche Ferrero", "Bûche Tropicale"]) {
      await expect(buches.locator("article", { hasText: name })).toHaveCount(1);
    }
    for (const price of ["38 €", "58 €"]) {
      await expect(buches.getByText(price, { exact: true })).toHaveCount(2);
    }
    await expect(buches.getByText("8/10 personnes", { exact: true })).toHaveCount(2);
    await expect(buches.locator("article").getByText("(végane)")).toHaveCount(2);
    await expect(buches.getByText(/version sans gluten/)).toBeVisible();
    await expect(buches.getByRole("link", { name: /Commander une bûche/ })).toHaveAttribute(
      "href",
      /gopal\.sumupstore\.com\/produits\?category=/,
    );
    await expect(buches.getByRole("link", { name: "Commander la Bûche Ferrero" })).toHaveAttribute(
      "href",
      "https://gopal.sumupstore.com/product/buche-ferrero",
    );
    await expect(buches.getByRole("link", { name: "Commander la Bûche Tropicale" })).toHaveAttribute(
      "href",
      "https://gopal.sumupstore.com/product/buche-tropicale",
    );
  });

  for (const [name, opener] of [
    ["L’affiche du menu de Noël", '[aria-controls="affiche-menu"]'],
    ["L’affiche des bûches de Noël", '[aria-controls="affiche-buches"] >> nth=0'],
  ]) {
    test(`${name} opens over the page and closes again`, async ({ page }) => {
      await page.goto("http://localhost:3000/noel");
      const viewer = page.getByRole("dialog", { name });
      await page.click(opener);
      await expect(viewer).toBeVisible();
      await expect(page).toHaveURL("http://localhost:3000/noel");
      await viewer.getByRole("button", { name: /Fermer/ }).click();
      await expect(viewer).toBeHidden();
      await page.click(opener);
      await page.keyboard.press("Escape");
      await expect(viewer).toBeHidden();
    });
  }
});
