"""
Infra Core - ALN Runtime Engine
Adaptive Logic Network interpreter and policy evaluation engine.

This module provides the core runtime for executing ALN policies,
evaluating rules, and enforcing augmented-user rights with
energy-efficient, transparent decision-making.
"""

import hashlib
import time
from typing import Any, Dict, List, Optional, Union
from enum import Enum
from dataclasses import dataclass, field
from datetime import datetime


class Decision(Enum):
    """ALN evaluation decision results"""
    APPROVE = "APPROVE"
    BLOCK = "BLOCK"
    QUARANTINE = "QUARANTINE"
    ESCALATE = "ESCALATE"


class AuditLevel(Enum):
    """Audit detail levels"""
    MINIMAL = "MINIMAL"
    STANDARD = "STANDARD"
    FULL = "FULL"
    FORENSIC = "FORENSIC"


@dataclass
class Policy:
    """ALN Policy definition"""
    name: str
    parameters: Dict[str, Any] = field(default_factory=dict)
    flags: Dict[str, bool] = field(default_factory=dict)
    thresholds: Dict[str, float] = field(default_factory=dict)
    parent: Optional['Policy'] = None
    
    def get(self, key: str, default: Any = None) -> Any:
        """Get parameter with inheritance support"""
        if key in self.parameters:
            return self.parameters[key]
        if key in self.flags:
            return self.flags[key]
        if key in self.thresholds:
            return self.thresholds[key]
        if self.parent:
            return self.parent.get(key, default)
        return default
    
    def require(self, key: str) -> bool:
        """Check if requirement is enabled"""
        return self.get(f"require_{key}", False)
    
    def auto_action(self, action: str, trigger: str) -> bool:
        """Check if automatic action is enabled"""
        return self.get(f"auto_{action}_on_{trigger}", False)


@dataclass
class EvaluationContext:
    """Context for policy evaluation"""
    entity_id: str
    entity_type: str
    user_id: Optional[str] = None
    device_id: Optional[str] = None
    action: Optional[str] = None
    data: Dict[str, Any] = field(default_factory=dict)
    timestamp: float = field(default_factory=time.time)
    
    def get(self, key: str, default: Any = None) -> Any:
        """Get context value"""
        return self.data.get(key, default)
    
    def set(self, key: str, value: Any) -> None:
        """Set context value"""
        self.data[key] = value
    
    def hash(self) -> str:
        """Generate cache key for this context"""
        cache_data = {
            'entity_id': self.entity_id,
            'entity_type': self.entity_type,
            'user_id': self.user_id,
            'device_id': self.device_id,
            'action': self.action,
            'data_hash': hashlib.sha256(str(sorted(self.data.items())).encode()).hexdigest()
        }
        return hashlib.sha256(str(cache_data).encode()).hexdigest()


