"""
BCI and Neuromorphic Device Management Module
Comprehensive device registration, session management, and augmentation tracking
for Brain-Computer Interfaces and neuromorphic systems.
"""

from typing import Dict, List, Optional, Any, Set
from enum import Enum
from dataclasses import dataclass, field
import time
import hashlib

from core.aln_runtime import runtime, AuditLog, Decision, Policy, EvaluationContext
from core.rights_framework import rights_registry, RightType, ViolationSeverity


class DeviceType(Enum):
    """Types of augmentation devices"""
    BCI_INVASIVE = "BCI_INVASIVE"
    BCI_NON_INVASIVE = "BCI_NON_INVASIVE"
    NEUROMORPHIC_CHIP = "NEUROMORPHIC_CHIP"
    NEURAL_IMPLANT = "NEURAL_IMPLANT"
    COGNITIVE_ENHANCER = "COGNITIVE_ENHANCER"
    SENSORY_AUGMENTATION = "SENSORY_AUGMENTATION"
    AR_VR_HEADSET = "AR_VR_HEADSET"
    WEARABLE_SENSOR = "WEARABLE_SENSOR"


class DeviceStatus(Enum):
    """Device operational status"""
    PENDING_REGISTRATION = "PENDING_REGISTRATION"
    ACTIVE = "ACTIVE"
    SUSPENDED = "SUSPENDED"
    QUARANTINED = "QUARANTINED"
    DEACTIVATED = "DEACTIVATED"
    EXPIRED = "EXPIRED"


class SessionStatus(Enum):
    """BCI session status"""
    INITIALIZING = "INITIALIZING"
    ACTIVE = "ACTIVE"
    PAUSED = "PAUSED"
    TERMINATED = "TERMINATED"
    QUARANTINED = "QUARANTINED"
    ERROR = "ERROR"


@dataclass
class DeviceIdentity:
    """Decentralized Identity (DID) for augmentation device"""
    did: str  # Decentralized Identifier
    device_type: DeviceType
    manufacturer: str
    model: str
    serial_number: str
    firmware_version: str
    certification_level: str  # FDA, CE, EXPERIMENTAL, etc.
    compliance_standards: List[str] = field(default_factory=list)  # HIPAA, GDPR, ISO, etc.
    
    def __post_init__(self):
        """Generate DID if not provided"""
        if not self.did:
            self.did = self._generate_did()
    
    def _generate_did(self) -> str:
        """Generate unique decentralized identifier"""
        id_string = f"{self.manufacturer}:{self.model}:{self.serial_number}:{time.time()}"
        did_hash = hashlib.sha256(id_string.encode()).hexdigest()
        return f"did:infra:device:{did_hash[:32]}"
    
    def to_dict(self) -> Dict[str, Any]:
        return {
            'did': self.did,
            'device_type': self.device_type.value,
            'manufacturer': self.manufacturer,
            'model': self.model,
            'serial_number': self.serial_number,
            'firmware_version': self.firmware_version,
            'certification_level': self.certification_level,
            'compliance_standards': self.compliance_standards
        }


@dataclass
class DeviceRegistration:
    """Complete device registration record"""
    identity: DeviceIdentity
    owner_id: str
    registered_by: str
    registered_at: float
    status: DeviceStatus
    consent_proof: Optional[str] = None
    blockchain_anchor: Optional[str] = None
    risk_assessment: Dict[str, Any] = field(default_factory=dict)
    last_audit: Optional[float] = None
    
    def is_operational(self) -> bool:
        """Check if device can be used"""
        return self.status == DeviceStatus.ACTIVE
    
    def to_dict(self) -> Dict[str, Any]:
        return {
            'identity': self.identity.to_dict(),
            'owner_id': self.owner_id,
            'registered_by': self.registered_by,
            'registered_at': self.registered_at,
            'status': self.status.value,
            'consent_proof': self.consent_proof,
            'blockchain_anchor': self.blockchain_anchor,
            'risk_assessment': self.risk_assessment,
            'last_audit': self.last_audit
        }


