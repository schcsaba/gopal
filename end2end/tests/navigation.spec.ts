import { test, expect } from "@playwright/test";

test.describe("Navigation", () => {
  test("homepage loads correctly", async ({ page }) => {
    await page.goto("http://localhost:3000/");
    
    // Check page title
    await expect(page).toHaveTitle("Le Gopal Tours");
    
    // Check main heading or key content exists (use first instance)
    await expect(page.locator("text=Le Gopal").first()).toBeVisible();
  });

  test("can navigate to all main pages", async ({ page }) => {
    await page.goto("http://localhost:3000/");
    
    // Test Menu page - click navigation link specifically
    await page.click("nav >> text=Menu");
    await expect(page).toHaveURL(/.*menu/);
    await expect(page.locator("text=Menu").first()).toBeVisible();
    
    // Test Reservation page
    await page.click("nav >> text=Réservation");
    await expect(page).toHaveURL(/.*reservation/);
    await expect(page.locator("text=Réservation").first()).toBeVisible();
    
    // Test Contact page
    await page.click("nav >> text=Contact");
    await expect(page).toHaveURL(/.*contact/);
    await expect(page.locator("text=Contact").first()).toBeVisible();
    
    // Test Gallery page (only available in footer)
    await page.click("footer >> text=Galerie");
    await expect(page).toHaveURL(/.*gallery/);
    
    // Test About page (only available in footer)
    await page.click("footer >> text=À propos");
    await expect(page).toHaveURL(/.*about/);
  });

  test("logo links back to homepage", async ({ page }) => {
    await page.goto("http://localhost:3000/menu");
    
    // Click on logo to return home (use first instance)
    await page.click('img[alt="Gopal Logo"]', { force: true });
    await expect(page).toHaveURL("http://localhost:3000/");
  });
});