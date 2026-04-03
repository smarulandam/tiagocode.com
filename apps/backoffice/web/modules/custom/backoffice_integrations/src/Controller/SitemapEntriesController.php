<?php

declare(strict_types=1);

namespace Drupal\backoffice_integrations\Controller;

use Drupal\Core\DependencyInjection\ContainerInjectionInterface;
use Drupal\Core\Entity\EntityTypeManagerInterface;
use Drupal\node\NodeInterface;
use Drupal\taxonomy\TermInterface;
use Symfony\Component\DependencyInjection\ContainerInterface;
use Symfony\Component\HttpFoundation\JsonResponse;
use Symfony\Component\HttpFoundation\RequestStack;

/**
 * Returns the public sitemap entries required to build the website sitemap.
 */
class SitemapEntriesController implements ContainerInjectionInterface {

  private const INCLUDED_NODE_TYPES = ['article', 'page', 'portfolio'];
  private const INCLUDED_TERM_VOCABULARIES = ['tags'];
  private const RESOURCE_TYPE = 'sitemap--entry';
  private const DEFAULT_SELF_PATH = '/jsonapi/nodes/sitemap';

  /**
   * Constructs the controller.
   */
  public function __construct(
    private readonly EntityTypeManagerInterface $entityTypeManager,
    private readonly RequestStack $requestStack,
  ) {
  }

  /**
   * {@inheritdoc}
   */
  public static function create(ContainerInterface $container): self {
    return new self(
      $container->get('entity_type.manager'),
      $container->get('request_stack'),
    );
  }

  /**
   * Returns the eligible aliases with their last modification date.
   */
  public function list(): JsonResponse {
    $pathAliasStorage = $this->entityTypeManager->getStorage('path_alias');
    $aliasIds = $pathAliasStorage
      ->getQuery()
      ->accessCheck(FALSE)
      ->execute();

    if ($aliasIds === []) {
      return new JsonResponse($this->jsonApiDocument([]));
    }

    $aliases = $pathAliasStorage->loadMultiple($aliasIds);
    $aliasMap = [];

    foreach ($aliases as $aliasEntity) {
      $aliasMap[$aliasEntity->getAlias()] = $aliasEntity->getPath();
    }

    $nodeStorage = $this->entityTypeManager->getStorage('node');
    $termStorage = $this->entityTypeManager->getStorage('taxonomy_term');
    $items = [];

    foreach ($aliases as $aliasEntity) {
      $alias = trim($aliasEntity->getAlias());

      if (!$this->isIncludedAlias($alias)) {
        continue;
      }

      $resolvedPath = $this->resolvePath(trim($aliasEntity->getPath()), $aliasMap);
      $item = $this->buildSitemapItem(
        $alias,
        $aliasEntity->language()->getId(),
        $resolvedPath,
        $nodeStorage,
        $termStorage,
      );

      if ($item === NULL) {
        continue;
      }

      $publicPath = $item['attributes']['path'] ?? $alias;

      if (in_array($publicPath, ['/portfolio/santiago-marulanda', '/es/portafolio/santiago-marulanda'], TRUE)) {
        continue;
      }

      $existingLastmod = $items[$publicPath]['attributes']['lastmod'] ?? '';
      $currentLastmod = $item['attributes']['lastmod'] ?? '';

      if (!isset($items[$publicPath]) || strcmp($currentLastmod, $existingLastmod) > 0) {
        $items[$publicPath] = $item;
      }
    }

    ksort($items);

    return new JsonResponse($this->jsonApiDocument(array_values($items)));
  }

  /**
   * Wraps the sitemap items using a JSON:API document.
   *
   * @param array<int, array<string, mixed>> $data
   *   The resources to expose.
   *
   * @return array<string, mixed>
   *   The JSON:API response payload.
   */
  private function jsonApiDocument(array $data): array {
    return [
      'jsonapi' => ['version' => '1.0'],
      'data' => $data,
      'links' => [
        'self' => [
          'href' => $this->requestStack->getCurrentRequest()?->getUri() ?? self::DEFAULT_SELF_PATH,
        ],
      ],
    ];
  }

  /**
   * Returns whether the alias should be considered for the sitemap.
   */
  private function isIncludedAlias(string $alias): bool {
    return $alias !== '' && str_starts_with($alias, '/');
  }

  /**
   * Resolves alias chains until the final system path is reached.
   *
   * @param array<string, string> $aliasMap
   *   An alias-to-path lookup map.
   */
  private function resolvePath(string $path, array $aliasMap): string {
    $visited = [];

    while ($path !== '' && isset($aliasMap[$path]) && !isset($visited[$path])) {
      $visited[$path] = TRUE;
      $path = $aliasMap[$path];
    }

    return $path;
  }

  /**
   * Builds the sitemap item when the resolved path points to eligible content.
   *
   * @param object $nodeStorage
   *   The node entity storage.
   * @param object $termStorage
   *   The taxonomy term entity storage.
   *
   * @return array<string, mixed>|null
   *   The sitemap item or NULL when the target is not eligible.
   */
  private function buildSitemapItem(
    string $alias,
    string $langcode,
    string $resolvedPath,
    object $nodeStorage,
    object $termStorage,
  ): ?array {
    $publicPath = $alias === '/en'
      ? '/'
      : ($langcode === 'es' && preg_match('@^/es(/|$)@', $alias) !== 1
      ? "/es{$alias}"
      : $alias);

    if (preg_match('@^/node/(\d+)$@', $resolvedPath, $matches) === 1) {
      $node = $nodeStorage->load((int) $matches[1]);

      if (!$node instanceof NodeInterface || !$node->isPublished()) {
        return NULL;
      }

      if (!in_array($node->bundle(), self::INCLUDED_NODE_TYPES, TRUE)) {
        return NULL;
      }

      $translation = $node;

      if ($langcode !== '' && $node->hasTranslation($langcode)) {
        $candidate = $node->getTranslation($langcode);

        if ($candidate->isPublished()) {
          $translation = $candidate;
        }
      }

      return [
        'id' => hash('sha256', $alias),
        'type' => self::RESOURCE_TYPE,
        'attributes' => [
          'path' => $publicPath,
          'lastmod' => gmdate(DATE_ATOM, $translation->getChangedTime()),
        ],
      ];
    }

    if (preg_match('@^/taxonomy/term/(\d+)$@', $resolvedPath, $matches) === 1) {
      $term = $termStorage->load((int) $matches[1]);

      if (!$term instanceof TermInterface) {
        return NULL;
      }

      if (!in_array($term->bundle(), self::INCLUDED_TERM_VOCABULARIES, TRUE)) {
        return NULL;
      }

      $translation = $term;

      if ($langcode !== '' && $term->hasTranslation($langcode)) {
        $translation = $term->getTranslation($langcode);
      }

      $lastmod = method_exists($translation, 'getChangedTime')
        ? gmdate(DATE_ATOM, $translation->getChangedTime())
        : NULL;

      return [
        'id' => hash('sha256', $alias),
        'type' => self::RESOURCE_TYPE,
        'attributes' => [
          'path' => $publicPath,
          'lastmod' => $lastmod,
        ],
      ];
    }

    return NULL;
  }

}