@dataclass
class BCISession:
    """BCI/Neuromorphic session tracking"""
    session_id: str
    device_did: str
    user_id: str
    started_at: float
    status: SessionStatus
    cognitive_load: float = 0.0
    entropy_level: float = 0.0
    data_transmitted: int = 0  # bytes
    consent_verified: bool = False
    monitoring_active: bool = False
    anomalies_detected: int = 0
    ended_at: Optional[float] = None
    
    def duration(self) -> float:
        """Calculate session duration in seconds"""
        end = self.ended_at if self.ended_at else time.time()
        return end - self.started_at
    
    def to_dict(self) -> Dict[str, Any]:
        return {
            'session_id': self.session_id,
            'device_did': self.device_did,
            'user_id': self.user_id,
            'started_at': self.started_at,
            'ended_at': self.ended_at,
            'duration': self.duration(),
            'status': self.status.value,
            'cognitive_load': self.cognitive_load,
            'entropy_level': self.entropy_level,
            'data_transmitted': self.data_transmitted,
            'consent_verified': self.consent_verified,
            'monitoring_active': self.monitoring_active,
            'anomalies_detected': self.anomalies_detected
        }


class DeviceRegistry:
    """
    Central registry for all augmentation devices.
    
    Features:
    - DID-based device identity
    - Blockchain-anchored registration
    - Compliance verification
    - Automatic quarantine
    - Rights integration
    """
    
    def __init__(self, blockchain_enabled: bool = True):
        self._devices: Dict[str, DeviceRegistration] = {}
        self._sessions: Dict[str, BCISession] = {}
        self._audit_log = AuditLog(blockchain_enabled=blockchain_enabled)
        self._blockchain_enabled = blockchain_enabled
        
        # Initialize device policies
        self._init_policies()
    
    def _init_policies(self):
        """Initialize device-specific ALN policies"""
        
        # Base neuromorphic device policy
        runtime.register_policy(Policy(
            name="NeuromorphicDevicePolicy",
            parameters={
                'audit_level': 'FULL',
                'encryption_standard': 'POST_QUANTUM',
                'session_timeout': 7200  # 2 hours
            },
            flags={
                'require_consent': True,
                'require_compliance': True,
                'blockchain_anchor': True,
                'auto_rollback_on_violation': True,
                'auto_quarantine_on_anomaly': True
            },
            thresholds={
                'max_cognitive_load': 0.85,
                'max_entropy_level': 0.90,
                'min_session_interval': 3600  # 1 hour between sessions
            }
        ))
        
        # High-risk device policy
        runtime.register_policy(Policy(
            name="HighRiskDevicePolicy",
            parameters={
                'audit_level': 'FORENSIC',
                'encryption_standard': 'POST_QUANTUM',
                'session_timeout': 3600  # 1 hour
            },
            flags={
                'require_consent': True,
                'require_compliance': True,
                'require_ethics_approval': True,
                'require_medical_clearance': True,
                'blockchain_anchor': True,
                'auto_rollback_on_violation': True,
                'auto_quarantine_on_anomaly': True
            },
            thresholds={
                'max_cognitive_load': 0.70,
                'max_entropy_level': 0.75,
                'min_session_interval': 7200,  # 2 hours
                'max_sessions_per_day': 3
            }
        ))
    
    def register_device(self, identity: DeviceIdentity, owner_id: str, 
                       registered_by: str, consent_proof: Optional[str] = None) -> str:
        """
        Register a new augmentation device.
        
        Performs comprehensive compliance checks and rights verification.
        """
        
        # Step 1: Verify owner has necessary rights
        if not rights_registry.verify_right(owner_id, RightType.SELF_OWNERSHIP):
            self._audit_log.record(
                'DEVICE_REGISTRATION_DENIED',
                identity.did,
                owner_id,
                details={'reason': 'Missing self-ownership right'}
            )
            raise PermissionError("Owner lacks self-ownership rights")
        
        if not rights_registry.verify_right(owner_id, RightType.CONSENT):
            self._audit_log.record(
                'DEVICE_REGISTRATION_DENIED',
                identity.did,
                owner_id,
                details={'reason': 'Missing consent right'}
            )
            raise PermissionError("Owner lacks consent rights")
        
        # Step 2: Risk assessment
        risk_assessment = self._assess_device_risk(identity)
        
        # Step 3: Policy evaluation
        context = EvaluationContext(
            entity_id=identity.did,
            entity_type='DEVICE',
            user_id=owner_id,
            device_id=identity.did,
            action='REGISTRATION',
            data={
                'device_type': identity.device_type.value,
                'certification_level': identity.certification_level,
                'risk_level': risk_assessment['risk_level'],
                'is_compliant': self._verify_compliance(identity),
                'has_consent': consent_proof is not None
            }
        )
        
        # Select appropriate policy
        policy_name = self._select_policy(risk_assessment['risk_level'])
        decision = runtime.evaluate(context, policy_name)
        
        if decision == Decision.BLOCK:
            self._audit_log.record(
                'DEVICE_REGISTRATION_BLOCKED',
                identity.did,
                owner_id,
                decision,
                details={'risk_assessment': risk_assessment}
            )
            raise ValueError("Device registration blocked by policy")
        
        if decision == Decision.ESCALATE:
            self._audit_log.record(
                'DEVICE_REGISTRATION_ESCALATED',
                identity.did,
                owner_id,
                decision,
                details={'risk_assessment': risk_assessment}
            )
            # In production, trigger escalation service
            raise ValueError("Device registration requires committee approval")
        
        # Step 4: Create registration
        registration = DeviceRegistration(
            identity=identity,
            owner_id=owner_id,
            registered_by=registered_by,
            registered_at=time.time(),
            status=DeviceStatus.ACTIVE,
            consent_proof=consent_proof,
            risk_assessment=risk_assessment
        )
        
        # Step 5: Blockchain anchoring
        if self._blockchain_enabled:
            registration.blockchain_anchor = self._blockchain_anchor_device(registration)
        
        # Step 6: Store registration
        self._devices[identity.did] = registration
        
        # Step 7: Grant device rights
        rights_registry.grant_right(
            RightType.EXISTENCE,
            identity.did,
            'DEVICE',
            registered_by
        )
        rights_registry.grant_right(
            RightType.AUDITABILITY,
            identity.did,
            'DEVICE',
            registered_by
        )
        
        # Step 8: Audit logging
        self._audit_log.record(
            'DEVICE_REGISTERED',
            identity.did,
            owner_id,
            Decision.APPROVE,
            device_type=identity.device_type.value,
            risk_level=risk_assessment['risk_level'],
            blockchain_anchor=registration.blockchain_anchor
        )
        
        return identity.did
    
    def start_session(self, device_did: str, user_id: str) -> str:
        """Start a BCI/neuromorphic session"""
        
        # Verify device exists and is operational
        if device_did not in self._devices:
            raise ValueError(f"Device {device_did} not registered")
        
        device = self._devices[device_did]
        if not device.is_operational():
            raise ValueError(f"Device {device_did} is not operational (status: {device.status.value})")
        
        # Verify user consent
        consent_status = runtime.consent_manager.verify(user_id, 'BCI_SESSION', time.time())
        if not consent_status['is_active']:
            self._audit_log.record(
                'SESSION_START_DENIED',
                device_did,
                user_id,
                details={'reason': 'Consent not active'}
            )
            raise PermissionError("Active consent required for BCI session")
        
        # Check temporal constraints (minimum interval between sessions)
        last_session = self._get_last_session(user_id, device_did)
        if last_session and last_session.ended_at:
            time_since_last = time.time() - last_session.ended_at
            policy = runtime.get_policy(self._select_policy(device.risk_assessment['risk_level']))
            min_interval = policy.thresholds.get('min_session_interval', 0)
            
            if time_since_last < min_interval:
                self._audit_log.record(
                    'SESSION_START_DENIED',
                    device_did,
                    user_id,
                    details={
                        'reason': 'Minimum interval not met',
                        'time_since_last': time_since_last,
                        'required_interval': min_interval
                    }
                )
                raise ValueError(f"Must wait {min_interval - time_since_last:.0f} more seconds")
        
        # Create session
        session_id = hashlib.sha256(f"{device_did}:{user_id}:{time.time()}".encode()).hexdigest()
        
        session = BCISession(
            session_id=session_id,
            device_did=device_did,
            user_id=user_id,
            started_at=time.time(),
            status=SessionStatus.ACTIVE,
            consent_verified=True,
            monitoring_active=True
        )
        
        self._sessions[session_id] = session
        
        # Audit logging
        self._audit_log.record(
            'BCI_SESSION_STARTED',
            device_did,
            user_id,
            details={
                'session_id': session_id,
                'consent_proof': consent_status['proof']
            }
        )
        
        return session_id
    
    def update_session_metrics(self, session_id: str, cognitive_load: Optional[float] = None,
                              entropy_level: Optional[float] = None, data_transmitted: Optional[int] = None) -> None:
        """Update session monitoring metrics"""
        
        if session_id not in self._sessions:
            raise ValueError(f"Session {session_id} not found")
        
        session = self._sessions[session_id]
        
        if cognitive_load is not None:
            session.cognitive_load = cognitive_load
        
        if entropy_level is not None:
            session.entropy_level = entropy_level
        
        if data_transmitted is not None:
            session.data_transmitted = data_transmitted
        
        # Check for threshold violations
        device = self._devices[session.device_did]
        policy = runtime.get_policy(self._select_policy(device.risk_assessment['risk_level']))
        
        violations = []
        
        if cognitive_load and cognitive_load > policy.thresholds.get('max_cognitive_load', 1.0):
            violations.append('COGNITIVE_OVERLOAD')
        
        if entropy_level and entropy_level > policy.thresholds.get('max_entropy_level', 1.0):
            violations.append('ENTROPY_THRESHOLD_EXCEEDED')
        
        if violations:
            self._handle_session_violations(session, violations)
    
    def end_session(self, session_id: str, reason: str = "USER_REQUESTED") -> None:
        """End a BCI session"""
        
        if session_id not in self._sessions:
            raise ValueError(f"Session {session_id} not found")
        
        session = self._sessions[session_id]
        session.status = SessionStatus.TERMINATED
        session.ended_at = time.time()
        session.monitoring_active = False
        
        self._audit_log.record(
            'BCI_SESSION_ENDED',
            session.device_did,
            session.user_id,
            details={
                'session_id': session_id,
                'duration': session.duration(),
                'reason': reason,
                'cognitive_load': session.cognitive_load,
                'anomalies_detected': session.anomalies_detected
            }
        )
    
    def quarantine_device(self, device_did: str, reason: str, evidence: Dict[str, Any]) -> None:
        """Quarantine a device for safety/compliance reasons"""
        
        if device_did not in self._devices:
            raise ValueError(f"Device {device_did} not registered")
        
        device = self._devices[device_did]
        device.status = DeviceStatus.QUARANTINED
        
        # Terminate all active sessions
        active_sessions = [s for s in self._sessions.values() 
                          if s.device_did == device_did and s.status == SessionStatus.ACTIVE]
        
        for session in active_sessions:
            session.status = SessionStatus.QUARANTINED
            session.ended_at = time.time()
        
        # Record rights violation
        rights_registry.record_violation(
            RightType.CROSS_LAYER_BOUNDARIES,
            device_did,
            "SYSTEM",
            ViolationSeverity.HIGH,
            f"Device quarantined: {reason}",
            evidence
        )
        
        # Audit logging
        self._audit_log.record(
            'DEVICE_QUARANTINED',
            device_did,
            device.owner_id,
            details={
                'reason': reason,
                'evidence': evidence,
                'active_sessions_terminated': len(active_sessions)
            }
        )
    
    def _assess_device_risk(self, identity: DeviceIdentity) -> Dict[str, Any]:
        """Assess risk level of device"""
        risk_score = 0
        factors = []
        
        # Factor 1: Device type
        high_risk_types = {DeviceType.BCI_INVASIVE, DeviceType.NEURAL_IMPLANT, DeviceType.NEUROMORPHIC_CHIP}
        if identity.device_type in high_risk_types:
            risk_score += 40
            factors.append("High-risk device type")
        
        # Factor 2: Certification level
        if identity.certification_level == "EXPERIMENTAL":
            risk_score += 30
            factors.append("Experimental certification")
        elif identity.certification_level == "PENDING":
            risk_score += 20
            factors.append("Pending certification")
        
        # Factor 3: Compliance standards
        required_standards = {'HIPAA', 'GDPR', 'ISO_27001'}
        missing_standards = required_standards - set(identity.compliance_standards)
        if missing_standards:
            risk_score += len(missing_standards) * 10
            factors.append(f"Missing compliance: {', '.join(missing_standards)}")
        
        # Determine risk level
        if risk_score < 30:
            risk_level = "LOW"
        elif risk_score < 60:
            risk_level = "MODERATE"
        elif risk_score < 90:
            risk_level = "HIGH"
        else:
            risk_level = "EXTREME"
        
        return {
            'risk_score': risk_score,
            'risk_level': risk_level,
            'factors': factors
        }
    
    def _verify_compliance(self, identity: DeviceIdentity) -> bool:
        """Verify device meets compliance requirements"""
        required_standards = {'HIPAA', 'GDPR'}
        return required_standards.issubset(set(identity.compliance_standards))
    
    def _select_policy(self, risk_level: str) -> str:
        """Select appropriate policy based on risk level"""
        if risk_level in ['HIGH', 'EXTREME']:
            return "HighRiskDevicePolicy"
        return "NeuromorphicDevicePolicy"
    
    def _blockchain_anchor_device(self, registration: DeviceRegistration) -> str:
        """Anchor device registration to blockchain"""
        reg_hash = hashlib.sha256(str(registration.to_dict()).encode()).hexdigest()
        return f"blockchain:device:{reg_hash}"
    
    def _get_last_session(self, user_id: str, device_did: str) -> Optional[BCISession]:
        """Get user's last session with device"""
        user_sessions = [s for s in self._sessions.values() 
                        if s.user_id == user_id and s.device_did == device_did]
        
        if not user_sessions:
            return None
        
        return max(user_sessions, key=lambda s: s.started_at)
    
    def _handle_session_violations(self, session: BCISession, violations: List[str]) -> None:
        """Handle session policy violations"""
        session.anomalies_detected += len(violations)
        
        device = self._devices[session.device_did]
        policy = runtime.get_policy(self._select_policy(device.risk_assessment['risk_level']))
        
        # Log violations
        for violation in violations:
            self._audit_log.record(
                'SESSION_VIOLATION',
                session.device_did,
                session.user_id,
                details={
                    'session_id': session.session_id,
                    'violation': violation,
                    'cognitive_load': session.cognitive_load,
                    'entropy_level': session.entropy_level
                }
            )
        
        # Auto-quarantine if configured
        if policy.auto_action('quarantine', 'anomaly'):
            self.quarantine_device(
                session.device_did,
                f"Policy violations: {', '.join(violations)}",
                {
                    'session_id': session.session_id,
                    'violations': violations,
                    'cognitive_load': session.cognitive_load,
                    'entropy_level': session.entropy_level
                }
            )
    
    def get_device(self, device_did: str) -> Optional[DeviceRegistration]:
        """Get device registration"""
        return self._devices.get(device_did)
    
    def get_session(self, session_id: str) -> Optional[BCISession]:
        """Get session details"""
        return self._sessions.get(session_id)
    
    def get_user_devices(self, user_id: str) -> List[DeviceRegistration]:
        """Get all devices owned by user"""
        return [d for d in self._devices.values() if d.owner_id == user_id]
    
    def get_user_sessions(self, user_id: str) -> List[BCISession]:
        """Get all sessions for user"""
        return [s for s in self._sessions.values() if s.user_id == user_id]


# Global device registry
device_registry = DeviceRegistry()


# Convenience functions
def register_bci_device(manufacturer: str, model: str, serial_number: str,
                       device_type: DeviceType, owner_id: str, 
                       certification_level: str = "FDA_APPROVED",
                       compliance_standards: Optional[List[str]] = None) -> str:
    """Quick device registration"""
    
    if compliance_standards is None:
        compliance_standards = ['HIPAA', 'GDPR', 'ISO_27001']
    
    identity = DeviceIdentity(
        did="",  # Will be auto-generated
        device_type=device_type,
        manufacturer=manufacturer,
        model=model,
        serial_number=serial_number,
        firmware_version="1.0.0",
        certification_level=certification_level,
        compliance_standards=compliance_standards
    )
    
    return device_registry.register_device(identity, owner_id, "SYSTEM")
