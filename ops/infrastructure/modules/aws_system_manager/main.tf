resource "aws_ssm_parameter" "project_name_parameter" {
  overwrite = true
  type = "String"
  name = "/${var.project_name}/project/name"
  description = "The name of the project, used to namespace configuration and parameters"
  value = var.project_name
}

resource "aws_ssm_parameter" "project_environment_parameter" {
  overwrite = true
  type = "String"
  name = "/${var.project_name}/project/environment"
  description = "The environment in which the project is running (e.g., development, staging, production)"
  value = var.project_environment
}

resource "aws_ssm_parameter" "project_domain_name_parameter" {
  overwrite = true
  type = "String"
  name = "/${var.project_name}/project/domain_name"
  description = "The primary domain name used by the project (e.g., tiagocode.com)"
  value = var.project_domain_name
}

resource "aws_ssm_parameter" "project_second_level_domain_parameter" {
  overwrite = true
  type = "String"
  name = "/${var.project_name}/project/second_level_domain"
  description = "The second‑level domain (SLD) portion of the project’s primary domain (e.g., 'tiagocode' for tiagocode.com)"
  value = var.project_second_level_domain
}

resource "aws_ssm_parameter" "database_host_parameter" {
  overwrite = true
  type = "String"
  name = "/${var.project_name}/database/host"
  description = "The hostname or IP address of the database server"
  value = var.database_host

  depends_on = [
    var.database_host
  ]
}

resource "aws_ssm_parameter" "database_name_parameter" {
  overwrite = true
  type = "SecureString"
  name = "/${var.project_name}/database/name"
  description = "The name of the application's database"
  value = var.database_name
}

resource "aws_ssm_parameter" "database_username_parameter" {
  overwrite = true
  type = "SecureString"
  name = "/${var.project_name}/database/username"
  description = "The database username used by the application"
  value = var.database_username
}

resource "aws_ssm_parameter" "database_password_parameter" {
  overwrite = true
  type = "SecureString"
  name = "/${var.project_name}/database/password"
  description = "The database password used by the application"
  value = var.database_password
}

resource "aws_ssm_parameter" "cloudflare_api_email" {
  type = "SecureString"
  name = "/${var.project_name}/cloudflare/email"
  description = "Cloudflare account email used for API authentication"
  value = var.cloudflare_email
}

resource "aws_ssm_parameter" "cloudflare_api_key" {
  type = "SecureString"
  name = "/${var.project_name}/cloudflare/api_key"
  description = "Cloudflare global API key used for legacy authentication"
  value = var.cloudflare_api_key
}

resource "aws_ssm_parameter" "cloudflare_api_token" {
  type = "SecureString"
  name = "/${var.project_name}/cloudflare/api_token"
  description = "Cloudflare scoped API token used for secure resource access"
  value = var.cloudflare_api_token
}

resource "aws_ssm_parameter" "cloudflare_dns" {
  type = "String"
  name = "/${var.project_name}/cloudflare/dns"
  description = "DNS zone name or record associated with Cloudflare configuration"
  value = var.cloudflare_dns
}

resource "aws_ssm_parameter" "service_traefik_username_password" {
  type = "SecureString"
  name = "/${var.project_name}/service-traefik/username_password"
  description = "Basic auth credentials used by Traefik for access control"
  value = var.service_traefik_username_password
}

resource "aws_ssm_parameter" "service_grafana_admin_user" {
  type = "SecureString"
  name = "/${var.project_name}/service-grafana/admin_user"
  description = "The admin user for the Grafana service"
  value = var.service_grafana_admin_user
}

resource "aws_ssm_parameter" "service_grafana_admin_password" {
  type = "SecureString"
  name = "/${var.project_name}/service-grafana/admin_password"
  description = "The admin password for the Grafana service"
  value = var.service_grafana_admin_password
}
  
