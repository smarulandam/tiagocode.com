<?php

declare(strict_types=1);

namespace Drupal\Tests\backoffice_integrations\Unit\Controller;

use Drupal\backoffice_integrations\Controller\WebsiteCachePurgeController;
use Drupal\backoffice_integrations\Integration\WebsiteCachePurgerInterface;
use Drupal\Tests\UnitTestCase;
use Symfony\Component\HttpFoundation\Request;
use Symfony\Component\HttpFoundation\RequestStack;

/**
 * Tests the website cache purge controller.
 *
 * @group backoffice_integrations
 */
final class WebsiteCachePurgeControllerTest extends UnitTestCase {

  /**
   * Verifies the controller purges the website cache and redirects back.
   */
  public function testFlushPurgesWebsiteCacheAndRedirectsToReferrer(): void {
    $request = Request::create('/admin/flush/website-cache');
    $request->server->set('HTTP_REFERER', 'http://localhost/admin/index');

    $requestStack = new RequestStack();
    $requestStack->push($request);

    $websiteCachePurger = $this->createMock(WebsiteCachePurgerInterface::class);
    $websiteCachePurger
      ->expects($this->once())
      ->method('purgeWebsiteCache');

    $controller = new WebsiteCachePurgeController($websiteCachePurger, $requestStack);
    $response = $controller->flush();

    $this->assertSame('http://localhost/admin/index', $response->getTargetUrl());
  }

  /**
   * Verifies the controller falls back to the site base path.
   */
  public function testFlushFallsBackToBasePath(): void {
    $requestStack = new RequestStack();
    $requestStack->push(Request::create('/admin/flush/website-cache'));

    $websiteCachePurger = $this->createMock(WebsiteCachePurgerInterface::class);
    $websiteCachePurger
      ->expects($this->once())
      ->method('purgeWebsiteCache');

    $controller = new WebsiteCachePurgeController($websiteCachePurger, $requestStack);
    $response = $controller->flush();

    $this->assertSame('/', $response->getTargetUrl());
  }

}
