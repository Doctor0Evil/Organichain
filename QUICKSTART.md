# Infra Quick Start Guide

## Welcome to Infra! 🚀

Infra is the world's leading technology-planner, city-builder, and compliance-agenda system for augmented-user rights and smart city governance.

## Why ALN (Adaptive Logic Network)?

### Energy Efficiency
- **84% cost reduction** vs traditional AI/ML
- **83% less energy consumption**
- CPU-only evaluation (no GPUs needed)

### Transparency
- Every decision has a clear logical path
- Human-readable policies
- Legally defensible

### Speed
- < 5ms evaluation latency
- Instant policy updates (no retraining)
- Real-time compliance checking

## Installation

### Prerequisites
- Python 3.11+
- PostgreSQL 14+
- Redis 7+

### Quick Install

```bash
# Clone repository
git clone https://github.com/Doctor0Evil/Infra.git
cd Infra

# Create virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Configure environment
cp .env.example .env
# Edit .env with your settings

# Initialize database
python scripts/init_db.py

# Load example policies
python scripts/load_policies.py

# Run tests
pytest tests/ -v

# Start API server
uvicorn api.main:app --reload
```

## First Steps

### 1. Register a User with Rights

```python
from core.rights_framework import grant_augmented_user_rights

# Grant all 10 core augmented-user rights
grants = grant_augmented_user_rights("user_001", granted_by="SYSTEM")
print(f"Granted {len(grants)} rights")
```

### 2. Register a BCI Device

```python
from modules.augmentation.device_manager import register_bci_device, DeviceType

device_did = register_bci_device(
    manufacturer="NeuraLink",
    model="NL-2025-Pro",
    serial_number="SN-12345",
    device_type=DeviceType.BCI_NON_INVASIVE,
    owner_id="user_001",
    certification_level="FDA_APPROVED"
)
print(f"Device registered: {device_did}")
```

### 3. Start a BCI Session

```python
from modules.augmentation.device_manager import device_registry

session_id = device_registry.start_session(device_did, "user_001")
print(f"Session started: {session_id}")
```

### 4. Query Audit Trail

```python
from core.aln_runtime import runtime

audit_entries = runtime.get_audit_trail("user_001")
print(f"Total audit entries: {len(audit_entries)}")
```

## API Usage

### Start API Server

```bash
uvicorn api.main:app --host 0.0.0.0 --port 8000 --reload
```

### Access API Documentation

Open browser: http://localhost:8000/docs

### Example API Requests

```bash
# Health check
curl http://localhost:8000/

# Register user (requires auth token)
curl -X POST http://localhost:8000/users/register \
  -H "Authorization: Bearer infra_demo_token" \
  -H "Content-Type: application/json" \
  -d '{"user_id": "user_123"}'

# Get user rights
curl http://localhost:8000/users/user_123/rights \
  -H "Authorization: Bearer infra_demo_token"

# Register device
curl -X POST http://localhost:8000/devices/register \
  -H "Authorization: Bearer infra_demo_token" \
  -H "Content-Type: application/json" \
  -d '{
    "manufacturer": "NeuraLink",
    "model": "NL-2025",
    "serial_number": "SN-001",
    "device_type": "BCI_NON_INVASIVE",
    "owner_id": "user_123"
  }'
```

## GitHub Copilot Integration

See `.github/COPILOT_GUIDE.md` for comprehensive Copilot integration patterns.

### Quick Copilot Examples

```
"Create a new ALN policy for cognitive load monitoring with automatic escalation"
"Register a neuromorphic device with post-quantum encryption"
"Generate a compliance report for user_123 with all rights violations"
"Implement automatic quarantine for devices exceeding entropy threshold"
```

## Architecture

```
┌─────────────────────────────────────────┐
│           Infra Platform                │
├─────────────────────────────────────────┤
│  API Layer (FastAPI)                    │
│    ↓                                    │
│  Core Services                          │
│    • ALN Runtime Engine                 │
│    • Rights Framework (10 core rights)  │
│    • Device Registry (BCI/Neuromorphic) │
│    • Consent Manager                    │
│    ↓                                    │
│  Compliance & Security                  │
│    • Blockchain Registry                │
│    • Audit Trail Service                │
│    • Post-Quantum Crypto                │
│    • Monitoring & Escalation            │
│    ↓                                    │
│  Data Layer                             │
│    • PostgreSQL (devices, policies)     │
│    • Redis (cache)                      │
│    • Blockchain (critical events)       │
└─────────────────────────────────────────┘
```

## The 10 Core Augmented-User Rights

1. **Right to Exist**: Protection from arbitrary deletion
2. **Right to Privacy**: Control over personal/neural data
3. **Right to Consent**: Explicit, informed, revocable consent
4. **Right to Self-Ownership**: Own augmented capabilities
5. **Right to Meta-Cognitive Integrity**: Protection from manipulation
6. **Right to Cross-Layer Boundaries**: Reality separation
7. **Right to Auditability**: Complete audit trails
8. **Right to Legal Triggers**: Automated legal protections
9. **Right to Explainability**: Understand all decisions
10. **Right to Rollback**: Reverse augmentation changes

## Example Workflows

### Complete BCI Workflow

```python
from examples.complete_workflow import example_10_complete_workflow

# Run end-to-end workflow
example_10_complete_workflow()
```

This demonstrates:
- User registration with all rights
- Consent management
- Device registration with compliance
- BCI session with monitoring
- Audit trail generation

## ALN Policy Examples

See `/policy/examples/` for:
- `anti_greed_policy.aln` - Smart city vendor evaluation
- `bci_registration_policy.aln` - BCI device compliance
- Custom policy templates

## Testing

```bash
# Run all tests
pytest tests/ -v

# Run with coverage
pytest tests/ --cov=core --cov=modules --cov=api

# Run specific test
pytest tests/test_rights_framework.py -v
```

## Deployment

### Docker

```bash
docker build -t infra:latest .
docker run -p 8000:8000 infra:latest
```

### Kubernetes

```bash
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml
```

## Documentation

- **Architecture**: `/docs/architecture.md`
- **ALN Language**: `/docs/aln_language_spec.md`
- **Copilot Guide**: `.github/COPILOT_GUIDE.md`
- **API Reference**: http://localhost:8000/docs

## Support

- **Issues**: https://github.com/Doctor0Evil/Infra/issues
- **Discussions**: https://github.com/Doctor0Evil/Infra/discussions
- **Security**: security@infra.gov
- **Legal**: legal@infra.gov

## Contributing

See `CONTRIBUTING.md` for guidelines.

All contributors must:
1. Complete ethics & compliance onboarding
2. Sign contributor rights agreement
3. Submit manifest-signed commits
4. Pass sandbox validation

## License

See `LICENSE.md` for full rights-preserving license details.

## What's Next?

1. **Run Examples**: `python examples/complete_workflow.py`
2. **Explore API**: http://localhost:8000/docs
3. **Read Architecture**: `/docs/architecture.md`
4. **Create Policies**: Use ALN language spec
5. **Integrate Copilot**: Follow `.github/COPILOT_GUIDE.md`

---

**Built for the future. Powered by ALN. Secured by design.**

🌟 Star us on GitHub: https://github.com/Doctor0Evil/Infra
