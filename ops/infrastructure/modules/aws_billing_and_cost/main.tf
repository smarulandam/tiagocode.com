resource "aws_budgets_budget" "app_budget" {
  
 name = "${var.project_name}-monthly-budget"
    budget_type = "COST"
    limit_amount = "15"
    limit_unit = "USD"
    time_period_start = "2026-02-01_00:00"
    time_period_end = "2030-02-01_23:59"
    time_unit = "MONTHLY"

    notification {
        comparison_operator        = "GREATER_THAN"
        threshold                  = 75
        threshold_type             = "PERCENTAGE"
        notification_type          = "FORECASTED"
        subscriber_email_addresses = [var.notification_email]
    }

    tags = {
        Name = "${var.project_name}-monthly-budget"
    }
}