use medi_token::{MediToken, utils::Utils, MediTokenError};

#[cfg(test)]
mod healthcare_integration_tests {
    use super::*;

    /// Test token-gated access control for healthcare resources
    #[test]
    fn test_token_gated_healthcare_access() {
        let system_admin = Utils::generate_random_address();
        let doctor = Utils::generate_random_address();
        let patient = Utils::generate_random_address();
        let unauthorized_user = Utils::generate_random_address();
        
        let mut token = MediToken::new(Utils::ether_to_wei(10000.0), system_admin.clone());
        
        // Distribute tokens to authorized users
        let doctor_tokens = Utils::ether_to_wei(100.0);
        let patient_tokens = Utils::ether_to_wei(50.0);
        
        token.transfer(&system_admin, &doctor, doctor_tokens).unwrap();
        token.transfer(&system_admin, &patient, patient_tokens).unwrap();
        
        // Simulate healthcare access control
        fn can_access_medical_records(token: &MediToken, user: &str, required_tokens: u64) -> bool {
            token.balance_of(user) >= required_tokens
        }
        
        fn can_write_prescription(token: &MediToken, user: &str) -> bool {
            token.balance_of(user) >= Utils::ether_to_wei(50.0) 
        }
        
        // Test access control
        assert!(can_access_medical_records(&token, &doctor, Utils::ether_to_wei(10.0)));
        assert!(can_access_medical_records(&token, &patient, Utils::ether_to_wei(10.0)));
        assert!(!can_access_medical_records(&token, &unauthorized_user, Utils::ether_to_wei(10.0)));
        
        assert!(can_write_prescription(&token, &doctor));
        assert!(can_write_prescription(&token, &patient));
        assert!(!can_write_prescription(&token, &unauthorized_user));
    }

    /// Test patient payment system using MediToken
    #[test]
    fn test_patient_payment_system() {
        let patient = Utils::generate_random_address();
        let healthcare_provider = Utils::generate_random_address();
        let insurance_company = Utils::generate_random_address();
        
        let mut token = MediToken::new(Utils::ether_to_wei(1000.0), patient.clone());
        
        // Simulate a medical bill payment
        let consultation_fee = Utils::ether_to_wei(25.0);
        let insurance_coverage = Utils::ether_to_wei(20.0);
        let patient_copay = Utils::ether_to_wei(5.0);
        
        // Patient approves insurance to pay on their behalf
        token.approve(&patient, &insurance_company, insurance_coverage).unwrap();
        
        // Insurance pays the provider
        token.transfer_from(&insurance_company, &patient, &healthcare_provider, insurance_coverage).unwrap();
        
        // Patient pays the remaining copay
        token.transfer(&patient, &healthcare_provider, patient_copay).unwrap();
        
        // Verify final balances
        let expected_patient_balance = Utils::ether_to_wei(1000.0) - consultation_fee;
        let expected_provider_balance = consultation_fee;
        
        assert_eq!(token.balance_of(&patient), expected_patient_balance);
        assert_eq!(token.balance_of(&healthcare_provider), expected_provider_balance);
        assert_eq!(token.balance_of(&insurance_company), 0);
    }

    /// Test health rewards system
    #[test]
    fn test_health_rewards_system() {
        let health_system = Utils::generate_random_address();
        let patient1 = Utils::generate_random_address();
        let patient2 = Utils::generate_random_address();
        let patient3 = Utils::generate_random_address();
        
        let mut token = MediToken::new(Utils::ether_to_wei(10000.0), health_system.clone());
        
        // Simulate health rewards for different activities
        struct HealthActivity {
            patient: String,
            activity: String,
            reward: u64,
        }
        
        let activities = vec![
            HealthActivity {
                patient: patient1.clone(),
                activity: "Annual checkup".to_string(),
                reward: Utils::ether_to_wei(10.0),
            },
            HealthActivity {
                patient: patient2.clone(),
                activity: "Vaccination".to_string(),
                reward: Utils::ether_to_wei(15.0),
            },
            HealthActivity {
                patient: patient3.clone(),
                activity: "Health screening".to_string(),
                reward: Utils::ether_to_wei(20.0),
            },
            HealthActivity {
                patient: patient1.clone(),
                activity: "Wellness program completion".to_string(),
                reward: Utils::ether_to_wei(25.0),
            },
        ];
        
        // Distribute rewards
        for activity in &activities {
            token.transfer(&health_system, &activity.patient, activity.reward).unwrap();
        }
        
        // Verify patient balances
        assert_eq!(token.balance_of(&patient1), Utils::ether_to_wei(35.0)); // 10 + 25
        assert_eq!(token.balance_of(&patient2), Utils::ether_to_wei(15.0));
        assert_eq!(token.balance_of(&patient3), Utils::ether_to_wei(20.0));
        
        let total_rewards = Utils::ether_to_wei(70.0);
        let expected_system_balance = Utils::ether_to_wei(10000.0) - total_rewards;
        assert_eq!(token.balance_of(&health_system), expected_system_balance);
    }

