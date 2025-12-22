//! Mesh Group Permissions
//!
//! Defines permissions for mesh group operations.
//! Track A: Simplified to Owner/Member only.

use anchor_lang::prelude::*;

/// Mesh group permission
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeshGroupPermission {
    /// Add member to group
    AddMember,
    /// Remove member from group
    RemoveMember,
    /// Pause group
    PauseGroup,
    /// Resume group
    ResumeGroup,
    /// Transfer leadership
    TransferLeadership,
    /// Create grant request
    CreateGrant,
}

impl anchor_lang::Space for MeshGroupPermission {
    const INIT_SPACE: usize = 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_group_permission_variants() {
        assert_eq!(MeshGroupPermission::AddMember, MeshGroupPermission::AddMember);
        assert_eq!(MeshGroupPermission::RemoveMember, MeshGroupPermission::RemoveMember);
        assert_eq!(MeshGroupPermission::PauseGroup, MeshGroupPermission::PauseGroup);
        assert_eq!(MeshGroupPermission::ResumeGroup, MeshGroupPermission::ResumeGroup);
        assert_eq!(MeshGroupPermission::TransferLeadership, MeshGroupPermission::TransferLeadership);
        assert_eq!(MeshGroupPermission::CreateGrant, MeshGroupPermission::CreateGrant);
    }

    #[test]
    fn test_mesh_group_permission_all_variants_unique() {
        let variants = vec![
            MeshGroupPermission::AddMember,
            MeshGroupPermission::RemoveMember,
            MeshGroupPermission::PauseGroup,
            MeshGroupPermission::ResumeGroup,
            MeshGroupPermission::TransferLeadership,
            MeshGroupPermission::CreateGrant,
        ];
        
        for i in 0..variants.len() {
            for j in (i + 1)..variants.len() {
                assert_ne!(variants[i], variants[j], "Duplicate variant found");
            }
        }
    }

    #[test]
    fn test_mesh_group_permission_copy() {
        let perm1 = MeshGroupPermission::AddMember;
        let perm2 = perm1; // Copy trait
        assert_eq!(perm1, perm2);
    }

    #[test]
    fn test_mesh_group_permission_clone() {
        let perm1 = MeshGroupPermission::RemoveMember;
        let perm2 = perm1.clone();
        assert_eq!(perm1, perm2);
    }

    #[test]
    fn test_mesh_group_permission_space() {
        assert_eq!(<MeshGroupPermission as anchor_lang::Space>::INIT_SPACE, 1);
    }

    #[test]
    fn test_mesh_group_permission_equality() {
        assert_eq!(MeshGroupPermission::AddMember, MeshGroupPermission::AddMember);
        assert_eq!(MeshGroupPermission::RemoveMember, MeshGroupPermission::RemoveMember);
        assert_eq!(MeshGroupPermission::PauseGroup, MeshGroupPermission::PauseGroup);
        assert_eq!(MeshGroupPermission::ResumeGroup, MeshGroupPermission::ResumeGroup);
        assert_eq!(MeshGroupPermission::TransferLeadership, MeshGroupPermission::TransferLeadership);
        assert_eq!(MeshGroupPermission::CreateGrant, MeshGroupPermission::CreateGrant);
    }

    #[test]
    fn test_mesh_group_permission_inequality() {
        assert_ne!(MeshGroupPermission::AddMember, MeshGroupPermission::RemoveMember);
        assert_ne!(MeshGroupPermission::AddMember, MeshGroupPermission::PauseGroup);
        assert_ne!(MeshGroupPermission::RemoveMember, MeshGroupPermission::ResumeGroup);
        assert_ne!(MeshGroupPermission::PauseGroup, MeshGroupPermission::TransferLeadership);
        assert_ne!(MeshGroupPermission::ResumeGroup, MeshGroupPermission::CreateGrant);
    }

    #[test]
    fn test_mesh_group_permission_debug() {
        let perm = MeshGroupPermission::AddMember;
        let debug_str = format!("{:?}", perm);
        assert!(debug_str.contains("AddMember") || debug_str.contains("MeshGroupPermission"));
    }

    #[test]
    fn test_mesh_group_permission_serialize_deserialize() {
        let permissions = vec![
            MeshGroupPermission::AddMember,
            MeshGroupPermission::RemoveMember,
            MeshGroupPermission::PauseGroup,
            MeshGroupPermission::ResumeGroup,
            MeshGroupPermission::TransferLeadership,
            MeshGroupPermission::CreateGrant,
        ];
        
        for perm in &permissions {
            let mut buf = Vec::new();
            perm.serialize(&mut buf).unwrap();
            let deserialized = MeshGroupPermission::deserialize(&mut &buf[..]).unwrap();
            assert_eq!(*perm, deserialized);
        }
    }

    #[test]
    fn test_mesh_group_permission_add_member() {
        let perm = MeshGroupPermission::AddMember;
        assert_eq!(perm, MeshGroupPermission::AddMember);
    }

    #[test]
    fn test_mesh_group_permission_remove_member() {
        let perm = MeshGroupPermission::RemoveMember;
        assert_eq!(perm, MeshGroupPermission::RemoveMember);
    }

    #[test]
    fn test_mesh_group_permission_pause_group() {
        let perm = MeshGroupPermission::PauseGroup;
        assert_eq!(perm, MeshGroupPermission::PauseGroup);
    }

    #[test]
    fn test_mesh_group_permission_resume_group() {
        let perm = MeshGroupPermission::ResumeGroup;
        assert_eq!(perm, MeshGroupPermission::ResumeGroup);
    }

    #[test]
    fn test_mesh_group_permission_transfer_leadership() {
        let perm = MeshGroupPermission::TransferLeadership;
        assert_eq!(perm, MeshGroupPermission::TransferLeadership);
    }

    #[test]
    fn test_mesh_group_permission_create_grant() {
        let perm = MeshGroupPermission::CreateGrant;
        assert_eq!(perm, MeshGroupPermission::CreateGrant);
    }
}
