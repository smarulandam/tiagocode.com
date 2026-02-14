terraform {
  required_providers {
    cloudflare = {
        source = "cloudflare/cloudflare"
    }
  }
}

data "cloudflare_zone" "tiagocode_zone" {
  zone_id = var.cloudflare_zone_id
}

resource "cloudflare_dns_record" "root_domain_record" {
  zone_id = data.cloudflare_zone.tiagocode_zone.zone_id
  name    = "@"
  type    = "A"
  content = var.instance_public_ip
  ttl     = 1
  proxied = true
}

resource "cloudflare_dns_record" "www_alias_record" {
  zone_id = data.cloudflare_zone.tiagocode_zone.zone_id
  name    = "www"
  type    = "CNAME"
  content = var.project_domain_name
  ttl     = 1
  proxied = true
}

resource "cloudflare_dns_record" "admin_subdomain_record" {
  zone_id = data.cloudflare_zone.tiagocode_zone.zone_id
  name    = "admin"
  type    = "A"
  content = var.instance_public_ip
  ttl     = 1
  proxied = true

}

resource "cloudflare_dns_record" "website_subdomain_record" {
  zone_id = data.cloudflare_zone.tiagocode_zone.zone_id
  name    = "website"
  type    = "A"
  content = var.instance_public_ip
  ttl     = 1
  proxied = true
}

resource "cloudflare_dns_record" "traefik_subdomain_record" {
  zone_id = data.cloudflare_zone.tiagocode_zone.zone_id
  name    = "traefik"
  type    = "A"
  content = var.instance_public_ip
  ttl     = 1
  proxied = true
}

resource "cloudflare_dns_record" "grafana_subdomain_record" {
  zone_id = data.cloudflare_zone.tiagocode_zone.zone_id
  name    = "grafana"
  type    = "A"
  content = var.instance_public_ip
  ttl     = 1
  proxied = true
}