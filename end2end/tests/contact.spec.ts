import { test, expect } from "@playwright/test";

test.describe("Contact Page", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("http://localhost:3000/contact");
  });

  test("displays contact information", async ({ page }) => {
    // Check opening hours section
    await expect(page.locator("text=Heures d'ouverture")).toBeVisible();
    await expect(page.locator("text=Mardi : 12h00 à 14h00")).toBeVisible();
    await expect(page.locator("text=Vendredi : 12h00 à 14h00 et 19h00 à 21h30")).toBeVisible();
    
    // Check contact details
    await expect(page.locator("text=Coordonnées")).toBeVisible();
    await expect(page.locator("text=8 Avenue du Mans, 37100 Tours, France")).toBeVisible();
    await expect(page.locator('a[href="tel:+33783654565"]').first()).toBeVisible();
    await expect(page.locator('a[href="mailto:contact@legopal.fr"]').first()).toBeVisible();
  });

  test("displays Google Maps iframe", async ({ page }) => {
    // Check that the map iframe is present and loaded
    const iframe = page.locator('iframe[title="Google Map"]');
    await expect(iframe).toBeVisible();
    await expect(iframe).toHaveAttribute("src", /google\.com\/maps/);
  });

  test("phone and email links are clickable", async ({ page }) => {
    // Test phone link (use first instance)
    const phoneLink = page.locator('a[href="tel:+33783654565"]').first();
    await expect(phoneLink).toBeVisible();
    await expect(phoneLink).toHaveText("07 83 65 45 65");
    
    // Test email link (use first instance)
    const emailLink = page.locator('a[href="mailto:contact@legopal.fr"]').first();
    await expect(emailLink).toBeVisible();
    await expect(emailLink).toHaveText("contact@legopal.fr");
  });

  test("displays transport information", async ({ page }) => {
    await expect(page.locator("text=Approche par les transports publics")).toBeVisible();
    await expect(page.locator("text=180m de l'arrêt de tram Tranchée")).toBeVisible();
    await expect(page.locator('a[href*="filbleu.fr"]')).toBeVisible();
  });
});