resource "aws_ssm_parameter" "service_redis_username" {
  type = "SecureString"
  name = "/${var.project_name}/service-redis/username"
  description = "Redis username used for accessing the Redis service"
  value = var.service_redis_username
}
  
resource "aws_ssm_parameter" "service_redis_password" {
  type = "SecureString"
  name = "/${var.project_name}/service-redis/password"
  description = "Redis password used for authenticating access to the Redis service"
  value = var.service_redis_password
}

resource "aws_ssm_parameter" "service_drupal_database_prefix" {
  type = "SecureString"
  name = "/${var.project_name}/service-drupal/database_prefix"
  description = "The table prefix used in the Drupal database for namespacing tables"
  value = var.service_drupal_database_prefix
}

resource "aws_ssm_parameter" "service_drupal_hash_salt" {
  type = "SecureString"
  name = "/${var.project_name}/service-drupal/hash_salt"
  description = "The cryptographic hash salt used by Drupal for secure hashing and token generation"
  value = var.service_drupal_hash_salt
}

resource "aws_ssm_parameter" "service_frontend_api_url" {
  type        = "String"
  name        = "/${var.project_name}/service-frontend/api_url"
  description = "The URL of the frontend API service"
  value       = var.service_frontend_api_url
}

resource "aws_ssm_parameter" "service_frontend_api_username" {
  type        = "SecureString"
  name        = "/${var.project_name}/service-frontend/api_username"
  description = "The username used for accessing the frontend API"
  value       = var.service_frontend_api_username
}

resource "aws_ssm_parameter" "service_frontend_api_password" {
  type        = "SecureString"
  name        = "/${var.project_name}/service-frontend/api_password"
  description = "The password used for authenticating with the frontend API"
  value       = var.service_frontend_api_password
}

resource "aws_ssm_parameter" "service_frontend_images_url" {
  type        = "String"
  name        = "/${var.project_name}/service-frontend/images_url"
  description = "The URL from which the frontend service loads static images"
  value       = var.service_frontend_images_url
}

resource "aws_ssm_parameter" "service_frontend_cache_purge_token" {
  type = "SecureString"
  name = "/${var.project_name}/service-frontend/cache_purge_token"
  description = "The cache purge token used by the frontend service to invalidate cached content"
  value = var.service_frontend_cache_purge_token
}

resource "aws_ssm_parameter" "github_container_registry_pat" {
  type        = "SecureString"
  name        = "/${var.project_name}/ghcr/pat"
  description = "The GitHub Container Registry pat used to pull Docker images for the project"
  value       = var.github_container_registry_pat
}

resource "aws_ssm_parameter" "aws_region" {
  type        = "String"
  name        = "/${var.project_name}/aws/region"
  description = "AWS region where the project’s infrastructure is deployed (e.g., eu‑north‑1)"
  value       = var.aws_region
}

resource "aws_ssm_parameter" "aws_access_key" {
  type        = "SecureString"
  name        = "/${var.project_name}/aws/access_key"
  description = "AWS Access Key ID used by CI/CD or application services to authenticate against AWS APIs"
  value       = var.aws_access_key
}

resource "aws_ssm_parameter" "aws_secret_key" {
  type        = "SecureString"
  name        = "/${var.project_name}/aws/secret_key"
  description = "AWS Secret Access Key corresponding to the Access Key ID for programmatic access"
  value       = var.aws_secret_key
}

resource "aws_ssm_parameter" "aws_s3_bucket_name" {
  type        = "String"
  name        = "/${var.project_name}/aws/s3_bucket_name"
  description = "S3 bucket name used by the project (e.g., for static assets, logs, or Terraform state)"
  value       = var.aws_s3_bucket_name
}

resource "aws_ssm_parameter" "aws_ec2_private_key_pem" {
  type = "SecureString"
  name = "/${var.project_name}/aws/ec2_private_key.pem"
  description = "SSH private key (PEM) for EC2"
  value = var.aws_ec2_private_key_pem
}