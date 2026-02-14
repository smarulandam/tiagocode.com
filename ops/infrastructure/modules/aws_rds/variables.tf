variable "project_name" {
  type = string
}

variable "private_subnets_ids" {
  type = list(string)
}

variable "security_groups_ids" {
  type = list(string)
}

variable "instance_engine" {
  type = string
}

variable "instance_engine_version" {
  type = string
}

variable "instance_class" {
  type = string
}

variable "instance_storage_type" {
  type = string
}

variable "database_name" {
  type = string
  sensitive = true
}

variable "database_username" {
  type = string
  sensitive = true
}

variable "database_password" {
  type = string
  sensitive = true
}