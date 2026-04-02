<?php

declare(strict_types=1);

namespace Drupal\backoffice_integrations\Integration;

use Drupal\Core\Logger\LoggerChannelFactoryInterface;
use Drupal\Core\Messenger\MessengerInterface;
use Drupal\Core\Routing\AdminContext;
use Drupal\Core\Routing\RouteMatchInterface;
use Drupal\Core\Site\Settings;
use Drupal\Core\StringTranslation\TranslatableMarkup;
use GuzzleHttp\ClientInterface;

/**
 * Purges the website cache through the configured integration endpoint.
 */
final class WebsiteCachePurger implements WebsiteCachePurgerInterface {

  private const SETTING_ENVIRONMENT = 'backoffice_integrations_environment';
  private const SETTING_PURGE_URL = 'backoffice_integrations_website_cache_purge_url';
  private const SETTING_PURGE_TOKEN = 'backoffice_integrations_website_cache_purge_token';
  private const SUCCESS_MESSAGE = 'Website cache purged successfully.';
  private const WARNING_MESSAGE = 'Website cache purge did not complete. Website content may still be stale.';

  /**
   * Constructs the website cache purger.
   */
  public function __construct(
    private readonly ClientInterface $httpClient,
    private readonly LoggerChannelFactoryInterface $loggerFactory,
    private readonly MessengerInterface $messenger,
    private readonly AdminContext $adminContext,
    private readonly RouteMatchInterface $currentRouteMatch,
  ) {
  }

  /**
   * Purges the website cache for the current environment.
   */
  public function purgeWebsiteCache(): void {
    $this->purgeRequest([]);
  }

  /**
   * Purges the website cache entries for the provided paths.
   *
   * @param string[] $paths
   *   The website paths to purge.
   */
  public function purgeWebsitePaths(array $paths): void {
    $paths = $this->normalizePaths($paths);

    if ($paths === []) {
      return;
    }

    $this->purgeRequest($paths);
  }

  /**
   * Sends the purge request to the website integration endpoint.
   *
   * @param string[] $paths
   *   The website paths to purge. An empty array purges the full cache.
   */
  private function purgeRequest(array $paths): void {
    $environment = $this->getEnvironment();
    $url = trim((string) Settings::get(self::SETTING_PURGE_URL, ''));
    $token = trim((string) Settings::get(self::SETTING_PURGE_TOKEN, ''));

    if (empty($url) || empty($token)) {
      $missing_settings = [];
      if ($url === '') {
        $missing_settings[] = self::SETTING_PURGE_URL;
      }
      if ($token === '') {
        $missing_settings[] = self::SETTING_PURGE_TOKEN;
      }

      $this->report('Website cache purge skipped because the integration is not fully configured.', [
        'environment' => $environment,
        'missing_settings' => implode(', ', $missing_settings),
      ]);
      return;
    }

    try {
      $options = [
        'headers' => [
          'x-webhook-token' => $token,
        ],
        'http_errors' => FALSE,
        'timeout' => 3.0,
        'connect_timeout' => 1.5,
      ];

      if ($paths !== []) {
        $options['json'] = ['paths' => $paths];
      }

      $response = $this->httpClient->request('POST', $url, $options);
    }
    catch (\Throwable $exception) {
      $context = [
        'environment' => $environment,
        'endpoint' => $url,
        'exception_class' => $exception::class,
        'exception_message' => $exception->getMessage(),
      ];

      if ($paths !== []) {
        $context['paths'] = implode(', ', $paths);
      }

      $this->report('Website cache purge request failed.', $context);
      return;
    }

    if ($response->getStatusCode() === 200) {
      if ($this->isAdminRequest()) {
        $this->messenger->addStatus(new TranslatableMarkup(self::SUCCESS_MESSAGE));
      }

      return;
    }

    $context = [
      'environment' => $environment,
      'endpoint' => $url,
      'status_code' => $response->getStatusCode(),
    ];

    if ($paths !== []) {
      $context['paths'] = implode(', ', $paths);
    }

    $response_body = $this->truncateBody((string) $response->getBody());

    if (!empty($response_body)) {
      $context['response_body'] = $response_body;
    }

    $this->report('Website cache purge failed with an unexpected response.', $context);
  }

  /**
   * Reports a warning and notifies admin users when applicable.
   *
   * @param string $message
   *   The message to log.
   * @param array<string, mixed> $context
   *   The log context.
   */
  private function report(string $message, array $context): void {
    $this->loggerFactory->get('backoffice_integrations')->warning($message, $context);

    if ($this->isAdminRequest()) {
      $this->messenger->addWarning(new TranslatableMarkup(self::WARNING_MESSAGE));
    }
  }

  /**
   * Returns whether the current request is for an admin route.
   */
  private function isAdminRequest(): bool {
    $route = $this->currentRouteMatch->getRouteObject();
    return $route !== NULL && $this->adminContext->isAdminRoute($route);
  }

  /**
   * Returns the normalized project environment.
   */
  private function getEnvironment(): string {
    return strtolower(trim((string) Settings::get(self::SETTING_ENVIRONMENT, '')));
  }

  /**
   * Truncates response bodies before logging them.
   */
  private function truncateBody(string $body): string {
    $body = trim($body);
    if ($body === '') {
      return '';
    }

    return substr($body, 0, 500);
  }

  /**
   * Normalizes the website paths before sending them to the purge endpoint.
   *
   * @param string[] $paths
   *   The input paths.
   *
   * @return string[]
   *   The normalized, unique paths.
   */
  private function normalizePaths(array $paths): array {
    $normalized = [];

    foreach ($paths as $path) {
      $path = trim((string) $path);

      if ($path === '' || !str_starts_with($path, '/') || str_starts_with($path, '//')) {
        continue;
      }

      $normalized[$path] = $path;
    }

    return array_values($normalized);
  }

}
