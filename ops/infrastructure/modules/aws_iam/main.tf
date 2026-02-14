# ---------------------------------------------------------------------
# --------------------- CodeDeploy Service Role -----------------------
#
# Role assumed by the AWS CodeDeploy service to deploy applications.
#
#----------------------------------------------------------------------

resource "aws_iam_role" "codedeploy_service_role" {
  name = "${title(var.project_name)}CodeDeployServiceRole"
  assume_role_policy = data.aws_iam_policy_document.codedeploy_assume_role_policy_document.json
}

resource "aws_iam_role_policy_attachment" "codeploy_policy_attachment" {
  role = aws_iam_role.codedeploy_service_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSCodeDeployRole"
}

data "aws_iam_policy_document" "codedeploy_assume_role_policy_document" {
  statement {
    sid = ""
    effect = "Allow"
    principals {
      type = "Service"
      identifiers = ["codedeploy.amazonaws.com"]
    }
    actions = ["sts:AssumeRole"]
  }  
}

# ---------------------------------------------------------------------
# ------------------------ EC2 Service Role ---------------------------
#
# - Role assumed by EC2 to interact with CodeDeploy.
# - Role assumed by EC2 to interact with System Manager.
#
# ---------------------------------------------------------------------

resource "aws_iam_role" "ec2_service_instance_role" {
  name               = "${title(var.project_name)}Ec2ServiceInstanceRole"
  assume_role_policy = data.aws_iam_policy_document.ec2_assume_role_policy_document.json
}

resource "aws_iam_role_policy_attachment" "ec2_for_code_deploy_policy_attachment" {
  role       = aws_iam_role.ec2_service_instance_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonEC2RoleforAWSCodeDeploy"
}

resource "aws_iam_role_policy_attachment" "sssm_managed_instance_core_policy_attachment" {
  role = aws_iam_role.ec2_service_instance_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
}

data "aws_iam_policy_document" "ec2_assume_role_policy_document" {
  statement {
    sid = "${title(var.project_name)}AllowEc2AssumeRole"
    effect = "Allow"
    principals {
      type        = "Service"
      identifiers = ["ec2.amazonaws.com"]
    }
    actions = ["sts:AssumeRole"]
  }
}

# ---------------------------------------------------------------------
# --------------------- EC2 Instance profile ---------------------------
#
# ---------------------------------------------------------------------

resource "aws_iam_instance_profile" "ec2_instance_profile" {
  name = "${title(var.project_name)}Ec2InstanceProfile"
  role = aws_iam_role.ec2_service_instance_role.name
}

