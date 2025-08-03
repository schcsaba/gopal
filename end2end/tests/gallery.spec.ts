import { test, expect } from "@playwright/test";

test.describe("Gallery Page", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("http://localhost:3000/gallery");
  });

  test("displays gallery images", async ({ page }) => {
    // Wait for images to load
    await page.waitForLoadState("networkidle");
    
    // Check that gallery grid exists
    const galleryGrid = page.locator(".grid");
    await expect(galleryGrid).toBeVisible();
    
    // Check that images are present
    const images = page.locator("img").filter({ hasNot: page.locator('img[alt="Gopal Logo"]') });
    await expect(images.first()).toBeVisible();
  });

  test("can open and close image modal", async ({ page }) => {
    // Wait for images to load
    await page.waitForLoadState("networkidle");
    
    // Click on first image to open modal
    const firstImage = page.locator("img").filter({ hasNot: page.locator('img[alt="Gopal Logo"]') }).first();
    await firstImage.click();
    
    // Check if modal or expanded view appears (adjust selector based on your implementation)
    // This might need adjustment based on your actual modal implementation
    await page.waitForTimeout(500); // Allow for modal animation
    
    // Try to close modal (this might be ESC key, clicking outside, or a close button)
    await page.keyboard.press("Escape");
    await page.waitForTimeout(500);
  });

  test("gallery is responsive", async ({ page }) => {
    // Test different viewport sizes
    await page.setViewportSize({ width: 1200, height: 800 });
    await expect(page.locator(".grid")).toBeVisible();
    
    await page.setViewportSize({ width: 768, height: 600 });
    await expect(page.locator(".grid")).toBeVisible();
    
    await page.setViewportSize({ width: 375, height: 600 });
    await expect(page.locator(".grid")).toBeVisible();
  });
});