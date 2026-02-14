resource "aws_codedeploy_app" "app_codeploy_app" {
  name = "${var.project_name}-app"
  compute_platform = "Server"
}

resource "aws_codedeploy_deployment_group" "app_codeploy_group" {
  app_name = aws_codedeploy_app.app_codeploy_app.name
  deployment_group_name = "${var.project_name}-dg"
  service_role_arn = var.deployer_role_arn

  ec2_tag_set {
    ec2_tag_filter {
      key = "Name"
      type = "KEY_AND_VALUE"
      value = var.instance_name
    }
  }

  deployment_style {
    deployment_type = "IN_PLACE"
    deployment_option = "WITHOUT_TRAFFIC_CONTROL"
  }
}