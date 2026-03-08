import { test, expect } from "@playwright/test";

test("website smoke flow covers portfolio, articles list, and article detail", async ({
  page,
}) => {
  await page.goto("http://localhost:3000/en");
  await expect(page.getByAltText("Tiagocode Logo")).toBeVisible();
  await expect(page.getByText("Made with love by Santiago Marulanda")).toBeVisible();

  await page.goto("http://localhost:3000/en/articles");
  await expect(page.getByText("My Tech Articles")).toBeVisible();
  await expect(page.getByRole("heading", { name: "Blog" })).toBeVisible();

  const firstArticleLink = page.locator('a[href^="/en/articles/"]').first();
  await expect(firstArticleLink).toBeVisible();

  const articlePath = await firstArticleLink.getAttribute("href");
  expect(articlePath).toBeTruthy();

  await page.goto(`http://localhost:3000${articlePath!}`);
  await expect(page.getByText("Article detail")).toBeVisible();
  await expect(page.locator(".article-detail")).toBeVisible();
});
