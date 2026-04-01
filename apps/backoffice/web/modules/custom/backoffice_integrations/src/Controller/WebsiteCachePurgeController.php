<?php

declare(strict_types=1);

namespace Drupal\backoffice_integrations\Controller;

use Drupal\backoffice_integrations\Integration\WebsiteCachePurgerInterface;
use Drupal\Core\DependencyInjection\ContainerInjectionInterface;
use Symfony\Component\DependencyInjection\ContainerInterface;
use Symfony\Component\HttpFoundation\RedirectResponse;
use Symfony\Component\HttpFoundation\RequestStack;

/**
 * Handles manual website cache purge actions.
 */
final class WebsiteCachePurgeController implements ContainerInjectionInterface {

  /**
   * Constructs the controller.
   */
  public function __construct(
    private readonly WebsiteCachePurgerInterface $websiteCachePurger,
    private readonly RequestStack $requestStack,
  ) {
  }

  /**
   * {@inheritdoc}
   */
  public static function create(ContainerInterface $container): self {
    return new self(
      $container->get('backoffice_integrations.website_cache_purger'),
      $container->get('request_stack'),
    );
  }

  /**
   * Purges the website cache and returns to the previous page.
   */
  public function flush(): RedirectResponse {
    $this->websiteCachePurger->purgeWebsiteCache();

    $request = $this->requestStack->getCurrentRequest();
    $target = $request?->server->get('HTTP_REFERER') ?: base_path();

    return new RedirectResponse($target);
  }

}
