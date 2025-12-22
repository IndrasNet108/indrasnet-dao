//! Proposal Template module
//!
//! Handles proposal templates for standardized proposal creation

use anchor_lang::prelude::*;
use crate::error::IndrasError;

/// Template field definition
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, InitSpace)]
pub struct TemplateField {
    #[max_len(50)]
    pub name: String,
    #[max_len(200)]
    pub description: String,
    pub field_type: TemplateFieldType,
    pub required: bool,
}

/// Template field type
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, InitSpace)]
pub enum TemplateFieldType {
    Text,
    Number,
    Date,
    Choice, // For dropdown/choice fields (choices stored separately if needed)
}

/// Proposal Template account structure
#[account]
#[derive(InitSpace)]
pub struct ProposalTemplate {
    pub template_id: u64,
    #[max_len(100)]
    pub name: String,
    #[max_len(500)]
    pub description: String,
    #[max_len(50)]
    pub proposal_type: String,
    #[max_len(20)]
    pub fields: Vec<TemplateField>, // Max 20 fields
    pub created_by: Pubkey,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub is_active: bool,
    pub bump: u8,
}

impl ProposalTemplate {
    /// Create a new proposal template
    pub fn new(
        template_id: u64,
        name: String,
        description: String,
        proposal_type: String,
        fields: Vec<TemplateField>,
        created_by: Pubkey,
        bump: u8,
    ) -> Result<Self> {
        Self::new_with_time(
            template_id,
            name,
            description,
            proposal_type,
            fields,
            created_by,
            bump,
            Clock::get()?.unix_timestamp,
        )
    }

    /// Create a new proposal template with specified time
    pub fn new_with_time(
        template_id: u64,
        name: String,
        description: String,
        proposal_type: String,
        fields: Vec<TemplateField>,
        created_by: Pubkey,
        bump: u8,
        current_time: i64,
    ) -> Result<Self> {
        require!(!name.is_empty(), IndrasError::InvalidInput);
        require!(name.len() <= 100, IndrasError::InvalidInput);
        require!(!description.is_empty(), IndrasError::InvalidInput);
        require!(description.len() <= 500, IndrasError::InvalidInput);
        require!(!proposal_type.is_empty(), IndrasError::InvalidInput);
        require!(proposal_type.len() <= 50, IndrasError::InvalidInput);
        require!(fields.len() <= 20, IndrasError::InvalidInput); // Max 20 fields

        // Validate all fields
        for field in &fields {
            require!(!field.name.is_empty(), IndrasError::InvalidInput);
            require!(field.name.len() <= 50, IndrasError::InvalidInput);
            require!(field.description.len() <= 200, IndrasError::InvalidInput);
        }

        Ok(Self {
            template_id,
            name,
            description,
            proposal_type,
            fields,
            created_by,
            created_at: current_time,
            updated_at: None,
            is_active: true,
            bump,
        })
    }

    /// Update template
    pub fn update(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        fields: Option<Vec<TemplateField>>,
    ) -> Result<()> {
        self.update_with_time(name, description, fields, Clock::get()?.unix_timestamp)
    }

    /// Update template with specified time
    pub fn update_with_time(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        fields: Option<Vec<TemplateField>>,
        current_time: i64,
    ) -> Result<()> {

        if let Some(new_name) = name {
            require!(!new_name.is_empty(), IndrasError::InvalidInput);
            require!(new_name.len() <= 100, IndrasError::InvalidInput);
            self.name = new_name;
        }

        if let Some(new_description) = description {
            require!(!new_description.is_empty(), IndrasError::InvalidInput);
            require!(new_description.len() <= 500, IndrasError::InvalidInput);
            self.description = new_description;
        }

        if let Some(new_fields) = fields {
            require!(new_fields.len() <= 20, IndrasError::InvalidInput);
            // Validate all fields
            for field in &new_fields {
                require!(!field.name.is_empty(), IndrasError::InvalidInput);
                require!(field.name.len() <= 50, IndrasError::InvalidInput);
                require!(field.description.len() <= 200, IndrasError::InvalidInput);
            }
            self.fields = new_fields;
        }

        self.updated_at = Some(current_time);
        Ok(())
    }

    /// Deactivate template
    pub fn deactivate(&mut self) -> Result<()> {
        self.deactivate_with_time(Clock::get()?.unix_timestamp)
    }

    /// Deactivate template with specified time
    pub fn deactivate_with_time(&mut self, current_time: i64) -> Result<()> {
        require!(self.is_active, IndrasError::InvalidState);
        self.is_active = false;
        self.updated_at = Some(current_time);
        Ok(())
    }

    /// Activate template
    pub fn activate(&mut self) -> Result<()> {
        self.activate_with_time(Clock::get()?.unix_timestamp)
    }

    /// Activate template with specified time
    pub fn activate_with_time(&mut self, current_time: i64) -> Result<()> {
        require!(!self.is_active, IndrasError::InvalidState);
        self.is_active = true;
        self.updated_at = Some(current_time);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::Pubkey;

    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::from([seed; 32])
    }

    fn create_test_field(name: &str) -> TemplateField {
        TemplateField {
            name: name.to_string(),
            description: "Test field".to_string(),
            field_type: TemplateFieldType::Text,
            required: false,
        }
    }

    #[test]
    fn test_proposal_template_new_with_time() {
        let author = create_test_pubkey(1);
        let fields = vec![
            create_test_field("field1"),
            create_test_field("field2"),
        ];
        
        let template = ProposalTemplate::new_with_time(
            1,
            "Test Template".to_string(),
            "Test Description".to_string(),
            "governance".to_string(),
            fields,
            author,
            255,
            1000,
        ).unwrap();

        assert_eq!(template.template_id, 1);
        assert_eq!(template.name, "Test Template");
        assert_eq!(template.fields.len(), 2);
        assert_eq!(template.created_by, author);
        assert_eq!(template.created_at, 1000);
        assert!(template.is_active);
    }

    #[test]
    fn test_proposal_template_validation_too_many_fields() {
        let author = create_test_pubkey(1);
        let fields: Vec<TemplateField> = (0..21)
            .map(|i| create_test_field(&format!("field{}", i)))
            .collect();
        
        let result = ProposalTemplate::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            fields,
            author,
            255,
            1000,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_proposal_template_update() {
        let author = create_test_pubkey(1);
        let mut template = ProposalTemplate::new_with_time(
            1,
            "Old Name".to_string(),
            "Old Description".to_string(),
            "governance".to_string(),
            vec![create_test_field("field1")],
            author,
            255,
            1000,
        ).unwrap();

        assert!(template.update_with_time(
            Some("New Name".to_string()),
            Some("New Description".to_string()),
            None,
            2000,
        ).is_ok());

        assert_eq!(template.name, "New Name");
        assert_eq!(template.description, "New Description");
        assert!(template.updated_at.is_some());
    }

    #[test]
    fn test_proposal_template_deactivate() {
        let author = create_test_pubkey(1);
        let mut template = ProposalTemplate::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            vec![],
            author,
            255,
            1000,
        ).unwrap();

        assert!(template.deactivate_with_time(2000).is_ok());
        assert!(!template.is_active);
        assert!(template.updated_at.is_some());
    }

    #[test]
    fn test_proposal_template_activate() {
        let author = create_test_pubkey(1);
        let mut template = ProposalTemplate::new_with_time(
            1,
            "Test".to_string(),
            "Description".to_string(),
            "governance".to_string(),
            vec![],
            author,
            255,
            1000,
        ).unwrap();

        template.is_active = false;
        assert!(template.activate_with_time(2000).is_ok());
        assert!(template.is_active);
    }
}
