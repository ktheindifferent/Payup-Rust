use payup::stripe::*;

#[cfg(test)]
mod auth_tests {
    use super::*;

    #[test]
    fn test_auth_new() {
        let auth = Auth::new("client_key".to_string(), "secret_key".to_string());
        assert_eq!(auth.client, "client_key");
        assert_eq!(auth.secret, "secret_key");
    }
    
    #[test]
    fn test_auth_clone() {
        let auth = Auth::new("client_key".to_string(), "secret_key".to_string());
        let auth_clone = auth.clone();
        assert_eq!(auth.client, auth_clone.client);
        assert_eq!(auth.secret, auth_clone.secret);
    }
}

#[cfg(test)]
mod balance_tests {
    use super::*;

    #[test]
    fn test_balance_fields() {
        // Balance doesn't have a new() method, it's retrieved via API
        // This test would verify the structure if we had mock data
        // Skipping for now as Balance::new() doesn't exist
    }
}

#[cfg(test)]
mod customer_tests {
    use super::*;

    #[test]
    fn test_customer_new() {
        let customer = Customer::new();
        assert!(customer.id.is_none());
        assert!(customer.name.is_none());
        assert!(customer.email.is_none());
        assert!(customer.phone.is_none());
    }

    #[test]
    fn test_customer_with_data() {
        let mut customer = Customer::new();
        customer.name = Some("John Doe".to_string());
        customer.email = Some("john@example.com".to_string());
        customer.phone = Some("123-456-7890".to_string());
        
        assert_eq!(customer.name.unwrap(), "John Doe");
        assert_eq!(customer.email.unwrap(), "john@example.com");
        assert_eq!(customer.phone.unwrap(), "123-456-7890");
    }
}

#[cfg(test)]
mod charge_tests {
    use super::*;

    #[test]
    fn test_charge_new() {
        let charge = Charge::new();
        assert!(charge.id.is_none());
        assert!(charge.amount.is_none());
        assert!(charge.currency.is_none());
        assert!(charge.customer.is_none());
    }

    #[test]
    fn test_charge_with_amount() {
        let mut charge = Charge::new();
        charge.amount = Some("1000".to_string());
        charge.currency = Some("usd".to_string());
        
        assert_eq!(charge.amount.unwrap(), "1000");
        assert_eq!(charge.currency.unwrap(), "usd");
    }
}

#[cfg(test)]
mod card_tests {
    use super::*;

    #[test]
    fn test_card_new() {
        let card = Card::new();
        assert!(card.number.is_none());
        assert!(card.exp_month.is_none());
        assert!(card.exp_year.is_none());
        assert!(card.cvc.is_none());
    }

    #[test]
    fn test_card_with_valid_data() {
        let mut card = Card::new();
        card.number = Some("4242424242424242".to_string());
        card.exp_month = Some("12".to_string());
        card.exp_year = Some("2025".to_string());
        card.cvc = Some("123".to_string());
        
        assert_eq!(card.number.unwrap(), "4242424242424242");
        assert_eq!(card.exp_month.unwrap(), "12");
        assert_eq!(card.exp_year.unwrap(), "2025");
        assert_eq!(card.cvc.unwrap(), "123");
    }
}

#[cfg(test)]
mod payment_method_tests {
    use super::*;

    #[test]
    fn test_payment_method_new() {
        let pm = PaymentMethod::new();
        assert!(pm.id.is_none());
        assert!(pm.method_type.is_none());
        assert!(pm.card.is_none());
    }

    #[test]
    fn test_payment_method_with_card() {
        let mut pm = PaymentMethod::new();
        pm.method_type = Some("card".to_string());
        
        let mut card = Card::new();
        card.number = Some("4242424242424242".to_string());
        pm.card = Some(card);
        
        assert_eq!(pm.method_type.unwrap(), "card");
        assert!(pm.card.is_some());
    }
}

#[cfg(test)]
mod subscription_tests {
    use super::*;

    #[test]
    fn test_subscription_new() {
        let sub = Subscription::new();
        assert!(sub.id.is_none());
        assert!(sub.customer.is_none());
        assert!(sub.status.is_none());
    }

    #[test]
    fn test_subscription_with_customer() {
        let mut sub = Subscription::new();
        sub.customer = Some("cus_123".to_string());
        sub.status = Some("active".to_string());
        
        assert_eq!(sub.customer.unwrap(), "cus_123");
        assert_eq!(sub.status.unwrap(), "active");
    }
}

#[cfg(test)]
mod plan_tests {
    use super::*;

    #[test]
    fn test_plan_new() {
        let plan = Plan::new();
        assert!(plan.id.is_none());
        assert!(plan.amount.is_none());
        assert!(plan.currency.is_none());
        assert!(plan.interval.is_none());
    }

    #[test]
    fn test_plan_with_pricing() {
        let mut plan = Plan::new();
        plan.amount = Some("999".to_string());
        plan.currency = Some("usd".to_string());
        plan.interval = Some("month".to_string());
        
        assert_eq!(plan.amount.unwrap(), "999");
        assert_eq!(plan.currency.unwrap(), "usd");
        assert_eq!(plan.interval.unwrap(), "month");
    }
}

#[cfg(test)]
mod invoice_tests {
    use super::*;

    #[test]
    fn test_invoice_new() {
        let invoice = Invoice::new();
        assert!(invoice.id.is_none());
        assert!(invoice.customer.is_none());
        assert!(invoice.status.is_none());
    }

    #[test]
    fn test_invoice_with_customer() {
        let mut invoice = Invoice::new();
        invoice.customer = Some("cus_456".to_string());
        invoice.status = Some("paid".to_string());
        
        assert_eq!(invoice.customer.unwrap(), "cus_456");
        assert_eq!(invoice.status.unwrap(), "paid");
    }
}