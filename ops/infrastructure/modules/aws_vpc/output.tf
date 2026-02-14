output "vpc_id" {
  value = aws_vpc.default_vpc.id
}

output "public_subnet_id" {
  value = aws_subnet.public_subnet.id
}

output "private_subnet_ids" {
  value = [
    aws_subnet.private_subnet_1.id,
    aws_subnet.private_subnet_2.id,
  ]
}

output "default_security_group_id" {
  value = aws_security_group.default_security_group.id
}

output "rds_security_group_id" {
  value = aws_security_group.rds_security_group.id
}

output "internet_gateway_id" {
  value = aws_internet_gateway.internet_gw.id
}