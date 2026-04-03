import { test, expect } from "@playwright/test";

test("website smoke flow covers portfolio, articles list, and article detail", async ({
  page,
}) => {
  await page.goto("http://localhost:3000/");
  await expect(page.getByAltText("Tiagocode Logo")).toBeVisible();
  await expect(page.getByText("Made with love by Santiago Marulanda")).toBeVisible();

  await page.goto("http://localhost:3000/articles");
  await expect(page.getByText("My Tech Articles")).toBeVisible();
  await expect(page.getByRole("heading", { name: "Blog" })).toBeVisible();

  const firstArticleLink = page.locator('a[href^="/articles/"]').first();
  await expect(firstArticleLink).toBeVisible();

  const articlePath = await firstArticleLink.getAttribute("href");
  expect(articlePath).toBeTruthy();

  await page.goto(`http://localhost:3000${articlePath!}`);
  await expect(page.getByText("Article detail")).toBeVisible();
  await expect(page.locator(".article-detail")).toBeVisible();
});

test("language switcher redirects to spanish home from english routes on desktop", async ({
  page,
}) => {
  await page.goto("http://localhost:3000/articles");

  const languageTrigger = page.getByRole("button", { name: "Change language" });
  await expect(languageTrigger).toBeVisible();

  await languageTrigger.click();
  await expect(page.getByRole("menu", { name: "Language options" })).toBeVisible();
  await expect(page.locator('[data-language-option="en"]')).toBeVisible();
  await expect(page.locator('[data-language-option="es"]')).toBeVisible();

  await Promise.all([
    page.waitForURL(/\/es$/),
    page.locator('[data-language-option="es"]').click(),
  ]);
});

test("language switcher stays visible on mobile and redirects to english home", async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("http://localhost:3000/es/articulos");

  const languageTrigger = page.getByRole("button", { name: "Change language" });
  const mobileMenuTrigger = page.locator('button[aria-controls="mobile-menu"]');

  await expect(languageTrigger).toBeVisible();
  await expect(mobileMenuTrigger).toBeVisible();

  await languageTrigger.click();
  await expect(page.getByRole("menu", { name: "Language options" })).toBeVisible();

  await page.keyboard.press("Escape");
  await expect(page.getByRole("menu", { name: "Language options" })).toBeHidden();

  await languageTrigger.click();

  await Promise.all([
    page.waitForURL("http://localhost:3000/"),
    page.locator('[data-language-option="en"]').click(),
  ]);
});

test("logo preserves spanish home when browsing spanish routes", async ({ page }) => {
  await page.goto("http://localhost:3000/es/articulos");

  await Promise.all([
    page.waitForURL("http://localhost:3000/es"),
    page.getByRole("link", { name: "Tiagocode Logo" }).click(),
  ]);
});