    /// Test research funding and donations
    #[test]
    fn test_research_funding() {
        let research_foundation = Utils::generate_random_address();
        let donor1 = Utils::generate_random_address();
        let donor2 = Utils::generate_random_address();
        let research_project = Utils::generate_random_address();
        
        let mut token = MediToken::new(Utils::ether_to_wei(0.0), research_foundation.clone());
        
        // Give tokens to donors
        let donor1_amount = Utils::ether_to_wei(500.0);
        let donor2_amount = Utils::ether_to_wei(300.0);
        
        // Simulate initial token distribution (e.g., from token sale)
        token.balances.insert(donor1.clone(), donor1_amount);
        token.balances.insert(donor2.clone(), donor2_amount);
        token.total_supply = donor1_amount + donor2_amount;
        
        // Donors fund research project
        let donation1 = Utils::ether_to_wei(100.0);
        let donation2 = Utils::ether_to_wei(150.0);
        
        token.transfer(&donor1, &research_project, donation1).unwrap();
        token.transfer(&donor2, &research_project, donation2).unwrap();
        
        // Verify funding
        let total_funding = donation1 + donation2;
        assert_eq!(token.balance_of(&research_project), total_funding);
        assert_eq!(token.balance_of(&donor1), donor1_amount - donation1);
        assert_eq!(token.balance_of(&donor2), donor2_amount - donation2);
    }

    /// Test medical inventory management
    #[test]
    fn test_medical_inventory_management() {
        let hospital = Utils::generate_random_address();
        let pharmacy = Utils::generate_random_address();
        let medical_supplier = Utils::generate_random_address();
        
        let mut token = MediToken::new(Utils::ether_to_wei(5000.0), hospital.clone());
        
        // Hospital purchases medical supplies
        let supply_orders = vec![
            ("Surgical masks", Utils::ether_to_wei(50.0)),
            ("Medications", Utils::ether_to_wei(200.0)),
            ("Medical equipment", Utils::ether_to_wei(500.0)),
        ];
        
        let mut total_cost = 0u64;
        for (_item, cost) in &supply_orders {
            token.transfer(&hospital, &medical_supplier, *cost).unwrap();
            total_cost += cost;
        }
        
        // Pharmacy orders from hospital
        let pharmacy_order = Utils::ether_to_wei(75.0);
        token.transfer(&hospital, &pharmacy, pharmacy_order).unwrap();
        
        // Verify balances
        let expected_hospital_balance = Utils::ether_to_wei(5000.0) - total_cost - pharmacy_order;
        assert_eq!(token.balance_of(&hospital), expected_hospital_balance);
        assert_eq!(token.balance_of(&medical_supplier), total_cost);
        assert_eq!(token.balance_of(&pharmacy), pharmacy_order);
    }

