<?php

declare(strict_types=1);

namespace Drupal\Tests\backoffice_integrations\Unit\Integration;

use Drupal\backoffice_integrations\Integration\WebsiteCachePurger;
use Drupal\Core\Logger\LoggerChannelFactoryInterface;
use Drupal\Core\Logger\LoggerChannelInterface;
use Drupal\Core\Messenger\MessengerInterface;
use Drupal\Core\Routing\AdminContext;
use Drupal\Core\Routing\RouteMatchInterface;
use Drupal\Core\Site\Settings;
use Drupal\Core\StringTranslation\TranslatableMarkup;
use Drupal\Tests\UnitTestCase;
use GuzzleHttp\ClientInterface;
use GuzzleHttp\Psr7\Response;
use Symfony\Component\Routing\Route;

/**
 * Tests the website cache purger integration.
 */
final class WebsiteCachePurgerTest extends UnitTestCase {

  /**
   * The mocked HTTP client.
   */
  private ClientInterface $httpClient;

  /**
   * The mocked logger factory.
   */
  private LoggerChannelFactoryInterface $loggerFactory;

  /**
   * The mocked logger channel.
   */
  private LoggerChannelInterface $loggerChannel;

  /**
   * The mocked messenger.
   */
  private MessengerInterface $messenger;

  /**
   * The mocked admin context.
   */
  private AdminContext $adminContext;

  /**
   * The mocked route match.
   */
  private RouteMatchInterface $currentRouteMatch;

  /**
   * {@inheritdoc}
   */
  protected function setUp(): void {
    parent::setUp();

    new Settings([]);

    $this->httpClient = $this->createMock(ClientInterface::class);
    $this->loggerFactory = $this->createMock(LoggerChannelFactoryInterface::class);
    $this->loggerChannel = $this->createMock(LoggerChannelInterface::class);
    $this->messenger = $this->createMock(MessengerInterface::class);
    $this->adminContext = $this->createMock(AdminContext::class);
    $this->currentRouteMatch = $this->createMock(RouteMatchInterface::class);
  }

  /**
   * Verifies the purger also runs in development environments.
   */
  public function testSuccessfulPurgeInDevelopment(): void {
    $purger = $this->createPurger([
      'backoffice_integrations_environment' => 'development',
      'backoffice_integrations_website_cache_purge_url' => 'http://website:3000/internal/cache/purge',
      'backoffice_integrations_website_cache_purge_token' => 'token',
    ]);

    $this->httpClient
      ->expects($this->once())
      ->method('request')
      ->with(
        'POST',
        'http://website:3000/internal/cache/purge',
        $this->callback(static function (array $options): bool {
          return $options['headers']['x-webhook-token'] === 'token'
            && $options['http_errors'] === FALSE
            && $options['timeout'] === 3.0
            && $options['connect_timeout'] === 1.5;
        }),
      )
      ->willReturn(new Response(200, [], 'Cache purged'));
    $this->loggerFactory->expects($this->never())->method('get');
    $this->messenger->expects($this->never())->method('addWarning');
    $this->messenger->expects($this->never())->method('addStatus');

    $purger->purgeWebsiteCache();
  }

  /**
   * Verifies missing configuration is logged and shown in admin routes.
   */
  public function testMissingConfigurationLogsAndWarnsInAdmin(): void {
    $purger = $this->createPurger(
      [
        'backoffice_integrations_environment' => 'prod',
        'backoffice_integrations_website_cache_purge_url' => '',
        'backoffice_integrations_website_cache_purge_token' => '',
      ],
      TRUE,
    );

    $this->httpClient->expects($this->never())->method('request');
    $this->loggerFactory
      ->expects($this->once())
      ->method('get')
      ->with('backoffice_integrations')
      ->willReturn($this->loggerChannel);
    $this->loggerChannel
      ->expects($this->once())
      ->method('warning')
      ->with(
        'Website cache purge skipped because the integration is not fully configured.',
        $this->callback(static function (array $context): bool {
          return $context['environment'] === 'prod'
            && str_contains($context['missing_settings'], 'backoffice_integrations_website_cache_purge_url')
            && str_contains($context['missing_settings'], 'backoffice_integrations_website_cache_purge_token');
        }),
      );
    $this->messenger
      ->expects($this->once())
      ->method('addWarning')
      ->with($this->callback(static function ($message): bool {
        return $message instanceof TranslatableMarkup
          && (string) $message === 'Website cache purge did not complete. Website content may still be stale.';
      }));

    $purger->purgeWebsiteCache();
  }

