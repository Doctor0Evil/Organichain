# Contributing to Infra

Thank you for your interest in contributing to **Infra**, the world's leading augmented-user rights and smart city governance platform! 🎉

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Contribution Workflow](#contribution-workflow)
- [ALN Policy Development](#aln-policy-development)
- [Testing Requirements](#testing-requirements)
- [Documentation Standards](#documentation-standards)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Code Review Process](#code-review-process)
- [Rights & Ethics Compliance](#rights--ethics-compliance)

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming, inclusive, and harassment-free experience for everyone, regardless of:
- Age, body size, disability, ethnicity
- Gender identity and expression
- Level of experience, education, socio-economic status
- Nationality, personal appearance, race, religion
- Sexual identity and orientation
- Augmentation status (augmented vs non-augmented)

### Core Principles

1. **Rights-First**: Respect the 10 Core Augmented-User Rights in all interactions
2. **Transparency**: All decisions and code must be auditable and explainable
3. **Consent**: Obtain explicit consent before using others' ideas or code
4. **Inclusivity**: Welcome contributors from all backgrounds and skill levels
5. **Energy Efficiency**: Prioritize sustainable, low-energy solutions

## How Can I Contribute?

### Reporting Bugs

Before creating a bug report:
1. Check the [issue tracker](https://github.com/Doctor0Evil/Infra/issues)
2. Review the [documentation](docs/)
3. Try to reproduce with the latest version

When filing a bug report, include:
- **Description**: Clear and descriptive title
- **Steps to Reproduce**: Detailed steps to reproduce the issue
- **Expected Behavior**: What you expected to happen
- **Actual Behavior**: What actually happened
- **Environment**: OS, Python version, Infra version
- **Audit Trail**: Relevant log entries or audit records

### Suggesting Enhancements

Enhancement suggestions are welcome! Include:
- **Use Case**: Why is this enhancement needed?
- **Proposed Solution**: How would it work?
- **Alternatives**: What other approaches did you consider?
- **Rights Impact**: How does this affect the 10 Core Rights?
- **Energy Efficiency**: What's the computational cost?

### Contributing Code

We welcome:
- 🐛 Bug fixes
- ✨ New features
- 📝 Documentation improvements
- 🧪 Test coverage expansion
- 🎨 ALN policy examples
- 🔒 Security enhancements
- ⚡ Performance optimizations

## Development Setup

### Prerequisites

- Python 3.11 or higher
- PostgreSQL 14+
- Redis 7+
- Git
- Docker (optional, for full stack)

### Initial Setup

```bash
# Fork and clone repository
git clone https://github.com/YOUR_USERNAME/Infra.git
cd Infra

# Create virtual environment
python -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt
pip install -r requirements-dev.txt  # Development tools

# Copy environment configuration
cp .env.example .env
# Edit .env with your settings

# Initialize database
python scripts/init_db.py

# Load example policies
python scripts/load_policies.py

# Run tests
pytest tests/ -v
```

### Docker Setup (Recommended)

```bash
# Start full stack
docker-compose up -d

# Access API
curl http://localhost:8000/health

# View logs
docker-compose logs -f api
```

## Contribution Workflow

### 1. Create a Branch

```bash
# Create feature branch
git checkout -b feature/your-feature-name

# Or bug fix branch
git checkout -b bugfix/issue-number-description
```

Branch naming conventions:
- `feature/` - New features
- `bugfix/` - Bug fixes
- `docs/` - Documentation
- `test/` - Test improvements
- `refactor/` - Code refactoring
- `perf/` - Performance improvements

### 2. Make Changes

Follow these guidelines:
- **Write Tests**: All code must have test coverage
- **Update Docs**: Document new features and API changes
- **Compliance Manifest**: Include compliance impact assessment
- **ALN Syntax**: Follow ALN language specification
- **Code Style**: Use Black formatter and pass Flake8

### 3. Test Locally

```bash
# Format code
black core/ modules/ api/ examples/

# Lint
flake8 core/ modules/ api/ examples/

# Type check
mypy core/ modules/ api/

# Run tests
pytest tests/ -v --cov

# Run example workflow
python examples/complete_workflow.py
```

### 4. Commit Changes

```bash
# Stage changes
git add .

# Commit with descriptive message
git commit -m "feat: Add cognitive load monitoring for BCI sessions

- Implement real-time cognitive load tracking
- Add automatic threshold enforcement
- Include audit logging for all measurements
- Compliant with Right to Meta-Cognitive Integrity

Refs: #123"
```

### 5. Push and Create Pull Request

```bash
# Push to your fork
git push origin feature/your-feature-name

# Create Pull Request on GitHub
# Include:
# - Clear description of changes
# - Related issue numbers
# - Rights impact assessment
# - Test coverage summary
# - Screenshots (if UI changes)
```

## ALN Policy Development

### Creating New Policies

1. **Define Policy Structure**

```aln
policy YourPolicyName {
  # Configuration
  parameter_name: value
  
  # Flags
  require_something: TRUE
  auto_action_on_trigger: TRUE
  
  # Thresholds
  max_threshold: 0.85
}
```

2. **Implement Evaluation Logic**

```aln
function EvaluateYourPolicy(entity) {
  # Step 1: Pre-checks
  if (!entity.isValid) {
    return BLOCK;
  }
  
  # Step 2: Policy evaluation
  # ... your logic ...
  
  # Step 3: Audit logging
  AuditLog.record("EVALUATION_COMPLETE", entity.id);
  
  return APPROVE;
}
```

3. **Add Documentation**

Include:
- Purpose and use cases
- Rights affected
- Compliance requirements
- Example usage
- Edge cases

4. **Write Tests**

```python
def test_your_policy():
    # Test normal case
    # Test edge cases
    # Test rights violations
    # Test escalation triggers
```

## Testing Requirements

### Minimum Coverage

- **Core Modules**: 90% coverage
- **API Endpoints**: 100% coverage
- **Rights Framework**: 100% coverage
- **Device Manager**: 85% coverage

### Test Categories

1. **Unit Tests** (`tests/unit/`)
   - Test individual functions
   - Mock external dependencies
   - Fast execution (<1s)

2. **Integration Tests** (`tests/integration/`)
   - Test component interactions
   - Use test database/Redis
   - Moderate execution (<30s)

3. **Compliance Tests** (`tests/compliance/`)
   - Verify rights enforcement
   - Check audit trail completeness
   - Validate blockchain anchoring

4. **Performance Tests** (`tests/performance/`)
   - Measure evaluation latency
   - Check throughput under load
   - Verify memory usage

## Documentation Standards

### Code Documentation

```python
def function_name(param1: Type, param2: Type) -> ReturnType:
    """
    Brief one-line description.
    
    Detailed explanation of what the function does,
    including any important behaviors or side effects.
    
    Args:
        param1: Description of first parameter
        param2: Description of second parameter
    
    Returns:
        Description of return value
    
    Raises:
        ExceptionType: When and why this exception occurs
    
    Compliance:
        - HIPAA: Encryption requirement
        - GDPR: Consent verification
        - Right to Auditability: Full logging
    
    Example:
        >>> result = function_name(arg1, arg2)
        >>> print(result)
    """
```

### ALN Policy Documentation

Every ALN policy file must include:
- Header comment with purpose
- Rights affected
- Compliance standards
- Usage examples
- Edge case handling

## Commit Message Guidelines

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Test additions or fixes
- `chore`: Build/tooling changes
- `rights`: Rights framework changes
- `policy`: ALN policy additions/changes

### Example

```
feat(device-manager): Add neuromorphic chip support

- Implement neuromorphic device type
- Add specialized risk assessment
- Include post-quantum encryption
- Extend monitoring for neural patterns

Complies with:
- Right to Meta-Cognitive Integrity
- Right to Privacy
- HIPAA Section 164.312

Refs: #456
Closes: #123
```

## Code Review Process

### Review Criteria

All PRs must pass:
1. ✅ Automated tests (CI/CD pipeline)
2. ✅ Code quality checks (Black, Flake8, MyPy)
3. ✅ Security scan (Bandit)
4. ✅ Rights impact assessment
5. ✅ Documentation review
6. ✅ Peer review (1+ approvals)

### Reviewer Guidelines

When reviewing:
- ✅ Code correctness and logic
- ✅ Test coverage adequacy
- ✅ Documentation completeness
- ✅ Rights framework compliance
- ✅ Energy efficiency considerations
- ✅ Security best practices
- ✅ ALN syntax adherence

### Feedback

Provide:
- **Constructive**: Suggest improvements, not just problems
- **Specific**: Point to exact lines/sections
- **Educational**: Explain the "why" behind suggestions
- **Respectful**: Remember the human behind the code

## Rights & Ethics Compliance

### Ethics Review Required For

- New rights definitions or modifications
- High-risk device types
- Irreversible augmentation features
- Cross-layer boundary changes
- AI/ML model integration
- Legal trigger modifications

### Ethics Committee

Contact: ethics-committee@infra.gov

Required for PRs involving:
- Changes to core rights enforcement
- New device risk categories
- Modifications to quarantine logic
- Escalation policy changes

## Recognition

Contributors will be:
- ✅ Listed in `CONTRIBUTORS.md`
- ✅ Credited in release notes
- ✅ Granted contributor badge
- ✅ Invited to quarterly contributor calls

## Questions?

- 💬 GitHub Discussions: Ask questions
- 📧 Email: dev-support@infra.gov
- 📚 Documentation: `/docs`
- 🐛 Issues: Report bugs

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (see `LICENSE.md`).

---

**Thank you for helping build the future of augmented-user rights and smart city governance!** 🌟

*"Trust-first, compliance-driven, rights-centric development for everyone."*
