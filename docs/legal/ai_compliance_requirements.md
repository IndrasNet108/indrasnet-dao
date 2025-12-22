# AI Compliance Requirements - Detailed Requirements

**Last Updated:** 2024-11-17  
**Status:** Current

---

## 🎯 Overview

AI Compliance is a mandatory check for ideas to ensure compliance with DAO regulations before they can enter mesh groups and request grants.

**CRITICAL:** Without AI compliance, an idea **cannot**:
- Enter a mesh group
- Request a grant

---

## 📋 AI Compliance Criteria

### Minimum Requirements for `can_enter_mesh_group()`:

1. **Ethics Compliance** ≥ 50%
   - Compliance with DAO ethical principles
   - Check for harm, discrimination, rights violations

2. **Legal Compliance** ≥ 50%
   - Compliance with legislation
   - Check for illegality, intellectual property violations

3. **Uniqueness** ≥ 70% ⭐ **CRITICAL!**
   - Idea uniqueness
   - Check for duplicates and plagiarism
   - Weight in overall score: **30%**

4. **Feasibility** ≥ 70% with verified artifacts
   - Idea feasibility
   - Presence of evidence (artifacts)

5. **Impact** ≥ 70%
   - Idea innovativeness
   - Potential impact

### Additional Criteria (for overall score):

6. **Charter Compliance** ≥ 50%
   - Compliance with DAO charter

7. **Governance Compliance** ≥ 50%
   - Compliance with governance rules

8. **Technical Feasibility** ≥ 70%
   - Technical feasibility

9. **Impact** ≥ 70%
   - Potential impact on community

---

## 🔒 Mandatory Checks

### 1. Creating Mesh Group with Idea

**Requirements:**
- ✅ Idea exists
- ✅ Idea has status `Approved`
- ✅ AI analysis exists
- ✅ AI analysis has `decision == Approve`
- ✅ All `can_enter_mesh_group()` criteria met

**Code Check:**
```rust
// Check idea status
require!(idea.status == IdeaStatus::Approved, IndrasError::InvalidState);

// Check AI analysis
require!(ai_analysis.decision == AIDecision::Approve, IndrasError::InvalidState);

// Check compliance criteria
require!(ai_analysis.can_enter_mesh_group(), IndrasError::InvalidState);
```

**Errors:**
- `InvalidState`: Idea not Approved
- `InvalidInput`: AI analysis not provided
- `InvalidState`: AI analysis decision != Approve
- `InvalidState`: `can_enter_mesh_group()` criteria not met

---

### 2. Adding Idea to Mesh Group

**Requirements:**
- ✅ Idea exists
- ✅ Idea has status `Approved`
- ✅ AI analysis exists
- ✅ AI analysis has `decision == Approve`
- ✅ All `can_enter_mesh_group()` criteria met

**Code Check:**
```rust
// Same checks as above
```

**Errors:**
- `InvalidState`: Idea not Approved
- `InvalidInput`: AI analysis not provided
- `InvalidState`: AI analysis decision != Approve

---

### 3. Grant Request

**Requirements:**
- ✅ Idea exists
- ✅ Idea has status `Approved` or `InProgress`
- ✅ Idea is in mesh group
- ✅ AI analysis exists
- ✅ AI analysis has `decision == Approve`
- ✅ All `can_enter_mesh_group()` criteria met

**Code Check:**
```rust
// Check idea status
require!(
    idea.status == IdeaStatus::Approved || idea.status == IdeaStatus::InProgress,
    IndrasError::InvalidState
);

// Deserialize and check decision == Approve
```

**Errors:**
- `InvalidState`: Idea not Approved/InProgress
- `InvalidInput`: AI analysis not provided or empty
- `InvalidState`: AI analysis decision != Approve
- `InvalidState`: `can_enter_mesh_group()` criteria not met

---

## 🔄 Complete Workflow

### Full Cycle:

1. Idea Creation
   └─> Status: Draft

2. AI Analysis (off-chain + on-chain)
   ├─> Off-chain: Gemini API analysis
   ├─> On-chain: analyze_idea() - record results
   └─> On-chain: update_idea_status_from_analysis() - update status

3. AI Analysis Result:
   ├─> Approved → Status: Approved ✅
   ├─> Rejected → Status: Rejected ❌
   └─> Appeal → Status: UnderReview ⚠️

4. Mesh Group Creation (only for Approved)
   ├─> Check: idea.status == Approved
   ├─> Check: AI analysis decision == Approve
   └─> Check: can_enter_mesh_group() == true

5. Grant Request (only for Approved in Mesh Group)
   ├─> Check: idea.status == Approved/InProgress
   ├─> Check: idea in mesh group
   ├─> Check: AI analysis decision == Approve
   └─> Check: can_enter_mesh_group() == true

---

## 💻 Implementation Example

### AI Compliance Check Before Operations:

```rust
// When creating mesh group with idea

  // 1. Check idea status
  require!(
      idea.status == IdeaStatus::Approved,
      IndrasError::InvalidState
  );

  // 2. Check AI analysis
  require!(
      ai_analysis.decision == AIDecision::Approve,
      IndrasError::InvalidState
  );

  // 3. Pass AI analysis account in transaction
```

```rust
// Before grant request

  // 1. Check idea status
  require!(
      idea.status == IdeaStatus::Approved || idea.status == IdeaStatus::InProgress,
      IndrasError::InvalidState
  );

  // 2. Check AI analysis
  require!(
      ai_analysis.decision == AIDecision::Approve,
      IndrasError::InvalidState
  );

  // Error handling
```

---

## 🧪 Testing

### AI Compliance Tests:

File: `tests/ai_compliance_checks.test.ts`

**Tests for Mesh Groups:**
- ✅ Creating mesh group without AI analysis should fail
- ✅ Creating mesh group with Reject decision should fail
- ✅ Creating mesh group with Appeal decision should fail
- ✅ Creating mesh group with Approve decision should pass
- ✅ Adding idea without AI analysis should fail

**Tests for Grants:**
- ✅ Grant request without AI analysis should fail
- ✅ Grant request with Reject decision should fail
- ✅ Grant request with Approve decision should pass

---

## 🐛 Troubleshooting

### Error: "AI analysis not found"

**Cause:** AI analysis was not performed for the idea.

**Solution:**
1. Create idea
2. Perform AI analysis via `aiAnalysisService.createIdeaWithAutoAIAnalysis()`
3. Wait for status update to `Approved`
4. Retry operation

---

### Error: "Idea AI analysis decision is 'Rejected'"

**Cause:** AI analysis rejected the idea.

**Solution:**
1. Check rejection reasons in AI analysis
2. Improve idea (uniqueness, ethics, legal aspects)
3. Create new idea with improvements
4. Retry AI analysis

---

### Error: "Idea is not Approved"

**Cause:** Idea did not pass AI analysis or status not updated.

**Solution:**
1. Check idea status
2. If status is `Draft` or `UnderReview`, perform AI analysis
3. If status is `Rejected`, create new idea
4. Wait for `Approved` status

---

## 🔗 Related Documents

- [AI Compliance Workflow](./AI_COMPLIANCE_WORKFLOW.md) - Complete workflow description
- [AI Compliance Explanation](./ai_compliance_explanation.md) - User explanation
- [Architecture Review](./ARCHITECTURE_REVIEW.md) - Architectural overview

---

**Important:** All checks are performed **on-chain**, ensuring DAO rule compliance at the contract level.
