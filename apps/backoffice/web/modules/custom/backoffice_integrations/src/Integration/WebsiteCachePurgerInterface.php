<?php

declare(strict_types=1);

namespace Drupal\backoffice_integrations\Integration;

/**
 * Defines the website cache purge integration contract.
 */
interface WebsiteCachePurgerInterface {

  /**
   * Purges the website cache through the configured integration endpoint.
   */
  public function purgeWebsiteCache(): void;

}
