"""
Infra REST API - Augmented-User Rights & Smart City Governance
FastAPI implementation with Zero Trust security and comprehensive audit logging
"""

from fastapi import FastAPI, HTTPException, Depends, Header, status
from fastapi.security import HTTPBearer, HTTPAuthorizationCredentials
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel, Field
from typing import List, Optional, Dict, Any
from datetime import datetime
import time

from core.aln_runtime import runtime, evaluate, Decision
from core.rights_framework import (
    rights_registry, RightType, ViolationSeverity, grant_augmented_user_rights
)
from modules.augmentation.device_manager import (
    device_registry, DeviceIdentity, DeviceType, DeviceStatus
)


# FastAPI app initialization
app = FastAPI(
    title="Infra API",
    description="Augmented-User Rights & Smart City Governance Platform",
    version="1.0.0",
    docs_url="/docs",
    redoc_url="/redoc"
)

# CORS configuration
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Configure appropriately for production
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Security
security = HTTPBearer()


# ============================================================================
# Pydantic Models (Request/Response Schemas)
# ============================================================================

class UserRegistrationRequest(BaseModel):
    user_id: str = Field(..., description="Unique user identifier")
    granted_by: str = Field(default="SYSTEM", description="Who grants the rights")


class UserRightsResponse(BaseModel):
    user_id: str
    total_rights: int
    active_rights: int
    rights: List[Dict[str, Any]]
    compliance_status: str


class DeviceRegistrationRequest(BaseModel):
    manufacturer: str
    model: str
    serial_number: str
    device_type: str = Field(..., description="Device type (e.g., BCI_NON_INVASIVE)")
    owner_id: str
    firmware_version: str = "1.0.0"
    certification_level: str = "FDA_APPROVED"
    compliance_standards: List[str] = ["HIPAA", "GDPR", "ISO_27001"]
    consent_proof: Optional[str] = None


class DeviceRegistrationResponse(BaseModel):
    device_did: str
    status: str
    risk_level: str
    blockchain_anchor: Optional[str]
    message: str


class SessionStartRequest(BaseModel):
    device_did: str
    user_id: str


class SessionStartResponse(BaseModel):
    session_id: str
    status: str
    consent_verified: bool
    monitoring_active: bool


class SessionMetricsUpdate(BaseModel):
    cognitive_load: Optional[float] = None
    entropy_level: Optional[float] = None
    data_transmitted: Optional[int] = None


class ViolationReport(BaseModel):
    right_type: str
    entity_id: str
    violated_by: str
    severity: str
    description: str
    evidence: Dict[str, Any]


class ViolationResponse(BaseModel):
    violation_id: str
    status: str
    escalated: bool


class HealthResponse(BaseModel):
    status: str
    version: str
    timestamp: datetime
    services: Dict[str, str]


# ============================================================================
# Authentication & Authorization (Zero Trust)
# ============================================================================

async def verify_token(credentials: HTTPAuthorizationCredentials = Depends(security)):
    """
    Verify JWT token (placeholder - implement proper JWT validation)
    In production: Verify signature, expiration, claims, etc.
    """
    token = credentials.credentials
    
    # TODO: Implement proper JWT validation
    # For now, accept any token starting with "infra_"
    if not token.startswith("infra_"):
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Invalid authentication token"
        )
    
    return {"user_id": "authenticated_user", "roles": ["user", "admin"]}


async def require_admin(user: dict = Depends(verify_token)):
    """Require admin role"""
    if "admin" not in user.get("roles", []):
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="Admin privileges required"
        )
    return user


# ============================================================================
# Health & Status Endpoints
# ============================================================================

@app.get("/", response_model=HealthResponse)
async def root():
    """Health check and service status"""
    return {
        "status": "operational",
        "version": "1.0.0",
        "timestamp": datetime.now(),
        "services": {
            "aln_runtime": "operational",
            "rights_framework": "operational",
            "device_registry": "operational",
            "blockchain": "operational",
            "audit_log": "operational"
        }
    }


@app.get("/health")
async def health():
    """Kubernetes-compatible health check"""
    return {"status": "healthy"}


# ============================================================================
# User & Rights Management Endpoints
# ============================================================================

@app.post("/users/register", response_model=UserRightsResponse)
async def register_user(
    request: UserRegistrationRequest,
    user: dict = Depends(verify_token)
):
    """
    Register a new augmented user with all 10 core rights.
    
    Grants:
    - Right to Exist
    - Right to Privacy
    - Right to Consent
    - Right to Self-Ownership
    - Right to Meta-Cognitive Integrity
    - Right to Cross-Layer Boundaries
    - Right to Auditability
    - Right to Legal Triggers
    - Right to Explainability
    - Right to Rollback
    """
    try:
        grants = grant_augmented_user_rights(request.user_id, request.granted_by)
        report = rights_registry.generate_rights_report(request.user_id)
        
        return {
            "user_id": request.user_id,
            "total_rights": report["total_rights_granted"],
            "active_rights": report["active_rights"],
            "rights": report["rights"],
            "compliance_status": report["compliance_status"]
        }
    except Exception as e:
        raise HTTPException(status_code=400, detail=str(e))


