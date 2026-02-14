resource "aws_vpc" "default_vpc" {
  cidr_block = var.vpc_cidr_block

  tags = {
    Name = "${var.project_name}-vpc"
  }
}

resource "aws_subnet" "public_subnet" {
  vpc_id = aws_vpc.default_vpc.id
  cidr_block = var.public_subnet_cidr_block

  tags = {
    Name = "${var.project_name}-public-subnet"
  }
}

resource "aws_subnet" "private_subnet_1" {
  vpc_id = aws_vpc.default_vpc.id
  cidr_block = var.private_subnet_1_cidr_block
  availability_zone = "${var.aws_region}a"

  tags = {
    Name = "${var.project_name}-private-subnet-1"
  }
}

resource "aws_subnet" "private_subnet_2" {
  vpc_id = aws_vpc.default_vpc.id
  cidr_block = var.private_subnet_2_cidr_block
  availability_zone = "${var.aws_region}b"

  tags = {
    Name = "${var.project_name}-private-subnet-2"
  }
}


resource "aws_internet_gateway" "internet_gw" {
  vpc_id = aws_vpc.default_vpc.id

  tags = {
    Name = "${var.project_name}-igw"
  }
}

resource "aws_route" "route" {
  destination_cidr_block = "0.0.0.0/0"
  route_table_id = aws_route_table.route_table.id
  gateway_id = aws_internet_gateway.internet_gw.id
}

resource "aws_route_table" "route_table" {
  vpc_id = aws_vpc.default_vpc.id

  tags = {
    Name = "${var.project_name}-route-table"
  }
}

resource "aws_route_table_association" "route_table_association" {
  route_table_id = aws_route_table.route_table.id
  subnet_id = aws_subnet.public_subnet.id
}

resource "aws_security_group" "default_security_group" {
  name = "${var.project_name}-default-sg"
  description = "Default security group to allow inbound/outbound traffic from the VPC"
  vpc_id = aws_vpc.default_vpc.id
  depends_on = [ 
    aws_vpc.default_vpc
  ]
}

resource "aws_vpc_security_group_ingress_rule" "http_ingress_rule" {
  security_group_id = aws_security_group.default_security_group.id
  cidr_ipv4 = "0.0.0.0/0"
  ip_protocol = "tcp"
  from_port = 80
  to_port = 80
}

resource "aws_vpc_security_group_ingress_rule" "https_ingress_rule" {
  security_group_id = aws_security_group.default_security_group.id
  cidr_ipv4 = "0.0.0.0/0"
  ip_protocol = "tcp"
  from_port = 443
  to_port = 443
}

# resource "aws_vpc_security_group_ingress_rule" "ssh_ingress_rule" {
#   security_group_id = aws_security_group.default_security_group.id
#   cidr_ipv4 = "0.0.0.0/0"
#   ip_protocol = "tcp"
#   from_port = 22
#   to_port = 22
# }

resource "aws_vpc_security_group_egress_rule" "all_egress_rule" {
  security_group_id = aws_security_group.default_security_group.id
  cidr_ipv4 = "0.0.0.0/0"
  ip_protocol = -1
}

resource "aws_security_group" "rds_security_group" {
  name = "${var.project_name}-rds-sg"
  description = "Default security group to allow inbound/outboud traffic from RDS"
  vpc_id = aws_vpc.default_vpc.id
  depends_on = [ 
    aws_vpc.default_vpc
  ]
}

resource "aws_vpc_security_group_ingress_rule" "postgres_ingress_rule" {
  security_group_id = aws_security_group.rds_security_group.id
  referenced_security_group_id = aws_security_group.default_security_group.id
  ip_protocol = "tcp"
  from_port = 5432
  to_port   = 5432
}