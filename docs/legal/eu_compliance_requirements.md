# 🌍 EU Compliance Requirements and Required Off-Chain Components

## 📋 Requirements Overview

### 1. **GDPR (General Data Protection Regulation)**

#### Requirements:
- ✅ Right to access data
- ✅ Right to correct data
- ❌ Right to deletion (right to be forgotten) - **blockchain is immutable**
- ✅ Right to data portability
- ❌ Consent for data processing - **requires implementation**
- ✅ Data minimization
- ✅ Processing transparency

#### What needs to be added:

**Off-chain Components:**
1. **GDPR Consent Management System**
   - Store data processing consents
   - Consent revocation mechanism
   - Logging of all consent operations

2. **Personal Data Storage (Off-chain)**
   - Store personal data in IPFS with encryption
   - Personal data hashes in blockchain
   - Data deletion mechanism (off-chain)

3. **Data Subject Rights Portal**
   - API for data access requests
   - API for data correction
   - API for data deletion
   - API for data export (portability)

**On-chain Components (Support):**
- Personal data hashes instead of actual data
- References to off-chain storage
- Consent flags (minimal data)

### 2. **MiCA (Markets in Crypto-Assets Regulation)**

#### Requirements:
- ❌ Crypto-asset registration - **requires implementation**
- ❌ Token issuance prospectus - **requires implementation**
- ✅ Risk management (partially)
- ✅ Reporting (partially)
- ❌ Investor protection - **requires implementation**

#### What needs to be added:

**Off-chain Components:**
1. **Asset Registration System**
   - Register all crypto-assets
   - Asset classification (EMT/ART/Utility/Other)
   - Store issuance prospectuses

2. **Investor Protection System**
   - KYC/AML checks
   - Risk information
   - Refund mechanism for violations

3. **Regulatory Reporting System**
   - Automatic report generation for regulators
   - Store reports in IPFS
   - Notify regulators

**On-chain Components (Support):**
- Asset registration through CommercialEnterprise
- References to issuance prospectuses
- MiCA compliance statuses

### 3. **AI Act (Artificial Intelligence Act)**

#### Requirements:
- ❌ AI system classification (minimal/high risk) - **requires implementation**
- ✅ Transparency requirements (partially)
- ✅ Security requirements (partially)
- ❌ Human oversight requirements - **requires implementation**
- ❌ Prohibition of certain AI practices - **requires implementation**

#### What needs to be added:

**Off-chain Components:**
1. **AI System Classification System**
   - Classify AI systems by risk
   - Register AI systems
   - Document AI systems

2. **Human Oversight System**
   - Human oversight mechanism for critical decisions
   - Log all AI decisions
   - Mechanism to override AI decisions

3. **AI Ethics Compliance System**
   - Check for prohibited AI practices
   - AI ethics principles
   - Regular AI system audits

**On-chain Components (Support):**
- Phenomenon classification by risk
- AI decision metadata
- Human oversight flags

---

## 🔍 **Implementation Status**

**Note**: These requirements describe compliance needs. In MVP:
- Legal modules exist in `programs/indrasnet-dao-core/src/legal/`
- Compliance structures may need implementation
- Off-chain services may need development for full compliance

**Current Status**: Basic legal framework exists, full compliance automation may need implementation.

---

*Document created for IndrasNet DAO*  
*Version 1.0-MVP - January 2025*
