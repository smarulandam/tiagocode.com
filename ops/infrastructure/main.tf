terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }

    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 5.5"
    }

    cloudinit = {
      source  = "hashicorp/cloudinit"
      version = "~> 2.3"
    }

    tls = {
      source  = "hashicorp/tls"
      version = "~> 4"
    }
  }

  backend "s3" {
    bucket = "tiagocode--eun1-az1--x-s3"
    key    = "terraform/backend"
    region = "eu-north-1"
  }
}

provider "aws" {
  region = var.aws_region
}

provider "cloudflare" {
  email     = var.cloudflare_email
  api_token = var.cloudflare_api_token
}

module "aws_system_manager" {
  source                             = "./modules/aws_system_manager"
  project_environment                = var.project_environment
  project_name                       = var.project_name
  project_domain_name                = var.project_domain_name
  project_subdomain_prefix           = var.project_domain_name
  project_second_level_domain        = var.project_second_level_domain
  cloudflare_email                   = var.cloudflare_email
  cloudflare_api_key                 = var.cloudflare_api_key
  cloudflare_api_token               = var.cloudflare_api_token
  cloudflare_dns                     = var.cloudflare_dns
  database_host                      = module.aws_rds.instance_address
  database_name                      = var.database_name
  database_username                  = var.database_username
  database_password                  = var.database_password
  service_redis_username             = var.service_redis_username
  service_redis_password             = var.service_redis_password
  service_traefik_username_password  = var.service_traefik_username_password
  service_grafana_admin_user         = var.service_grafana_admin_user
  service_grafana_admin_password     = var.service_grafana_admin_password
  service_drupal_database_prefix     = var.service_drupal_database_prefix
  service_drupal_hash_salt           = var.service_drupal_hash_salt
  service_frontend_api_url           = var.service_frontend_api_url
  service_frontend_api_username      = var.service_frontend_api_username
  service_frontend_api_password      = var.service_frontend_api_password
  service_frontend_images_url        = var.service_frontend_images_url
  service_frontend_cache_purge_token = var.service_frontend_cache_purge_token
  github_container_registry_pat      = var.github_container_registry_pat
  aws_region                         = var.aws_region
  aws_s3_bucket_name                 = module.aws_s3.bucket_id
  // @todo refactor this part, find a better way to bring s3 access to the applications
  aws_access_key          = var.aws_access_key
  aws_secret_key          = var.aws_secret_key
  aws_ec2_private_key_pem = module.aws_ec2.instance_private_key_pem_content
}

module "aws_vpc" {
  source                      = "./modules/aws_vpc"
  aws_region                  = var.aws_region
  project_name                = var.project_name
  vpc_cidr_block              = var.aws_vpc_cidr_block
  public_subnet_cidr_block    = var.aws_public_subnet_cidr_block
  private_subnet_1_cidr_block = var.aws_private_subnet_1_cidr_block
  private_subnet_2_cidr_block = var.aws_private_subnet_2_cidr_block
}

module "aws_s3" {
  source              = "./modules/aws_s3"
  project_name        = var.project_name
  project_domain_name = var.project_domain_name
}

module "aws_iam" {
  source       = "./modules/aws_iam"
  project_name = var.project_name
}

module "aws_rds" {
  source                  = "./modules/aws_rds"
  project_name            = var.project_name
  instance_class          = var.aws_rds_instance_class
  instance_engine         = var.aws_rds_instance_engine
  instance_engine_version = var.aws_rds_instance_engine_version
  instance_storage_type   = var.aws_rds_instance_storage_type
  database_name           = var.database_name
  database_username       = var.database_username
  database_password       = var.database_password
  private_subnets_ids     = module.aws_vpc.private_subnet_ids
  security_groups_ids     = [module.aws_vpc.rds_security_group_id]
}

module "aws_ec2" {
  aws_region          = var.aws_region
  source              = "./modules/aws_ec2"
  project_name        = var.project_name
  ami                 = var.aws_ec2_instance_ami
  instance_type       = var.aws_ec2_instance_type
  instance_profile    = module.aws_iam.ec2_instance_profile_name
  vpc_id              = module.aws_vpc.vpc_id
  subnet_id           = module.aws_vpc.public_subnet_id
  security_groups_ids = [module.aws_vpc.default_security_group_id]
  internet_gateway_id = module.aws_vpc.internet_gateway_id
}

module "aws_cloudwatch" {
  source          = "./modules/aws_cloudwatch"
  aws_region      = var.aws_region
  project_name    = var.project_name
  vpc_id          = module.aws_vpc.vpc_id
  bucket_id       = module.aws_s3.bucket_id
  ec2_instance_id = module.aws_ec2.instance_id
  rds_instance_id = module.aws_rds.instance_id
}

module "aws_codedeploy" {
  source            = "./modules/aws_codedeploy"
  aws_region        = var.aws_region
  project_name      = var.project_name
  instance_name     = module.aws_ec2.instance_name
  deployer_role_arn = module.aws_iam.codedeploy_service_role_arn
}

module "aws_biling_and_cost" {
  source             = "./modules/aws_billing_and_cost"
  project_name       = var.project_name
  notification_email = var.cloudflare_email
}

module "cloudflare_dns" {
  source              = "./modules/cloudflare_dns"
  cloudflare_zone_id  = var.cloudflare_zone_id
  project_domain_name = var.project_domain_name
  instance_public_ip  = module.aws_ec2.instance_public_ip
}
