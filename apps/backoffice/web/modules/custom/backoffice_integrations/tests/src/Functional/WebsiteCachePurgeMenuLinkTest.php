<?php

declare(strict_types=1);

namespace Drupal\Tests\backoffice_integrations\Functional;

use Drupal\Tests\BrowserTestBase;

/**
 * Tests the dedicated admin toolbar link for the website cache purge.
 *
 * @group backoffice_integrations
 */
final class WebsiteCachePurgeMenuLinkTest extends BrowserTestBase {

  /**
   * {@inheritdoc}
   */
  protected $defaultTheme = 'stark';

  /**
   * {@inheritdoc}
   */
  protected static $modules = [
    'admin_toolbar',
    'admin_toolbar_tools',
    'backoffice_integrations',
  ];

  /**
   * A user with access to the admin toolbar purge links.
   *
   * @var \Drupal\user\UserInterface
   */
  protected $adminUser;

  /**
   * {@inheritdoc}
   */
  protected function setUp(): void {
    parent::setUp();

    $this->adminUser = $this->drupalCreateUser([
      'access administration pages',
      'access toolbar',
      'administer site configuration',
    ]);
  }

  /**
   * Verifies the menu link is exposed and the route is protected by CSRF.
   */
  public function testWebsiteCacheMenuLinkAndRoute(): void {
    $assert = $this->assertSession();

    $this->drupalLogin($this->adminUser);
    $this->drupalGet('admin/index');

    $assert->elementExists(
      'xpath',
      '//div[@id="toolbar-item-administration-tray"]//a[contains(@href, "/admin/flush/website-cache?token=") and normalize-space()="Flush website cache"]',
    );

    $route = \Drupal::service('router.route_provider')->getRouteByName('backoffice_integrations.flush_website_cache');
    $this->assertSame('/admin/flush/website-cache', $route->getPath());
    $this->assertSame('administer site configuration', $route->getRequirement('_permission'));
    $this->assertSame('TRUE', $route->getRequirement('_csrf_token'));
    $this->assertTrue((bool) $route->getOption('_admin_route'));

    $this->click('a[href*="/admin/flush/website-cache?token="]');
    $assert->pageTextContains('Website cache purge did not complete. Website content may still be stale.');
  }

}
