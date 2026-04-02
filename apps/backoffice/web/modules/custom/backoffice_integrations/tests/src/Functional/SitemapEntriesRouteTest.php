<?php

declare(strict_types=1);

namespace Drupal\Tests\backoffice_integrations\Functional;

use Drupal\Tests\BrowserTestBase;

/**
 * Verifies the authenticated sitemap entries endpoint is available.
 *
 * @group backoffice_integrations
 */
final class SitemapEntriesRouteTest extends BrowserTestBase {

  /**
   * {@inheritdoc}
   */
  protected $defaultTheme = 'stark';

  /**
   * {@inheritdoc}
   */
  protected static $modules = [
    'backoffice_integrations',
    'node',
  ];

  /**
   * Verifies the route responds with a JSON payload.
   */
  public function testRouteReturnsJsonResponse(): void {
    $user = $this->drupalCreateUser();
    $this->drupalLogin($user);
    $this->drupalGet('/api/sitemap/entries');
    $payload = json_decode((string) $this->getSession()->getPage()->getContent(), TRUE);

    $this->assertSession()->statusCodeEquals(200);
    $this->assertStringContainsString(
      'application/json',
      (string) $this->getSession()->getResponseHeader('Content-Type'),
    );
    $this->assertSame('1.0', $payload['jsonapi']['version'] ?? NULL);
    $this->assertSame([], $payload['data'] ?? NULL);
    $this->assertStringContainsString('/api/sitemap/entries', $payload['links']['self']['href'] ?? '');
  }

}
