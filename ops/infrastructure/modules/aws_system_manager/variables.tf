variable "project_environment" {
  type = string
}

variable "project_name" {
  type = string
}

variable "project_domain_name" {
  type = string
}

variable "project_second_level_domain" {
  type = string
}

variable "project_subdomain_prefix" {
  type = string
}

variable "database_host" {
  type = string
}

variable "database_name" {
  type = string
}

variable "database_username" {
  sensitive = true
  type = string
}

variable "database_password" {
  sensitive = true
  type = string
}

variable "cloudflare_api_token" {
  sensitive = true
  type = string
}

variable "cloudflare_email" {
  sensitive = true
  type = string
}

variable "cloudflare_dns" {
  type = string
}

variable "cloudflare_api_key" {
  sensitive = true
  type = string
}

variable "service_traefik_username_password" {
  sensitive = true
  type = string
}

variable "service_grafana_admin_user" {
  sensitive = true
  type = string
}

variable "service_grafana_admin_password" {
  sensitive = true
  type = string
}

variable "service_redis_username" {
  sensitive = true
  type = string
}

variable "service_redis_password" {
  sensitive = true
  type = string
}

variable "service_drupal_hash_salt" {
  sensitive = true
  type = string
}

variable "service_drupal_database_prefix" {
  sensitive = true
  type = string
}

variable "service_frontend_api_url" {
  type = string
}

variable "service_frontend_api_username" {
  sensitive = true
  type = string
}

variable "service_frontend_api_password" {
  sensitive = true
  type = string  
}

variable "service_frontend_images_url" {
  type = string
}

variable "service_frontend_cache_purge_token" {
  sensitive = true
  type = string
}

variable "github_container_registry_pat" {
  sensitive = true
  type = string
}

variable "aws_region" {
  type = string
}

variable "aws_secret_key" {
  sensitive = true
  type = string
}

variable "aws_access_key" {
  sensitive = true
  type = string
}

variable "aws_s3_bucket_name" {
  type = string
}

variable "aws_ec2_private_key_pem" {
  sensitive = true
  type = string
}