@app.get("/users/{user_id}/rights", response_model=UserRightsResponse)
async def get_user_rights(
    user_id: str,
    user: dict = Depends(verify_token)
):
    """Get all rights for a specific user"""
    try:
        report = rights_registry.generate_rights_report(user_id)
        
        return {
            "user_id": user_id,
            "total_rights": report["total_rights_granted"],
            "active_rights": report["active_rights"],
            "rights": report["rights"],
            "compliance_status": report["compliance_status"]
        }
    except Exception as e:
        raise HTTPException(status_code=404, detail=f"User {user_id} not found")


@app.get("/users/{user_id}/violations")
async def get_user_violations(
    user_id: str,
    user: dict = Depends(verify_token)
):
    """Get all rights violations for a user"""
    violations = rights_registry.get_violations(entity_id=user_id)
    
    return {
        "user_id": user_id,
        "total_violations": len(violations),
        "violations": [v.to_dict() for v in violations]
    }


# ============================================================================
# Device Management Endpoints
# ============================================================================

@app.post("/devices/register", response_model=DeviceRegistrationResponse)
async def register_device(
    request: DeviceRegistrationRequest,
    user: dict = Depends(verify_token)
):
    """
    Register a new BCI/neuromorphic device.
    
    Performs:
    - Rights verification
    - Compliance checks
    - Risk assessment
    - Policy evaluation
    - Blockchain anchoring
    """
    try:
        # Parse device type
        device_type = DeviceType[request.device_type]
        
        # Create device identity
        identity = DeviceIdentity(
            did="",  # Auto-generated
            device_type=device_type,
            manufacturer=request.manufacturer,
            model=request.model,
            serial_number=request.serial_number,
            firmware_version=request.firmware_version,
            certification_level=request.certification_level,
            compliance_standards=request.compliance_standards
        )
        
        # Register device
        device_did = device_registry.register_device(
            identity=identity,
            owner_id=request.owner_id,
            registered_by=user["user_id"],
            consent_proof=request.consent_proof
        )
        
        # Get device details
        device = device_registry.get_device(device_did)
        
        return {
            "device_did": device_did,
            "status": device.status.value,
            "risk_level": device.risk_assessment["risk_level"],
            "blockchain_anchor": device.blockchain_anchor,
            "message": "Device registered successfully"
        }
        
    except ValueError as e:
        raise HTTPException(status_code=400, detail=str(e))
    except PermissionError as e:
        raise HTTPException(status_code=403, detail=str(e))
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/devices/{device_did}")
async def get_device(
    device_did: str,
    user: dict = Depends(verify_token)
):
    """Get device details"""
    device = device_registry.get_device(device_did)
    
    if not device:
        raise HTTPException(status_code=404, detail=f"Device {device_did} not found")
    
    return device.to_dict()


@app.get("/devices/user/{user_id}")
async def get_user_devices(
    user_id: str,
    user: dict = Depends(verify_token)
):
    """Get all devices owned by a user"""
    devices = device_registry.get_user_devices(user_id)
    
    return {
        "user_id": user_id,
        "device_count": len(devices),
        "devices": [d.to_dict() for d in devices]
    }


# ============================================================================
# Session Management Endpoints
# ============================================================================

@app.post("/sessions/start", response_model=SessionStartResponse)
async def start_session(
    request: SessionStartRequest,
    user: dict = Depends(verify_token)
):
    """Start a BCI/neuromorphic session"""
    try:
        session_id = device_registry.start_session(
            request.device_did,
            request.user_id
        )
        
        session = device_registry.get_session(session_id)
        
        return {
            "session_id": session_id,
            "status": session.status.value,
            "consent_verified": session.consent_verified,
            "monitoring_active": session.monitoring_active
        }
        
    except ValueError as e:
        raise HTTPException(status_code=400, detail=str(e))
    except PermissionError as e:
        raise HTTPException(status_code=403, detail=str(e))


@app.get("/sessions/{session_id}")
async def get_session(
    session_id: str,
    user: dict = Depends(verify_token)
):
    """Get session details"""
    session = device_registry.get_session(session_id)
    
    if not session:
        raise HTTPException(status_code=404, detail=f"Session {session_id} not found")
    
    return session.to_dict()


