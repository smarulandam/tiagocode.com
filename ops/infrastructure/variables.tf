variable "aws_region" {
  type        = string
  default     = "eu-north-1"
  description = "The AWS region to deploy resources into."
}

variable "aws_access_key" {
  sensitive   = true
  type        = string
  description = "AWS Access Key ID used by Terraform or CI/CD pipelines for authenticated AWS API calls."
}

variable "aws_secret_key" {
  sensitive   = true
  type        = string
  description = "AWS Secret Access Key paired with the Access Key ID for secure programmatic access."
}

variable "aws_availability_zone_1" {
  type        = string
  default     = "eu-north-1a"
  description = "The first availability zone used for deploying resources."
}

variable "aws_availability_zone_2" {
  type        = string
  default     = "eu-north-1b"
  description = "The second availability zone used for deploying resources."
}

variable "aws_ec2_instance_ami" {
  type        = string
  default     = "ami-00f34bf9aeacdf007"
  description = "The ID of the Amazon Machine Image (AMI) to use for the virtual machine."
}

variable "aws_ec2_instance_type" {
  type        = string
  default     = "t3.small"
  description = "The instance type for the virtual machine (e.g., t3.micro)."
}

variable "aws_rds_instance_engine" {
  type        = string
  default     = "postgres"
  description = "The database engine for the RDS instance (e.g., postgres, mysql)."
}

variable "aws_rds_instance_engine_version" {
  type        = string
  default     = "17.5"
  description = "The version of the database engine for the RDS instance."
}

variable "aws_rds_instance_class" {
  type        = string
  default     = "db.t4g.micro"
  description = "The instance class for the RDS instance (e.g., db.t3.micro, db.m5.large)."
}

variable "aws_rds_instance_storage_type" {
  type        = string
  default     = "gp2"
  description = "The storage type for the RDS instance (e.g., gp2, io1)."
}

variable "aws_vpc_cidr_block" {
  type        = string
  default     = "10.0.0.0/16"
  description = "The CIDR block to be used for the Virtual Private Cloud (VPC)."
}

variable "aws_public_subnet_cidr_block" {
  type        = string
  default     = "10.0.0.0/24"
  description = "The CIDR block to be used for the public subnet within the VPC."
}

variable "aws_private_subnet_1_cidr_block" {
  type        = string
  default     = "10.0.1.0/24"
  description = "The CIDR block to be used for the private subnet within the VPC."
}

variable "aws_private_subnet_2_cidr_block" {
  type        = string
  default     = "10.0.2.0/24"
  description = "The CIDR block to be used for the private subnet within the VPC."
}

variable "cloudflare_zone_id" {
  sensitive   = true
  type        = string
  description = "The ID of the Cloudflare zone to use for DNS records."
}

variable "cloudflare_email" {
  sensitive   = true
  type        = string
  description = "Email address associated with your Cloudflare account. Required for authenticating with the Cloudflare provider."
}

variable "cloudflare_api_key" {
  sensitive   = true
  type        = string
  description = "Cloudflare API key for legacy authentication. Use only if not using API token."
}

variable "cloudflare_api_token" {
  sensitive   = true
  type        = string
  description = "Cloudflare API token with permissions to manage DNS records. Used for authenticating with the Cloudflare provider."
}

variable "cloudflare_dns" {
  type        = string
  description = "The DNS name or configuration used in the Cloudflare integration. This may refer to a DNS record or setting needed by the project."
}

variable "project_environment" {
  type        = string
  description = "Specifies the environment in which the project is deployed (e.g., development, staging, production)."
}

variable "project_name" {
  type        = string
  description = "A short, unique identifier for the project. Used to name, group, and tag AWS resources consistently."
}

variable "project_domain_name" {
  type        = string
  description = "The primary domain name used for the project (e.g., for DNS records, TLS certificates, and routing)."
}

variable "project_second_level_domain" {
  type        = string
  description = "The second-level domain used in the project, typically the main domain without subdomains (e.g., for admin.tiagocode.com is tiagocode)."
}

variable "project_subdomain_prefix" {
  type        = string
  default     = ""
  description = "The prefix for subdomains used in the project (e.g., 'local-' for local-admin.tiagocode.com)."
}

variable "database_name" {
  sensitive   = true
  type        = string
  description = "The name of the application's primary database."
}

variable "database_username" {
  sensitive   = true
  type        = string
  description = "The username used to authenticate with the application's database."
}

variable "database_password" {
  sensitive   = true
  type        = string
  description = "The password used to authenticate with the application's database."
}

variable "service_redis_username" {
  sensitive   = true
  type        = string
  description = "Username required to access the Redis service instance."
}

variable "service_redis_password" {
  sensitive   = true
  type        = string
  description = "Password used to authenticate with the Redis service."
}

variable "service_traefik_username_password" {
  sensitive   = true
  type        = string
  description = "Combined username and password (in user:pass format) used for basic auth with the Traefik dashboard."
}

variable "service_grafana_admin_user" {
  sensitive   = true
  type        = string
  description = "The admin user for the Grafana service."
}

variable "service_grafana_admin_password" {
  sensitive   = true
  type        = string
  description = "The admin password for the Grafana service."
}

variable "service_drupal_database_prefix" {
  sensitive   = true
  type        = string
  description = "Prefix used for Drupal database tables to namespace installations or differentiate environments."
}

variable "service_drupal_hash_salt" {
  sensitive   = true
  type        = string
  description = "A secure random string used by Drupal for one-way hashing and securing data (e.g., sessions, passwords)."
}

variable "service_frontend_api_url" {
  type        = string
  description = "Base URL for the frontend to communicate with the backend API."
}

variable "service_frontend_api_username" {
  sensitive   = true
  type        = string
  description = "Username used by the frontend to authenticate with the backend API."
}

variable "service_frontend_api_password" {
  sensitive   = true
  type        = string
  description = "Password used by the frontend to authenticate with the backend API."
}

variable "service_frontend_images_url" {
  type        = string
  description = "URL used by the frontend to access static image assets."
}

variable "service_frontend_cache_purge_token" {
  sensitive   = true
  type        = string
  description = "The cache purge token used by the frontend service to invalidate cached content."
}

variable "github_container_registry_pat" {
  sensitive   = true
  type        = string
  description = "The GitHub Container Registry pat used to pull Docker images for the project"
}
