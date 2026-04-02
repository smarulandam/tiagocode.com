<?php

declare(strict_types=1);

namespace Drupal\Tests\backoffice_integrations\Unit\Integration;

use Drupal\backoffice_integrations\Integration\WebsiteCacheNodeInvalidator;
use Drupal\backoffice_integrations\Integration\WebsiteCachePurgerInterface;
use Drupal\Core\Entity\EntityStorageInterface;
use Drupal\Core\Entity\EntityTypeManagerInterface;
use Drupal\Core\Entity\Query\QueryInterface;
use Drupal\Core\Language\LanguageInterface;
use Drupal\node\NodeInterface;
use Drupal\path_alias\PathAliasInterface;
use Drupal\Tests\UnitTestCase;

/**
 * Tests the node-specific website cache invalidator.
 */
final class WebsiteCacheNodeInvalidatorTest extends UnitTestCase {

  /**
   * Verifies published node updates purge old and new aliases.
   */
  public function testPublishedUpdatePurgesOldAndNewAliases(): void {
    $purger = $this->createMock(WebsiteCachePurgerInterface::class);
    $purger
      ->expects($this->once())
      ->method('purgeWebsitePaths')
      ->with(['/en/articles/old-slug', '/en/articles/new-slug']);

    $storage = $this->createPathAliasStorage([
      ['1' => $this->createAlias('/en/articles/old-slug', 'en')],
      ['1' => $this->createAlias('/en/articles/old-slug', 'en')],
      ['2' => $this->createAlias('/en/articles/new-slug', 'en')],
    ]);

    $invalidator = new WebsiteCacheNodeInvalidator(
      $this->createEntityTypeManager($storage),
      $purger,
    );

    $original = $this->createNode('7', 'article', 'en', TRUE);
    $node = $this->createNode('7', 'article', 'en', TRUE, $original);

    $invalidator->captureNodeUpdateState($node);
    $invalidator->invalidateUpdatedNode($node);
  }

  /**
   * Verifies unpublishing a node purges its previously public aliases.
   */
  public function testUnpublishPurgesPreviouslyPublicAliases(): void {
    $purger = $this->createMock(WebsiteCachePurgerInterface::class);
    $purger
      ->expects($this->once())
      ->method('purgeWebsitePaths')
      ->with(['/en/articles/old-slug', '/es/articles/slug-es']);

    $storage = $this->createPathAliasStorage([
      [
        '1' => $this->createAlias('/en/articles/old-slug', 'en'),
      ],
      [
        '1' => $this->createAlias('/en/articles/old-slug', 'en'),
        '2' => $this->createAlias('/es/articles/slug-es', 'es'),
      ],
    ]);

    $invalidator = new WebsiteCacheNodeInvalidator(
      $this->createEntityTypeManager($storage),
      $purger,
    );

    $original = $this->createNode('7', 'article', 'en', TRUE, NULL, [
      'en' => $this->createNode('7', 'article', 'en', TRUE),
      'es' => $this->createNode('7', 'article', 'es', TRUE),
    ]);
    $node = $this->createNode('7', 'article', 'en', FALSE, $original);

    $invalidator->captureNodeUpdateState($node);
    $invalidator->invalidateUpdatedNode($node);
  }

  /**
   * Verifies deleting a tracked node purges the captured public aliases.
   */
  public function testDeletePurgesCapturedAliases(): void {
    $purger = $this->createMock(WebsiteCachePurgerInterface::class);
    $purger
      ->expects($this->once())
      ->method('purgeWebsitePaths')
      ->with(['/en/portfolio/project']);

    $storage = $this->createPathAliasStorage([
      ['1' => $this->createAlias('/en/portfolio/project', 'en')],
    ]);

    $invalidator = new WebsiteCacheNodeInvalidator(
      $this->createEntityTypeManager($storage),
      $purger,
    );

    $node = $this->createNode('9', 'portfolio', 'en', TRUE);

    $invalidator->captureNodeDeleteState($node);
    $invalidator->invalidateDeletedNode($node);
  }

  /**
   * Creates an entity type manager mock for the provided alias storage.
   */
  private function createEntityTypeManager(EntityStorageInterface $storage): EntityTypeManagerInterface {
    $entityTypeManager = $this->createMock(EntityTypeManagerInterface::class);
    $entityTypeManager
      ->method('getStorage')
      ->with('path_alias')
      ->willReturn($storage);

    return $entityTypeManager;
  }

  /**
   * Creates a path alias storage mock with consecutive query results.
   *
   * @param array<int, array<string, \Drupal\path_alias\PathAliasInterface>> $results
   *   The alias results returned by consecutive calls.
   */
  private function createPathAliasStorage(array $results): EntityStorageInterface {
    $storage = $this->createMock(EntityStorageInterface::class);
    $query = $this->createMock(QueryInterface::class);

    $storage
      ->expects($this->exactly(count($results)))
      ->method('getQuery')
      ->willReturn($query);
    $query
      ->expects($this->exactly(count($results)))
      ->method('accessCheck')
      ->with(FALSE)
      ->willReturnSelf();
    $query
      ->method('condition')
      ->willReturnSelf();
    $query
      ->expects($this->exactly(count($results)))
      ->method('execute')
      ->willReturnOnConsecutiveCalls(...array_map('array_keys', $results));
    $storage
      ->expects($this->exactly(count($results)))
      ->method('loadMultiple')
      ->willReturnOnConsecutiveCalls(...$results);

    return $storage;
  }

  /**
   * Creates a node mock for the test scenario.
   *
   * @param array<string, \Drupal\node\NodeInterface> $translations
   *   Additional translations available on the node.
   */
  private function createNode(
    int|string $id,
    string $bundle,
    string $langcode,
    bool $published,
    ?NodeInterface $original = NULL,
    array $translations = [],
  ): NodeInterface {
    $node = $this->createMock(NodeInterface::class);
    $language = $this->createMock(LanguageInterface::class);
    $language
      ->method('getId')
      ->willReturn($langcode);

    $node
      ->method('bundle')
      ->willReturn($bundle);
    $node
      ->method('id')
      ->willReturn($id);
    $node
      ->method('isNew')
      ->willReturn(FALSE);
    $node
      ->method('language')
      ->willReturn($language);
    $node
      ->method('isPublished')
      ->willReturn($published);
    $node
      ->method('getOriginal')
      ->willReturn($original);
    $node
      ->method('hasTranslation')
      ->willReturnCallback(static function (string $requestedLangcode) use ($translations): bool {
        return isset($translations[$requestedLangcode]);
      });
    $node
      ->method('getTranslation')
      ->willReturnCallback(static function (string $requestedLangcode) use ($translations): NodeInterface {
        return $translations[$requestedLangcode];
      });

    return $node;
  }

  /**
   * Creates a path alias mock for the provided alias and language.
   */
  private function createAlias(string $alias, string $langcode): PathAliasInterface {
    $pathAlias = $this->createMock(PathAliasInterface::class);
    $language = $this->createMock(LanguageInterface::class);
    $language
      ->method('getId')
      ->willReturn($langcode);

    $pathAlias
      ->method('getAlias')
      ->willReturn($alias);
    $pathAlias
      ->method('language')
      ->willReturn($language);

    return $pathAlias;
  }

}
