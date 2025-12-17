"""
Augmented-User Rights Framework
Comprehensive rights management system for augmented users, BCI devices, and neuromorphic systems.

This module implements the 10 Core Augmented-User Rights:
1. Right to Exist
2. Right to Privacy
3. Right to Consent
4. Right to Self-Ownership
5. Right to Meta-Cognitive Integrity
6. Right to Cross-Layer Boundaries
7. Right to Auditability
8. Right to Legal Triggers
9. Right to Explainability
10. Right to Rollback
"""

from typing import Dict, List, Optional, Any, Set
from enum import Enum
from dataclasses import dataclass, field
from datetime import datetime
import time
import hashlib

from core.aln_runtime import AuditLog, Decision


class RightType(Enum):
    """10 Core Augmented-User Rights"""
    EXISTENCE = "RIGHT_TO_EXIST"
    PRIVACY = "RIGHT_TO_PRIVACY"
    CONSENT = "RIGHT_TO_CONSENT"
    SELF_OWNERSHIP = "RIGHT_TO_SELF_OWNERSHIP"
    META_COGNITIVE_INTEGRITY = "RIGHT_TO_META_COGNITIVE_INTEGRITY"
    CROSS_LAYER_BOUNDARIES = "RIGHT_TO_CROSS_LAYER_BOUNDARIES"
    AUDITABILITY = "RIGHT_TO_AUDITABILITY"
    LEGAL_TRIGGERS = "RIGHT_TO_LEGAL_TRIGGERS"
    EXPLAINABILITY = "RIGHT_TO_EXPLAINABILITY"
    ROLLBACK = "RIGHT_TO_ROLLBACK"


class RightStatus(Enum):
    """Status of a right"""
    ACTIVE = "ACTIVE"
    SUSPENDED = "SUSPENDED"
    VIOLATED = "VIOLATED"
    UNDER_REVIEW = "UNDER_REVIEW"


class ViolationSeverity(Enum):
    """Severity levels for rights violations"""
    LOW = "LOW"
    MODERATE = "MODERATE"
    HIGH = "HIGH"
    CRITICAL = "CRITICAL"
    CATASTROPHIC = "CATASTROPHIC"


@dataclass
class RightDefinition:
    """Definition of an augmented-user right"""
    right_type: RightType
    description: str
    enforcement_level: str  # MANDATORY, RECOMMENDED, OPTIONAL
    legal_basis: List[str]  # GDPR, HIPAA, etc.
    technical_requirements: List[str]
    violation_consequences: Dict[str, str]
    
    def __post_init__(self):
        """Generate unique right ID"""
        self.right_id = hashlib.sha256(
            f"{self.right_type.value}:{self.description}".encode()
        ).hexdigest()[:16]


@dataclass
class RightGrant:
    """Grant of a right to a specific entity"""
    right_id: str
    right_type: RightType
    entity_id: str
    entity_type: str  # USER, DEVICE, AI_AGENT, etc.
    granted_by: str
    granted_at: float
    expires_at: Optional[float]
    status: RightStatus = RightStatus.ACTIVE
    conditions: Dict[str, Any] = field(default_factory=dict)
    blockchain_proof: Optional[str] = None
    
    def is_active(self) -> bool:
        """Check if right grant is currently active"""
        if self.status != RightStatus.ACTIVE:
            return False
        
        if self.expires_at and time.time() > self.expires_at:
            return False
        
        return True
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for serialization"""
        return {
            'right_id': self.right_id,
            'right_type': self.right_type.value,
            'entity_id': self.entity_id,
            'entity_type': self.entity_type,
            'granted_by': self.granted_by,
            'granted_at': self.granted_at,
            'expires_at': self.expires_at,
            'status': self.status.value,
            'conditions': self.conditions,
            'blockchain_proof': self.blockchain_proof
        }


@dataclass
class RightViolation:
    """Record of a rights violation"""
    violation_id: str
    right_type: RightType
    entity_id: str
    violated_by: str
    severity: ViolationSeverity
    description: str
    timestamp: float
    evidence: Dict[str, Any]
    remediation_status: str = "PENDING"
    escalated: bool = False
    blockchain_anchored: bool = False
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for serialization"""
        return {
            'violation_id': self.violation_id,
            'right_type': self.right_type.value,
            'entity_id': self.entity_id,
            'violated_by': self.violated_by,
            'severity': self.severity.value,
            'description': self.description,
            'timestamp': self.timestamp,
            'timestamp_iso': datetime.fromtimestamp(self.timestamp).isoformat(),
            'evidence': self.evidence,
            'remediation_status': self.remediation_status,
            'escalated': self.escalated,
            'blockchain_anchored': self.blockchain_anchored
        }