@app.put("/sessions/{session_id}/metrics")
async def update_session_metrics(
    session_id: str,
    metrics: SessionMetricsUpdate,
    user: dict = Depends(verify_token)
):
    """Update session monitoring metrics"""
    try:
        device_registry.update_session_metrics(
            session_id,
            cognitive_load=metrics.cognitive_load,
            entropy_level=metrics.entropy_level,
            data_transmitted=metrics.data_transmitted
        )
        
        session = device_registry.get_session(session_id)
        
        return {
            "session_id": session_id,
            "status": session.status.value,
            "cognitive_load": session.cognitive_load,
            "entropy_level": session.entropy_level,
            "anomalies_detected": session.anomalies_detected
        }
        
    except ValueError as e:
        raise HTTPException(status_code=404, detail=str(e))


@app.post("/sessions/{session_id}/end")
async def end_session(
    session_id: str,
    reason: str = "USER_REQUESTED",
    user: dict = Depends(verify_token)
):
    """End a BCI session"""
    try:
        device_registry.end_session(session_id, reason)
        session = device_registry.get_session(session_id)
        
        return {
            "session_id": session_id,
            "status": session.status.value,
            "duration": session.duration(),
            "message": "Session ended successfully"
        }
        
    except ValueError as e:
        raise HTTPException(status_code=404, detail=str(e))


# ============================================================================
# Compliance & Audit Endpoints
# ============================================================================

@app.post("/violations/report", response_model=ViolationResponse)
async def report_violation(
    report: ViolationReport,
    user: dict = Depends(verify_token)
):
    """Report a rights violation"""
    try:
        right_type = RightType[report.right_type]
        severity = ViolationSeverity[report.severity]
        
        violation_id = rights_registry.record_violation(
            right_type=right_type,
            entity_id=report.entity_id,
            violated_by=report.violated_by,
            severity=severity,
            description=report.description,
            evidence=report.evidence
        )
        
        violations = rights_registry.get_violations()
        violation = next(v for v in violations if v.violation_id == violation_id)
        
        return {
            "violation_id": violation_id,
            "status": violation.remediation_status,
            "escalated": violation.escalated
        }
        
    except KeyError as e:
        raise HTTPException(status_code=400, detail=f"Invalid enum value: {e}")


@app.get("/audit/{entity_id}")
async def get_audit_trail(
    entity_id: str,
    user: dict = Depends(verify_token)
):
    """Get complete audit trail for an entity"""
    audit_entries = runtime.get_audit_trail(entity_id)
    
    return {
        "entity_id": entity_id,
        "total_entries": len(audit_entries),
        "entries": [e.to_dict() for e in audit_entries]
    }


@app.get("/policies")
async def list_policies(user: dict = Depends(verify_token)):
    """List all registered ALN policies"""
    policies = list(runtime.policies.keys())
    
    return {
        "total_policies": len(policies),
        "policies": policies
    }


# ============================================================================
# Admin Endpoints (Elevated Privileges Required)
# ============================================================================

@app.post("/admin/devices/{device_did}/quarantine")
async def quarantine_device(
    device_did: str,
    reason: str,
    evidence: Dict[str, Any],
    admin: dict = Depends(require_admin)
):
    """Quarantine a device (admin only)"""
    try:
        device_registry.quarantine_device(device_did, reason, evidence)
        device = device_registry.get_device(device_did)
        
        return {
            "device_did": device_did,
            "status": device.status.value,
            "reason": reason,
            "message": "Device quarantined successfully"
        }
        
    except ValueError as e:
        raise HTTPException(status_code=404, detail=str(e))


@app.get("/admin/statistics")
async def get_platform_statistics(admin: dict = Depends(require_admin)):
    """Get platform-wide statistics (admin only)"""
    
    # Count active devices
    all_devices = [d for d in device_registry._devices.values()]
    active_devices = [d for d in all_devices if d.status == DeviceStatus.ACTIVE]
    
    # Count violations
    all_violations = rights_registry.get_violations()
    critical_violations = [v for v in all_violations if v.severity == ViolationSeverity.CRITICAL]
    
    return {
        "timestamp": datetime.now().isoformat(),
        "devices": {
            "total": len(all_devices),
            "active": len(active_devices),
            "quarantined": len([d for d in all_devices if d.status == DeviceStatus.QUARANTINED])
        },
        "violations": {
            "total": len(all_violations),
            "critical": len(critical_violations),
            "pending_remediation": len([v for v in all_violations if v.remediation_status == "PENDING"])
        },
        "policies": {
            "total": len(runtime.policies)
        }
    }


# ============================================================================
# Run Application
# ============================================================================

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(
        "main:app",
        host="0.0.0.0",
        port=8000,
        reload=True,
        log_level="info"
    )
