<?php

declare(strict_types=1);

namespace Drupal\backoffice_integrations\Integration;

use Drupal\Core\Entity\EntityInterface;
use Drupal\Core\Entity\EntityStorageInterface;
use Drupal\Core\Entity\EntityTypeManagerInterface;
use Drupal\node\NodeInterface;
use Drupal\path_alias\PathAliasInterface;

/**
 * Collects affected node aliases and triggers targeted website cache purges.
 */
final class WebsiteCacheNodeInvalidator {

  /**
   * Node bundles that can invalidate the website detail cache.
   */
  private const INCLUDED_NODE_TYPES = ['article', 'page', 'portfolio'];

  /**
   * Tracked update state per node translation.
   *
   * @var array<string, array{was_published: bool, old_translation_aliases: string[], old_public_aliases: string[]}>
   */
  private array $updateStates = [];

  /**
   * Tracked delete state per node.
   *
   * @var array<string, string[]>
   */
  private array $deleteStates = [];

  /**
   * Constructs the node invalidator.
   */
  public function __construct(
    private readonly EntityTypeManagerInterface $entityTypeManager,
    private readonly WebsiteCachePurgerInterface $websiteCachePurger,
  ) {
  }

  /**
   * Captures the alias state before a tracked node update.
   */
  public function captureNodeUpdateState(EntityInterface $entity): void {
    if (!$this->isTrackedNode($entity) || $entity->isNew()) {
      return;
    }

    $node = $entity;
    $original = $node->getOriginal();

    if (!$original instanceof NodeInterface || $original->id() === NULL) {
      return;
    }

    $langcode = $this->getActiveLangcode($node);
    $translation = $this->getBestTranslation($original, $langcode);

    $this->updateStates[$this->buildUpdateStateKey($node)] = [
      'was_published' => $translation->isPublished(),
      'old_translation_aliases' => $this->loadAliasesByNodeId($original->id(), $langcode),
      'old_public_aliases' => $this->loadPublicAliases($original),
    ];
  }

  /**
   * Purges affected aliases after a tracked node update.
   */
  public function invalidateUpdatedNode(EntityInterface $entity): void {
    if (!$this->isTrackedNode($entity) || $entity->id() === NULL) {
      return;
    }

    $node = $entity;
    $key = $this->buildUpdateStateKey($node);
    $state = $this->updateStates[$key] ?? NULL;
    unset($this->updateStates[$key]);

    if ($state === NULL) {
      return;
    }

    $translation = $this->getBestTranslation($node, $this->getActiveLangcode($node));
    $paths = [];

    if ($translation->isPublished()) {
      $paths = array_merge(
        $state['old_translation_aliases'],
        $this->loadAliasesByNodeId($node->id(), $this->getActiveLangcode($node)),
      );
    }
    elseif ($state['was_published']) {
      $paths = $state['old_public_aliases'];
    }

    $this->websiteCachePurger->purgeWebsitePaths($paths);
  }

  /**
   * Captures the public aliases before a tracked node deletion.
   */
  public function captureNodeDeleteState(EntityInterface $entity): void {
    if (!$this->isTrackedNode($entity) || $entity->id() === NULL) {
      return;
    }

    $this->deleteStates[$this->buildDeleteStateKey($entity)] = $this->loadPublicAliases($entity);
  }

  /**
   * Purges previously captured aliases after a tracked node deletion.
   */
  public function invalidateDeletedNode(EntityInterface $entity): void {
    if (!$this->isTrackedNode($entity)) {
      return;
    }

    $key = $this->buildDeleteStateKey($entity);
    $paths = $this->deleteStates[$key] ?? [];
    unset($this->deleteStates[$key]);

    $this->websiteCachePurger->purgeWebsitePaths($paths);
  }

  /**
   * Returns whether the entity is a tracked node bundle.
   */
  private function isTrackedNode(EntityInterface $entity): bool {
    return $entity instanceof NodeInterface
      && in_array($entity->bundle(), self::INCLUDED_NODE_TYPES, TRUE);
  }

  /**
   * Loads all current aliases for a node, optionally filtered by language.
   *
   * @param int|string $nodeId
   *   The node ID.
   *
   * @return string[]
   *   The matching aliases.
   */
  private function loadAliasesByNodeId(int|string $nodeId, ?string $langcode = NULL): array {
    $nodeId = (string) $nodeId;

    $query = $this->pathAliasStorage()
      ->getQuery()
      ->accessCheck(FALSE)
      ->condition('path', "/node/{$nodeId}");

    if ($langcode !== NULL && $langcode !== '') {
      $query->condition('langcode', $langcode);
    }

    $aliasIds = $query->execute();

    if ($aliasIds === []) {
      return [];
    }

    $aliases = [];

    foreach ($this->pathAliasStorage()->loadMultiple($aliasIds) as $aliasEntity) {
      if ($aliasEntity instanceof PathAliasInterface) {
        $aliases[$aliasEntity->getAlias()] = $aliasEntity->getAlias();
      }
    }

    return array_values($aliases);
  }

  /**
   * Loads all public aliases currently attached to the node.
   *
   * @return string[]
   *   The public aliases.
   */
  private function loadPublicAliases(NodeInterface $node): array {
    if ($node->id() === NULL) {
      return [];
    }

    $query = $this->pathAliasStorage()
      ->getQuery()
      ->accessCheck(FALSE)
      ->condition('path', "/node/{$node->id()}");
    $aliasIds = $query->execute();

    if ($aliasIds === []) {
      return [];
    }

    $aliases = [];

    foreach ($this->pathAliasStorage()->loadMultiple($aliasIds) as $aliasEntity) {
      if (!$aliasEntity instanceof PathAliasInterface) {
        continue;
      }

      $langcode = $aliasEntity->language()->getId();

      if ($this->getBestTranslation($node, $langcode)->isPublished()) {
        $aliases[$aliasEntity->getAlias()] = $aliasEntity->getAlias();
      }
    }

    return array_values($aliases);
  }

  /**
   * Returns the most suitable translation for the provided language.
   */
  private function getBestTranslation(NodeInterface $node, string $langcode): NodeInterface {
    if ($langcode !== '' && $node->hasTranslation($langcode)) {
      $translation = $node->getTranslation($langcode);

      if ($translation instanceof NodeInterface) {
        return $translation;
      }
    }

    return $node;
  }

  /**
   * Builds the in-request state key for a node update.
   */
  private function buildUpdateStateKey(NodeInterface $node): string {
    return implode(':', [
      'update',
      (string) $node->id(),
      $this->getActiveLangcode($node),
    ]);
  }

  /**
   * Builds the in-request state key for a node deletion.
   */
  private function buildDeleteStateKey(NodeInterface $node): string {
    return implode(':', ['delete', (string) $node->id()]);
  }

  /**
   * Returns the active language being edited for the node.
   */
  private function getActiveLangcode(NodeInterface $node): string {
    return $node->language()->getId();
  }

  /**
   * Returns the path alias storage.
   */
  private function pathAliasStorage(): EntityStorageInterface {
    return $this->entityTypeManager->getStorage('path_alias');
  }

}
