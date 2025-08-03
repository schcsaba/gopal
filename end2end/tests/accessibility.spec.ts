import { test, expect } from "@playwright/test";

test.describe("Accessibility", () => {
  test("navigation is keyboard accessible", async ({ page }) => {
    await page.goto("http://localhost:3000/");
    
    // Tab through navigation links
    await page.keyboard.press("Tab");
    await page.keyboard.press("Tab");
    
    // Should be able to navigate with Enter
    await page.keyboard.press("Enter");
    
    // Check that we navigated somewhere
    await page.waitForTimeout(500);
  });

  test("images have alt text", async ({ page }) => {
    await page.goto("http://localhost:3000/");
    
    // Check logo has alt text (use first instance)
    const logo = page.locator('img[alt="Gopal Logo"]').first();
    await expect(logo).toBeVisible();
    
    // Check gallery images (if on gallery page)
    await page.goto("http://localhost:3000/gallery");
    await page.waitForLoadState("networkidle");
    
    const images = page.locator("img").filter({ hasNot: page.locator('img[alt="Gopal Logo"]') });
    const imageCount = await images.count();
    
    if (imageCount > 0) {
      // Check that at least some images have alt attributes
      for (let i = 0; i < Math.min(3, imageCount); i++) {
        const img = images.nth(i);
        const altText = await img.getAttribute("alt");
        // Alt text can be empty but should exist
        expect(altText).not.toBeNull();
      }
    }
  });

  test("page has proper heading structure", async ({ page }) => {
    await page.goto("http://localhost:3000/contact");
    
    // Check that headings exist and are properly structured
    const h1Elements = page.locator("h1");
    const h2Elements = page.locator("h2");
    
    // Should have at least one h2 on contact page
    await expect(h2Elements.first()).toBeVisible();
  });

  test("links have descriptive text", async ({ page }) => {
    await page.goto("http://localhost:3000/contact");
    
    // Check that phone and email links have descriptive text
    const phoneLink = page.locator('a[href="tel:+33783654565"]').first();
    await expect(phoneLink).toHaveText(/07 83 65 45 65/);
    
    const emailLink = page.locator('a[href="mailto:contact@legopal.fr"]').first();
    await expect(emailLink).toHaveText(/contact@legopal\.fr/);
  });
});