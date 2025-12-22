/*
 * Copyright (c) 2024 Captain Light. All rights reserved.
 * IndrasNet DAO Orchestrator - Coordinates complex operations between subprograms via CPI
 * Created by: Captain Light
 * Contact: info@indrasnet.ee
 * GitLab: https://gitlab.com/IndrasNet/indrasnet-dao-v3
 */

// CPI imports for subprograms
// These will be uncommented as subprograms are migrated
// use indrasnet_dao_core::cpi::accounts::*;
// use indrasnet_dao_ai::cpi::accounts::*;
// use indrasnet_dao_security::cpi::accounts::*;
// use indrasnet_dao_partnerships::cpi::accounts::*;

// ===== WORKFLOW FUNCTIONS =====
// Workflow functions coordinate complex operations between subprograms via CPI.
// Each workflow function:
// 1. Validates inputs
// 2. Calls subprograms via CPI in the correct order
// 3. Handles errors and provides meaningful messages
// 4. Does NOT contain business logic (that's in subprograms)

pub mod workflows;

pub use workflows::*;

// All workflow functions are implemented in workflows.rs