    /// Test governance and voting system
    #[test]
    fn test_healthcare_governance() {
        let governance_contract = Utils::generate_random_address();
        let stakeholder1 = Utils::generate_random_address();
        let stakeholder2 = Utils::generate_random_address();
        let stakeholder3 = Utils::generate_random_address();
        
        let mut token = MediToken::new(Utils::ether_to_wei(1000.0), governance_contract.clone());
        
        // Distribute voting tokens
        let voting_power1 = Utils::ether_to_wei(300.0);
        let voting_power2 = Utils::ether_to_wei(250.0);
        let voting_power3 = Utils::ether_to_wei(200.0);
        
        token.transfer(&governance_contract, &stakeholder1, voting_power1).unwrap();
        token.transfer(&governance_contract, &stakeholder2, voting_power2).unwrap();
        token.transfer(&governance_contract, &stakeholder3, voting_power3).unwrap();
        
        // Simulate voting power calculation
        fn calculate_voting_power(token: &MediToken, voter: &str) -> f64 {
            let balance = token.balance_of(voter);
            Utils::wei_to_ether(balance)
        }
        
        let total_voting_power = calculate_voting_power(&token, &stakeholder1) +
                               calculate_voting_power(&token, &stakeholder2) +
                               calculate_voting_power(&token, &stakeholder3);
        
        assert_eq!(total_voting_power, 750.0); // 300 + 250 + 200
        
        // Test minimum voting threshold
        let min_voting_threshold = 100.0; // 100 tokens minimum to vote
        assert!(calculate_voting_power(&token, &stakeholder1) >= min_voting_threshold);
        assert!(calculate_voting_power(&token, &stakeholder2) >= min_voting_threshold);
        assert!(calculate_voting_power(&token, &stakeholder3) >= min_voting_threshold);
    }

    /// Test patient data ownership and access
    #[test]
    fn test_patient_data_ownership() {
        let patient = Utils::generate_random_address();
        let doctor1 = Utils::generate_random_address();
        let doctor2 = Utils::generate_random_address();
        let researcher = Utils::generate_random_address();
        
        let mut token = MediToken::new(Utils::ether_to_wei(100.0), patient.clone());
        
        // Patient grants access to doctors (tokens represent access rights)
        let doctor_access_fee = Utils::ether_to_wei(10.0);
        let research_access_fee = Utils::ether_to_wei(5.0);
        
        // Doctor 1 pays for access
        token.balances.insert(doctor1.clone(), doctor_access_fee);
        token.transfer(&doctor1, &patient, doctor_access_fee).unwrap();
        
        // Doctor 2 pays for access  
        token.balances.insert(doctor2.clone(), doctor_access_fee);
        token.transfer(&doctor2, &patient, doctor_access_fee).unwrap();
        
        // Researcher pays for anonymized data access
        token.balances.insert(researcher.clone(), research_access_fee);
        token.transfer(&researcher, &patient, research_access_fee).unwrap();
        
        let total_access_payments = doctor_access_fee * 2 + research_access_fee;
        let expected_patient_balance = Utils::ether_to_wei(100.0) + total_access_payments;
        
        assert_eq!(token.balance_of(&patient), expected_patient_balance);
        assert_eq!(token.balance_of(&doctor1), 0);
        assert_eq!(token.balance_of(&doctor2), 0);
        assert_eq!(token.balance_of(&researcher), 0);
    }

    /// Test emergency access override system
    #[test]
    fn test_emergency_access_system() {
        let emergency_responder = Utils::generate_random_address();
        let patient = Utils::generate_random_address();
        let hospital_admin = Utils::generate_random_address();
        
        let mut token = MediToken::new(Utils::ether_to_wei(1000.0), hospital_admin.clone());
        
        // Give patient some tokens
        let patient_tokens = Utils::ether_to_wei(50.0);
        token.transfer(&hospital_admin, &patient, patient_tokens).unwrap();
        
        // Emergency responder needs immediate access
        let emergency_access_tokens = Utils::ether_to_wei(100.0);
        
        // Hospital admin grants emergency access
        token.transfer(&hospital_admin, &emergency_responder, emergency_access_tokens).unwrap();
        
        // Verify emergency responder has sufficient access
        assert!(token.balance_of(&emergency_responder) >= Utils::ether_to_wei(50.0));
        
        // After emergency, tokens can be reclaimed
        let remaining_tokens = token.balance_of(&emergency_responder);
        token.transfer(&emergency_responder, &hospital_admin, remaining_tokens).unwrap();
        
        assert_eq!(token.balance_of(&emergency_responder), 0);
    }
}