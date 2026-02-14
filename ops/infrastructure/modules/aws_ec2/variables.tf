variable "aws_region" {
  type = string
}

variable "project_name" {
  type = string
}

variable "ami" {
  type = string
}

variable "instance_type" {
  type = string
}

variable "instance_profile" {
  type = string
}

variable "vpc_id" {
  type = string
}

variable "subnet_id" {
  type = string
}

variable "security_groups_ids" {
  type = list(string)
}

variable "internet_gateway_id" {
  type = string
}