  /**
   * Verifies a successful purge sends the expected request.
   */
  public function testSuccessfulPurge(): void {
    $purger = $this->createPurger([
      'backoffice_integrations_environment' => 'staging',
      'backoffice_integrations_website_cache_purge_url' => 'http://website:3000/internal/cache/purge',
      'backoffice_integrations_website_cache_purge_token' => 'token',
    ]);

    $this->httpClient
      ->expects($this->once())
      ->method('request')
      ->with(
        'POST',
        'http://website:3000/internal/cache/purge',
        $this->callback(static function (array $options): bool {
          return $options['headers']['x-webhook-token'] === 'token'
            && $options['http_errors'] === FALSE
            && $options['timeout'] === 3.0
            && $options['connect_timeout'] === 1.5;
        }),
      )
      ->willReturn(new Response(200, [], 'Cache purged'));
    $this->loggerFactory->expects($this->never())->method('get');
    $this->messenger->expects($this->never())->method('addWarning');
    $this->messenger->expects($this->never())->method('addStatus');

    $purger->purgeWebsiteCache();
  }

  /**
   * Verifies successful purges show a status message in admin routes.
   */
  public function testSuccessfulPurgeShowsStatusInAdmin(): void {
    $purger = $this->createPurger(
      [
        'backoffice_integrations_environment' => 'staging',
        'backoffice_integrations_website_cache_purge_url' => 'http://website:3000/internal/cache/purge',
        'backoffice_integrations_website_cache_purge_token' => 'token',
      ],
      TRUE,
    );

    $this->httpClient
      ->expects($this->once())
      ->method('request')
      ->willReturn(new Response(200, [], 'Cache purged'));
    $this->loggerFactory->expects($this->never())->method('get');
    $this->messenger->expects($this->never())->method('addWarning');
    $this->messenger
      ->expects($this->once())
      ->method('addStatus')
      ->with($this->callback(static function ($message): bool {
        return $message instanceof TranslatableMarkup
          && (string) $message === 'Website cache purged successfully.';
      }));

    $purger->purgeWebsiteCache();
  }

  /**
   * Verifies targeted purges send the expected JSON payload.
   */
  public function testTargetedPurgeSendsPathsPayload(): void {
    $purger = $this->createPurger([
      'backoffice_integrations_environment' => 'staging',
      'backoffice_integrations_website_cache_purge_url' => 'http://website:3000/internal/cache/purge',
      'backoffice_integrations_website_cache_purge_token' => 'token',
    ]);

    $this->httpClient
      ->expects($this->once())
      ->method('request')
      ->with(
        'POST',
        'http://website:3000/internal/cache/purge',
        $this->callback(static function (array $options): bool {
          return $options['headers']['x-webhook-token'] === 'token'
            && $options['json'] === ['paths' => ['/en/articles/example', '/es/portfolio/item']]
            && $options['http_errors'] === FALSE
            && $options['timeout'] === 3.0
            && $options['connect_timeout'] === 1.5;
        }),
      )
      ->willReturn(new Response(200, [], 'Cache purged'));
    $this->loggerFactory->expects($this->never())->method('get');
    $this->messenger->expects($this->never())->method('addWarning');
    $this->messenger->expects($this->never())->method('addStatus');

    $purger->purgeWebsitePaths([
      '/en/articles/example',
      '/es/portfolio/item',
      '/en/articles/example',
    ]);
  }

