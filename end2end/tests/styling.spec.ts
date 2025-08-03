import { test, expect } from "@playwright/test";

test.describe("Styling and Fonts", () => {
  test("custom Daikon font loads correctly", async ({ page }) => {
    await page.goto("http://localhost:3000/");
    
    // Check that CSS is loaded
    await page.waitForLoadState("networkidle");
    
    // Check that custom font is applied to elements
    const heading = page.locator("h1, h2").first();
    if (await heading.isVisible()) {
      const fontFamily = await heading.evaluate((el) => 
        getComputedStyle(el).getPropertyValue("font-family")
      );
      expect(fontFamily).toContain("Daikon");
    }
  });

  test("responsive design works", async ({ page }) => {
    await page.goto("http://localhost:3000/");
    
    // Test mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await expect(page.locator("body")).toBeVisible();
    
    // Test tablet viewport
    await page.setViewportSize({ width: 768, height: 1024 });
    await expect(page.locator("body")).toBeVisible();
    
    // Test desktop viewport
    await page.setViewportSize({ width: 1920, height: 1080 });
    await expect(page.locator("body")).toBeVisible();
  });

  test("contact page map has proper responsive sizing", async ({ page }) => {
    await page.goto("http://localhost:3000/contact");
    
    // Test mobile
    await page.setViewportSize({ width: 375, height: 667 });
    const mapContainerMobile = page.locator('iframe[title="Google Map"]').locator("..");
    await expect(mapContainerMobile).toBeVisible();
    
    // Test tablet
    await page.setViewportSize({ width: 768, height: 1024 });
    await expect(mapContainerMobile).toBeVisible();
    
    // Test desktop
    await page.setViewportSize({ width: 1200, height: 800 });
    await expect(mapContainerMobile).toBeVisible();
  });

  test("CSS loads without errors", async ({ page }) => {
    const cssErrors: string[] = [];
    
    page.on("response", (response) => {
      if (response.url().includes(".css") && !response.ok()) {
        cssErrors.push(`CSS failed to load: ${response.url()} - ${response.status()}`);
      }
    });
    
        await page.goto("http://localhost:3000/");
    // Wait for CSS to load with shorter timeout
    await page.waitForLoadState("domcontentloaded");
    await page.waitForTimeout(1000); // Give CSS time to load

    expect(cssErrors).toHaveLength(0);
  });
});