# AI Compliance - How It Works

## 🔍 Current Situation

When creating an idea via `createIdea` in `ideaService.ts`:
- ✅ Idea is created in Core program
- ❌ AI analysis **NOT** executed automatically
- ❌ AI compliance check missing

## 📋 How AI Compliance Should Work

### Process:

1. **Idea Creation** → Core program (`create_idea`)
   - Idea created with status `Draft`
   
2. **AI Analysis** → AI program (`analyze_idea`)
   - Called **separately** after idea creation
   - Requires off-chain AI service (Gemini API) to get scores
   - Records results on-chain in PDA `ai_analysis`

3. **Status Update** → AI program (`update_idea_status_from_analysis`)
   - Based on analysis updates idea status:
     - `Approved` → idea can join mesh group
     - `Rejected` → idea rejected
     - `UnderReview` → requires human appeal

### AI Compliance Criteria:

- **Charter Compliance** (≥50): DAO charter compliance
- **Governance Compliance** (≥50): Governance rules compliance
- **Ethics Compliance** (≥50): Ethical requirements
- **Legal Compliance** (≥50): Legal requirements
- **Technical Feasibility** (≥70): Technical feasibility
- **Uniqueness** (≥70): **CRITICAL!** Idea uniqueness
- **Impact** (≥70): Potential impact
- **Feasibility** (≥70): Overall feasibility

### Weights for Overall Score:
- Charter: 15%
- Governance: 10%
- Ethics: 10%
- Legal: 15%
- Technical: 10%
- **Uniqueness: 30%** (most important!)
- Impact: 10%

## 🚧 Why It Doesn't Work Now

1. **Orchestrator workflow skips AI analysis**:
   - Current workflow: `createIdea` → directly to mesh group
   - Missing: AI analysis step

2. **AI analysis requires off-chain service**:
   - Needs Gemini API connection
   - Needs off-chain AI service running
   - Currently not integrated

3. **Manual AI analysis required**:
   - Need to call `analyze_idea` separately
   - Need to wait for AI service response
   - Need to call `update_idea_status_from_analysis`

## ✅ Solution

### Option 1: Manual AI Analysis (Current)

After creating idea:
1. Call `analyze_idea` instruction
2. Wait for AI service response
3. Call `update_idea_status_from_analysis`

### Option 2: Automated AI Analysis (Future)

Integrate into orchestrator workflow:
1. `create_idea` → creates idea with `Draft` status
2. Automatically trigger `analyze_idea`
3. Automatically call `update_idea_status_from_analysis`
4. Idea status updated based on AI analysis

## 📝 Implementation Notes

- AI analysis requires off-chain AI service
- Gemini API integration needed
- AI service must be running and accessible
- On-chain storage of AI analysis results in PDA

---

**Status:** ⚠️ Manual AI analysis required  
**Future:** Automated AI analysis integration planned