class RightsRegistry:
    """
    Central registry for augmented-user rights.
    
    Manages:
    - Right definitions (what rights exist)
    - Right grants (who has what rights)
    - Right violations (when rights are violated)
    - Right enforcement (automated protection)
    """
    
    def __init__(self, blockchain_enabled: bool = True):
        self._definitions: Dict[RightType, RightDefinition] = {}
        self._grants: Dict[str, List[RightGrant]] = {}  # entity_id -> [grants]
        self._violations: List[RightViolation] = []
        self._audit_log = AuditLog(blockchain_enabled=blockchain_enabled)
        self._blockchain_enabled = blockchain_enabled
        
        # Initialize core rights
        self._initialize_core_rights()
    
    def _initialize_core_rights(self):
        """Initialize the 10 Core Augmented-User Rights"""
        
        # 1. Right to Exist
        self.define_right(RightDefinition(
            right_type=RightType.EXISTENCE,
            description="Right of augmented entities to exist and persist without arbitrary deletion",
            enforcement_level="MANDATORY",
            legal_basis=["GDPR Article 17 (Right to be Forgotten)", "Universal Declaration of Human Rights"],
            technical_requirements=["Immutable identity ledger", "Deletion appeal process", "Backup mechanisms"],
            violation_consequences={
                "LOW": "Warning and logging",
                "HIGH": "Immediate rollback and incident investigation",
                "CRITICAL": "Legal action and regulatory notification"
            }
        ))
        
        # 2. Right to Privacy
        self.define_right(RightDefinition(
            right_type=RightType.PRIVACY,
            description="Right to control access to personal data, cognitive patterns, and neural information",
            enforcement_level="MANDATORY",
            legal_basis=["GDPR", "HIPAA", "CCPA", "Fourth Amendment"],
            technical_requirements=["Encryption at rest and in transit", "Access control lists", "Zero-knowledge proofs"],
            violation_consequences={
                "LOW": "Access revocation and audit",
                "HIGH": "Data breach protocol and notification",
                "CRITICAL": "Immediate system quarantine and legal escalation"
            }
        ))
        
        # 3. Right to Consent
        self.define_right(RightDefinition(
            right_type=RightType.CONSENT,
            description="Right to explicit, informed, revocable consent for all augmentation actions",
            enforcement_level="MANDATORY",
            legal_basis=["GDPR Article 7", "Informed Consent Doctrine", "Belmont Report"],
            technical_requirements=["Consent tracking system", "Revocation mechanism", "Cryptographic proof"],
            violation_consequences={
                "LOW": "Action blocked and user notified",
                "HIGH": "Session termination and ethics review",
                "CRITICAL": "Immediate rollback and legal escalation"
            }
        ))
        
        # 4. Right to Self-Ownership
        self.define_right(RightDefinition(
            right_type=RightType.SELF_OWNERSHIP,
            description="Right to own one's augmented capabilities, data, and cognitive enhancements",
            enforcement_level="MANDATORY",
            legal_basis=["Property Rights", "Right to Self-Determination", "Bodily Autonomy"],
            technical_requirements=["Decentralized identity", "Self-sovereign data control", "Ownership proofs"],
            violation_consequences={
                "LOW": "Ownership dispute resolution",
                "HIGH": "Forced transfer reversal",
                "CRITICAL": "Legal action for theft/coercion"
            }
        ))
        
        # 5. Right to Meta-Cognitive Integrity
        self.define_right(RightDefinition(
            right_type=RightType.META_COGNITIVE_INTEGRITY,
            description="Right to protection from unwanted cognitive manipulation or thought alteration",
            enforcement_level="MANDATORY",
            legal_basis=["Cognitive Liberty", "Mental Integrity", "Freedom of Thought"],
            technical_requirements=["Cognitive load monitoring", "Manipulation detection", "Emergency shutdown"],
            violation_consequences={
                "LOW": "Warning and monitoring increase",
                "HIGH": "Immediate session termination",
                "CRITICAL": "Device quarantine and criminal investigation"
            }
        ))
        
        # 6. Right to Cross-Layer Boundaries
        self.define_right(RightDefinition(
            right_type=RightType.CROSS_LAYER_BOUNDARIES,
            description="Right to maintain separation between physical, cyber, and meta realities",
            enforcement_level="MANDATORY",
            legal_basis=["Privacy Rights", "Security Best Practices", "Zero Trust Architecture"],
            technical_requirements=["Layer isolation", "Cross-domain guards", "Entropy monitoring"],
            violation_consequences={
                "LOW": "Boundary strengthening",
                "HIGH": "Cross-layer operation blocking",
                "CRITICAL": "Complete system quarantine"
            }
        ))
        
        # 7. Right to Auditability
        self.define_right(RightDefinition(
            right_type=RightType.AUDITABILITY,
            description="Right to access complete, immutable audit trails of all augmentation activities",
            enforcement_level="MANDATORY",
            legal_basis=["GDPR Article 15 (Right of Access)", "HIPAA Audit Controls", "Accountability Principle"],
            technical_requirements=["Blockchain-anchored logs", "Tamper-evident storage", "User-accessible interface"],
            violation_consequences={
                "LOW": "Audit gap documentation",
                "HIGH": "System recertification required",
                "CRITICAL": "Regulatory penalty and operational suspension"
            }
        ))
        
        # 8. Right to Legal Triggers
        self.define_right(RightDefinition(
            right_type=RightType.LEGAL_TRIGGERS,
            description="Right to automated legal protections and emergency intervention mechanisms",
            enforcement_level="MANDATORY",
            legal_basis=["Due Process", "Emergency Powers Doctrine", "Protective Custody"],
            technical_requirements=["Kill switch", "Legal escalation automation", "Emergency contact protocol"],
            violation_consequences={
                "LOW": "Legal notification",
                "HIGH": "Immediate legal involvement",
                "CRITICAL": "Law enforcement notification and evidence preservation"
            }
        ))
        
        # 9. Right to Explainability
        self.define_right(RightDefinition(
            right_type=RightType.EXPLAINABILITY,
            description="Right to understand how and why augmentation decisions are made",
            enforcement_level="MANDATORY",
            legal_basis=["GDPR Article 22 (Automated Decision-Making)", "AI Act", "Algorithmic Accountability"],
            technical_requirements=["Decision tracing", "Plain-language explanations", "Appeal mechanism"],
            violation_consequences={
                "LOW": "Explanation provision required",
                "HIGH": "Decision reversal",
                "CRITICAL": "System decertification"
            }
        ))
        
        # 10. Right to Rollback
        self.define_right(RightDefinition(
            right_type=RightType.ROLLBACK,
            description="Right to reverse augmentation changes and restore previous states",
            enforcement_level="MANDATORY",
            legal_basis=["Right to Remedy", "Undo Doctrine", "Consumer Protection"],
            technical_requirements=["State snapshots", "Rollback procedures", "Recovery testing"],
            violation_consequences={
                "LOW": "Rollback capability restoration",
                "HIGH": "Compensatory measures",
                "CRITICAL": "Product recall and remediation"
            }
        ))
    
    def define_right(self, definition: RightDefinition) -> str:
        """Register a right definition"""
        self._definitions[definition.right_type] = definition
        
        self._audit_log.record(
            'RIGHT_DEFINED',
            definition.right_id,
            details={
                'right_type': definition.right_type.value,
                'enforcement_level': definition.enforcement_level
            }
        )
        
        return definition.right_id
    
    def grant_right(self, right_type: RightType, entity_id: str, entity_type: str,
                   granted_by: str, duration: Optional[int] = None,
                   conditions: Optional[Dict[str, Any]] = None) -> RightGrant:
        """Grant a right to an entity"""
        
        if right_type not in self._definitions:
            raise ValueError(f"Right type {right_type} not defined")
        
        definition = self._definitions[right_type]
        
        expires_at = None
        if duration:
            expires_at = time.time() + duration
        
        grant = RightGrant(
            right_id=definition.right_id,
            right_type=right_type,
            entity_id=entity_id,
            entity_type=entity_type,
            granted_by=granted_by,
            granted_at=time.time(),
            expires_at=expires_at,
            conditions=conditions or {}
        )
        
        # Blockchain anchoring for grant
        if self._blockchain_enabled:
            grant.blockchain_proof = self._blockchain_anchor_grant(grant)
        
        # Store grant
        if entity_id not in self._grants:
            self._grants[entity_id] = []
        self._grants[entity_id].append(grant)
        
        self._audit_log.record(
            'RIGHT_GRANTED',
            entity_id,
            granted_by,
            details={
                'right_type': right_type.value,
                'entity_type': entity_type,
                'expires_at': expires_at
            }
        )
        
        return grant
    
    def grant_all_rights(self, entity_id: str, entity_type: str, granted_by: str) -> List[RightGrant]:
        """Grant all 10 core rights to an entity"""
        grants = []
        for right_type in RightType:
            grant = self.grant_right(right_type, entity_id, entity_type, granted_by)
            grants.append(grant)
        
        self._audit_log.record(
            'ALL_RIGHTS_GRANTED',
            entity_id,
            granted_by,
            details={'entity_type': entity_type, 'count': len(grants)}
        )
        
        return grants
    
    def verify_right(self, entity_id: str, right_type: RightType) -> bool:
        """Verify entity has an active right"""
        if entity_id not in self._grants:
            return False
        
        for grant in self._grants[entity_id]:
            if grant.right_type == right_type and grant.is_active():
                return True
        
        return False
    
    def get_entity_rights(self, entity_id: str) -> List[RightGrant]:
        """Get all rights for an entity"""
        return self._grants.get(entity_id, [])
    
    def record_violation(self, right_type: RightType, entity_id: str, violated_by: str,
                        severity: ViolationSeverity, description: str,
                        evidence: Dict[str, Any]) -> str:
        """Record a rights violation"""
        
        violation_id = hashlib.sha256(
            f"{right_type.value}:{entity_id}:{time.time()}".encode()
        ).hexdigest()
        
        violation = RightViolation(
            violation_id=violation_id,
            right_type=right_type,
            entity_id=entity_id,
            violated_by=violated_by,
            severity=severity,
            description=description,
            timestamp=time.time(),
            evidence=evidence
        )
        
        # Blockchain anchor critical violations
        if severity in [ViolationSeverity.CRITICAL, ViolationSeverity.CATASTROPHIC]:
            if self._blockchain_enabled:
                self._blockchain_anchor_violation(violation)
                violation.blockchain_anchored = True
        
        self._violations.append(violation)
        
        # Audit logging
        self._audit_log.record(
            'RIGHTS_VIOLATION',
            entity_id,
            violated_by,
            details={
                'violation_id': violation_id,
                'right_type': right_type.value,
                'severity': severity.value,
                'description': description
            }
        )
        
        # Auto-escalate high-severity violations
        if severity in [ViolationSeverity.HIGH, ViolationSeverity.CRITICAL, ViolationSeverity.CATASTROPHIC]:
            self._escalate_violation(violation)
            violation.escalated = True
        
        return violation_id
    
    def get_violations(self, entity_id: Optional[str] = None,
                      right_type: Optional[RightType] = None,
                      severity: Optional[ViolationSeverity] = None) -> List[RightViolation]:
        """Query violations with filters"""
        results = self._violations
        
        if entity_id:
            results = [v for v in results if v.entity_id == entity_id]
        
        if right_type:
            results = [v for v in results if v.right_type == right_type]
        
        if severity:
            results = [v for v in results if v.severity == severity]
        
        return results
    
    def _blockchain_anchor_grant(self, grant: RightGrant) -> str:
        """Anchor right grant to blockchain (placeholder)"""
        # In production, interact with actual blockchain
        grant_hash = hashlib.sha256(str(grant.to_dict()).encode()).hexdigest()
        return f"blockchain_proof:{grant_hash}"
    
    def _blockchain_anchor_violation(self, violation: RightViolation) -> None:
        """Anchor violation to blockchain (placeholder)"""
        # In production, interact with actual blockchain
        pass
    
    def _escalate_violation(self, violation: RightViolation) -> None:
        """Escalate violation to appropriate authorities (placeholder)"""
        # In production, trigger escalation service
        pass
    
    def generate_rights_report(self, entity_id: str) -> Dict[str, Any]:
        """Generate comprehensive rights report for entity"""
        grants = self.get_entity_rights(entity_id)
        violations = self.get_violations(entity_id=entity_id)
        
        active_rights = [g for g in grants if g.is_active()]
        
        return {
            'entity_id': entity_id,
            'timestamp': time.time(),
            'total_rights_granted': len(grants),
            'active_rights': len(active_rights),
            'rights': [g.to_dict() for g in grants],
            'total_violations': len(violations),
            'violations_by_severity': {
                'LOW': len([v for v in violations if v.severity == ViolationSeverity.LOW]),
                'MODERATE': len([v for v in violations if v.severity == ViolationSeverity.MODERATE]),
                'HIGH': len([v for v in violations if v.severity == ViolationSeverity.HIGH]),
                'CRITICAL': len([v for v in violations if v.severity == ViolationSeverity.CRITICAL]),
                'CATASTROPHIC': len([v for v in violations if v.severity == ViolationSeverity.CATASTROPHIC])
            },
            'recent_violations': [v.to_dict() for v in violations[-10:]],
            'compliance_status': 'COMPLIANT' if len([v for v in violations if v.remediation_status == 'PENDING']) == 0 else 'NON_COMPLIANT'
        }


# Global rights registry
rights_registry = RightsRegistry()


# Convenience functions for Copilot
def grant_augmented_user_rights(user_id: str, granted_by: str = "SYSTEM") -> List[RightGrant]:
    """Grant all 10 core rights to an augmented user"""
    return rights_registry.grant_all_rights(user_id, "USER", granted_by)


def verify_consent(user_id: str) -> bool:
    """Verify user has active consent right"""
    return rights_registry.verify_right(user_id, RightType.CONSENT)


def verify_privacy(user_id: str) -> bool:
    """Verify user has active privacy right"""
    return rights_registry.verify_right(user_id, RightType.PRIVACY)


def report_violation(right_type: RightType, entity_id: str, violated_by: str,
                    severity: ViolationSeverity, description: str,
                    **evidence) -> str:
    """Report a rights violation"""
    return rights_registry.record_violation(
        right_type, entity_id, violated_by, severity, description, evidence
    )
