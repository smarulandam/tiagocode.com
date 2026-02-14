variable "cloudflare_zone_id" {
  sensitive = true
  type = string
}

variable "project_domain_name" {
  type = string
}

variable "instance_public_ip" {
  type = string
}