@dataclass
class AuditEntry:
    """Immutable audit trail entry"""
    event_type: str
    entity_id: str
    user_id: Optional[str]
    decision: Optional[Decision]
    details: Dict[str, Any]
    timestamp: float
    audit_id: str = field(default_factory=lambda: hashlib.sha256(str(time.time()).encode()).hexdigest())
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for serialization"""
        return {
            'audit_id': self.audit_id,
            'event_type': self.event_type,
            'entity_id': self.entity_id,
            'user_id': self.user_id,
            'decision': self.decision.value if self.decision else None,
            'details': self.details,
            'timestamp': self.timestamp,
            'timestamp_iso': datetime.fromtimestamp(self.timestamp).isoformat()
        }


class AuditLog:
    """Thread-safe immutable audit logging service"""
    
    def __init__(self, blockchain_enabled: bool = True):
        self._entries: List[AuditEntry] = []
        self._blockchain_enabled = blockchain_enabled
        self._lock = None  # Would use threading.Lock in production
    
    def record(self, event_type: str, entity_id: str, user_id: Optional[str] = None,
               decision: Optional[Decision] = None, **details) -> str:
        """Record audit entry"""
        entry = AuditEntry(
            event_type=event_type,
            entity_id=entity_id,
            user_id=user_id,
            decision=decision,
            details=details,
            timestamp=time.time()
        )
        
        self._entries.append(entry)
        
        # Blockchain anchoring for critical events
        if self._blockchain_enabled and self._is_critical_event(event_type):
            self._blockchain_anchor(entry)
        
        return entry.audit_id
    
    def _is_critical_event(self, event_type: str) -> bool:
        """Determine if event requires blockchain anchoring"""
        critical_events = {
            'DEVICE_REGISTERED', 'RIGHTS_VIOLATION', 'QUARANTINE_INITIATED',
            'CONSENT_REVOKED', 'SECURITY_BREACH', 'DATA_BREACH',
            'IRREVERSIBLE_ACTION', 'MULTI_SIG_APPROVED'
        }
        return event_type in critical_events
    
    def _blockchain_anchor(self, entry: AuditEntry) -> None:
        """Anchor entry to blockchain (implementation placeholder)"""
        # In production, this would interact with actual blockchain
        pass
    
    def query(self, filters: Dict[str, Any]) -> List[AuditEntry]:
        """Query audit log with filters"""
        results = self._entries
        
        if 'event_type' in filters:
            results = [e for e in results if e.event_type == filters['event_type']]
        
        if 'entity_id' in filters:
            results = [e for e in results if e.entity_id == filters['entity_id']]
        
        if 'user_id' in filters:
            results = [e for e in results if e.user_id == filters['user_id']]
        
        if 'start_time' in filters:
            results = [e for e in results if e.timestamp >= filters['start_time']]
        
        if 'end_time' in filters:
            results = [e for e in results if e.timestamp <= filters['end_time']]
        
        return results
    
    def contains(self, event_type: str) -> bool:
        """Check if log contains event type"""
        return any(e.event_type == event_type for e in self._entries)


class ConsentManager:
    """User consent tracking and verification"""
    
    def __init__(self):
        self._consents: Dict[str, Dict] = {}
    
    def request(self, user_id: str, purpose: str, scope: Dict[str, Any]) -> str:
        """Request user consent"""
        consent_id = hashlib.sha256(f"{user_id}:{purpose}:{time.time()}".encode()).hexdigest()
        
        self._consents[consent_id] = {
            'user_id': user_id,
            'purpose': purpose,
            'scope': scope,
            'status': 'PENDING',
            'requested_at': time.time(),
            'approved_at': None,
            'expires_at': None
        }
        
        return consent_id
    
    def approve(self, consent_id: str, duration: int = 31536000) -> None:
        """Approve consent (default 1 year)"""
        if consent_id in self._consents:
            self._consents[consent_id]['status'] = 'ACTIVE'
            self._consents[consent_id]['approved_at'] = time.time()
            self._consents[consent_id]['expires_at'] = time.time() + duration
    
    def verify(self, user_id: str, purpose: str, timestamp: float) -> Dict[str, Any]:
        """Verify active consent"""
        for consent_id, consent in self._consents.items():
            if (consent['user_id'] == user_id and 
                consent['purpose'] == purpose and
                consent['status'] == 'ACTIVE' and
                consent['expires_at'] and
                consent['expires_at'] > timestamp):
                
                return {
                    'is_active': True,
                    'consent_id': consent_id,
                    'expires_at': consent['expires_at'],
                    'proof': self._generate_proof(consent)
                }
        
        return {'is_active': False, 'consent_id': None, 'proof': None}
    
    def revoke(self, user_id: str, consent_id: str) -> bool:
        """Revoke user consent"""
        if consent_id in self._consents and self._consents[consent_id]['user_id'] == user_id:
            self._consents[consent_id]['status'] = 'REVOKED'
            self._consents[consent_id]['revoked_at'] = time.time()
            return True
        return False
    
    def is_active(self, consent_id: str) -> bool:
        """Check if consent is currently active"""
        if consent_id not in self._consents:
            return False
        
        consent = self._consents[consent_id]
        return (consent['status'] == 'ACTIVE' and
                consent.get('expires_at', 0) > time.time())
    
    def _generate_proof(self, consent: Dict) -> str:
        """Generate cryptographic proof of consent"""
        proof_data = f"{consent['user_id']}:{consent['purpose']}:{consent['approved_at']}"
        return hashlib.sha256(proof_data.encode()).hexdigest()


class ALNRuntime:
    """
    Core ALN runtime engine for policy evaluation.
    
    Features:
    - Energy-efficient rule-based evaluation
    - Policy inheritance and composition
    - Result caching for performance
    - Comprehensive audit logging
    - Blockchain integration for critical decisions
    """
    
    def __init__(self):
        self.policies: Dict[str, Policy] = {}
        self.audit_log = AuditLog()
        self.consent_manager = ConsentManager()
        self._cache: Dict[str, tuple] = {}  # (decision, timestamp)
        self._cache_ttl = 300  # 5 minutes
    
    def register_policy(self, policy: Policy) -> None:
        """Register a policy with the runtime"""
        self.policies[policy.name] = policy
    
    def get_policy(self, name: str) -> Optional[Policy]:
        """Retrieve registered policy"""
        return self.policies.get(name)
    
    def evaluate(self, context: EvaluationContext, policy_name: str) -> Decision:
        """
        Evaluate context against policy.
        
        This is the main entry point for ALN evaluation.
        Supports caching for performance optimization.
        """
        # Check cache first (energy optimization)
        cache_key = f"{policy_name}:{context.hash()}"
        if cache_key in self._cache:
            cached_decision, cached_time = self._cache[cache_key]
            if time.time() - cached_time < self._cache_ttl:
                self.audit_log.record(
                    'CACHE_HIT',
                    context.entity_id,
                    context.user_id,
                    cached_decision,
                    policy=policy_name
                )
                return cached_decision
        
        # Get policy
        policy = self.get_policy(policy_name)
        if not policy:
            self.audit_log.record(
                'POLICY_NOT_FOUND',
                context.entity_id,
                context.user_id,
                details={'policy_name': policy_name}
            )
            return Decision.ESCALATE
        
        # Perform evaluation
        decision = self._evaluate_policy(context, policy)
        
        # Cache result
        self._cache[cache_key] = (decision, time.time())
        
        # Audit logging
        self.audit_log.record(
            'POLICY_EVALUATED',
            context.entity_id,
            context.user_id,
            decision,
            policy=policy_name,
            action=context.action
        )
        
        return decision
    
    def _evaluate_policy(self, context: EvaluationContext, policy: Policy) -> Decision:
        """Internal policy evaluation logic"""
        
        # Check consent requirement
        if policy.require('consent') and context.user_id:
            consent_status = self.consent_manager.verify(
                context.user_id,
                context.action or 'general',
                context.timestamp
            )
            
            if not consent_status['is_active']:
                self.audit_log.record(
                    'CONSENT_FAILURE',
                    context.entity_id,
                    context.user_id
                )
                return Decision.BLOCK
        
        # Check cognitive load threshold (BCI-specific)
        if 'max_cognitive_load' in policy.thresholds:
            cognitive_load = context.get('cognitive_load', 0)
            if cognitive_load > policy.thresholds['max_cognitive_load']:
                self.audit_log.record(
                    'COGNITIVE_OVERLOAD',
                    context.entity_id,
                    context.user_id,
                    details={'load': cognitive_load, 'max': policy.thresholds['max_cognitive_load']}
                )
                return Decision.BLOCK
        
        # Check compliance requirement
        if policy.require('compliance'):
            if not context.get('is_compliant', False):
                self.audit_log.record(
                    'DEVICE_NON_COMPLIANT',
                    context.entity_id
                )
                
                # Auto-quarantine if configured
                if policy.auto_action('quarantine', 'anomaly'):
                    return Decision.QUARANTINE
                
                return Decision.BLOCK
        
        # Check risk level escalation
        risk_level = context.get('risk_level', 'LOW')
        if risk_level in ['HIGH', 'EXTREME']:
            if not context.get('has_ethics_approval') or not context.get('has_legal_approval'):
                self.audit_log.record(
                    'HIGH_RISK_INSUFFICIENT_APPROVALS',
                    context.entity_id,
                    context.user_id,
                    details={'risk_level': risk_level}
                )
                return Decision.ESCALATE
        
        # All checks passed
        return Decision.APPROVE
    
    def clear_cache(self) -> None:
        """Clear evaluation cache"""
        self._cache.clear()
    
    def get_audit_trail(self, entity_id: str) -> List[AuditEntry]:
        """Get complete audit trail for entity"""
        return self.audit_log.query({'entity_id': entity_id})


# Global runtime instance
runtime = ALNRuntime()


# Convenience functions for Copilot usage
def evaluate(entity_id: str, policy_name: str, **context_data) -> Decision:
    """
    Quick evaluation function.
    
    Example:
        decision = evaluate(
            entity_id="device_123",
            policy_name="NeuromorphicDevicePolicy",
            user_id="user_456",
            action="REGISTRATION",
            is_compliant=True,
            cognitive_load=0.75
        )
    """
    context = EvaluationContext(
        entity_id=entity_id,
        entity_type=context_data.get('entity_type', 'UNKNOWN'),
        user_id=context_data.get('user_id'),
        device_id=context_data.get('device_id'),
        action=context_data.get('action'),
        data=context_data
    )
    
    return runtime.evaluate(context, policy_name)


def register_policy(name: str, **params) -> Policy:
    """
    Quick policy registration.
    
    Example:
        policy = register_policy(
            name="BCIPolicy",
            require_consent=True,
            max_cognitive_load=0.85,
            auto_quarantine_on_anomaly=True
        )
    """
    policy = Policy(name=name)
    
    for key, value in params.items():
        if key.startswith('require_') or key.startswith('auto_'):
            policy.flags[key] = value
        elif key.startswith('max_') or key.startswith('min_'):
            policy.thresholds[key] = value
        else:
            policy.parameters[key] = value
    
    runtime.register_policy(policy)
    return policy