  /**
   * Verifies targeted purges skip invalid paths without making a request.
   */
  public function testTargetedPurgeSkipsWhenNoValidPathsRemain(): void {
    $purger = $this->createPurger([
      'backoffice_integrations_environment' => 'staging',
      'backoffice_integrations_website_cache_purge_url' => 'http://website:3000/internal/cache/purge',
      'backoffice_integrations_website_cache_purge_token' => 'token',
    ]);

    $this->httpClient->expects($this->never())->method('request');
    $this->loggerFactory->expects($this->never())->method('get');
    $this->messenger->expects($this->never())->method('addWarning');
    $this->messenger->expects($this->never())->method('addStatus');

    $purger->purgeWebsitePaths(['', 'relative/path', '//invalid']);
  }

  /**
   * Verifies non-200 responses are logged and shown in admin routes.
   */
  public function testUnexpectedResponseLogsAndWarnsInAdmin(): void {
    $purger = $this->createPurger(
      [
        'backoffice_integrations_environment' => 'prod',
        'backoffice_integrations_website_cache_purge_url' => 'http://website:3000/internal/cache/purge',
        'backoffice_integrations_website_cache_purge_token' => 'token',
      ],
      TRUE,
    );

    $this->httpClient
      ->expects($this->once())
      ->method('request')
      ->willReturn(new Response(500, [], 'Purging failed'));
    $this->loggerFactory
      ->expects($this->once())
      ->method('get')
      ->with('backoffice_integrations')
      ->willReturn($this->loggerChannel);
    $this->loggerChannel
      ->expects($this->once())
      ->method('warning')
      ->with(
        'Website cache purge failed with an unexpected response.',
        $this->callback(static function (array $context): bool {
          return $context['environment'] === 'prod'
            && $context['endpoint'] === 'http://website:3000/internal/cache/purge'
            && $context['status_code'] === 500
            && $context['response_body'] === 'Purging failed';
        }),
      );
    $this->messenger->expects($this->once())->method('addWarning');

    $purger->purgeWebsiteCache();
  }

  /**
   * Verifies request exceptions are logged without showing non-admin messages.
   */
  public function testRequestExceptionLogsWithoutAdminMessage(): void {
    $purger = $this->createPurger([
      'backoffice_integrations_environment' => 'production',
      'backoffice_integrations_website_cache_purge_url' => 'http://website:3000/internal/cache/purge',
      'backoffice_integrations_website_cache_purge_token' => 'token',
    ]);

    $this->httpClient
      ->expects($this->once())
      ->method('request')
      ->willThrowException(new \RuntimeException('Connection refused'));
    $this->loggerFactory
      ->expects($this->once())
      ->method('get')
      ->with('backoffice_integrations')
      ->willReturn($this->loggerChannel);
    $this->loggerChannel
      ->expects($this->once())
      ->method('warning')
      ->with(
        'Website cache purge request failed.',
        $this->callback(static function (array $context): bool {
          return $context['environment'] === 'production'
            && $context['endpoint'] === 'http://website:3000/internal/cache/purge'
            && $context['exception_class'] === \RuntimeException::class
            && $context['exception_message'] === 'Connection refused';
        }),
      );
    $this->messenger->expects($this->never())->method('addWarning');

    $purger->purgeWebsiteCache();
  }

  /**
   * Creates a purger instance for the given settings.
   *
   * @param array<string, mixed> $settings
   *   The settings to inject.
   * @param bool $adminRoute
   *   Whether the current route should be considered admin.
   *
   * @return \Drupal\backoffice_integrations\Integration\WebsiteCachePurger
   *   The purger.
   */
  private function createPurger(array $settings, bool $adminRoute = FALSE): WebsiteCachePurger {
    new Settings($settings);

    $route = new Route('/admin/example');
    $route->setOption('_admin_route', $adminRoute);

    $this->currentRouteMatch
      ->method('getRouteObject')
      ->willReturn($adminRoute ? $route : NULL);
    $this->adminContext
      ->method('isAdminRoute')
      ->with($this->anything())
      ->willReturn($adminRoute);

    return new WebsiteCachePurger(
      $this->httpClient,
      $this->loggerFactory,
      $this->messenger,
      $this->adminContext,
      $this->currentRouteMatch,
    );
  }

}
