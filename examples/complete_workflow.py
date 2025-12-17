"""
Example: Complete BCI Device Registration and Session Management
Demonstrates full Infra workflow with all 10 rights enforcement
"""

from core.aln_runtime import runtime, register_policy
from core.rights_framework import rights_registry, grant_augmented_user_rights, RightType, ViolationSeverity
from modules.augmentation.device_manager import (
    device_registry, DeviceIdentity, DeviceType, register_bci_device
)


def example_1_register_user_with_rights():
    """
    Example 1: Register a new augmented user with all 10 core rights
    """
    print("\n=== Example 1: User Rights Registration ===\n")
    
    user_id = "user_12345"
    
    # Grant all 10 core augmented-user rights
    grants = grant_augmented_user_rights(user_id, granted_by="SYSTEM")
    
    print(f"✅ Granted {len(grants)} rights to user {user_id}")
    for grant in grants:
        print(f"   - {grant.right_type.value}: {'ACTIVE' if grant.is_active() else 'INACTIVE'}")
    
    # Generate rights report
    report = rights_registry.generate_rights_report(user_id)
    print(f"\n📊 Rights Report:")
    print(f"   Total Rights: {report['total_rights_granted']}")
    print(f"   Active Rights: {report['active_rights']}")
    print(f"   Compliance Status: {report['compliance_status']}")
    
    return user_id


def example_2_register_bci_device(user_id):
    """
    Example 2: Register a BCI device with full compliance checks
    """
    print("\n=== Example 2: BCI Device Registration ===\n")
    
    # Create device identity
    identity = DeviceIdentity(
        did="",  # Will be auto-generated
        device_type=DeviceType.BCI_NON_INVASIVE,
        manufacturer="NeuraLink",
        model="NL-2025-Pro",
        serial_number="SN-98765-ABCD",
        firmware_version="2.5.1",
        certification_level="FDA_APPROVED",
        compliance_standards=["HIPAA", "GDPR", "ISO_27001", "ISO_13485"]
    )
    
    print(f"📱 Registering device:")
    print(f"   Type: {identity.device_type.value}")
    print(f"   Manufacturer: {identity.manufacturer}")
    print(f"   Model: {identity.model}")
    print(f"   Certification: {identity.certification_level}")
    
    # Register device (triggers full compliance workflow)
    try:
        device_did = device_registry.register_device(
            identity=identity,
            owner_id=user_id,
            registered_by="SYSTEM",
            consent_proof="consent_proof_hash_abc123"
        )
        
        print(f"\n✅ Device registered successfully!")
        print(f"   DID: {device_did}")
        
        # Get device details
        device = device_registry.get_device(device_did)
        print(f"   Status: {device.status.value}")
        print(f"   Risk Level: {device.risk_assessment['risk_level']}")
        print(f"   Blockchain Anchor: {device.blockchain_anchor}")
        
        return device_did
        
    except Exception as e:
        print(f"❌ Device registration failed: {e}")
        return None


def example_3_request_and_grant_consent(user_id):
    """
    Example 3: Request and grant user consent for BCI sessions
    """
    print("\n=== Example 3: Consent Management ===\n")
    
    # Request consent
    consent_id = runtime.consent_manager.request(
        user_id=user_id,
        purpose="BCI_SESSION",
        scope={
            "data_types": ["neural_activity", "cognitive_state"],
            "duration": "per_session",
            "third_party_sharing": False
        }
    )
    
    print(f"📝 Consent requested: {consent_id}")
    
    # User approves (in real system, this would be through UI)
    runtime.consent_manager.approve(consent_id, duration=31536000)  # 1 year
    
    print(f"✅ Consent approved")
    print(f"   Valid for: 1 year")
    print(f"   Purpose: BCI_SESSION")
    
    # Verify consent
    consent_status = runtime.consent_manager.verify(user_id, "BCI_SESSION", time.time())
    print(f"   Active: {consent_status['is_active']}")
    print(f"   Proof: {consent_status['proof'][:32]}...")
    
    return consent_id


def example_4_start_bci_session(user_id, device_did):
    """
    Example 4: Start a BCI session with real-time monitoring
    """
    print("\n=== Example 4: BCI Session Start ===\n")
    
    try:
        # Start session
        session_id = device_registry.start_session(device_did, user_id)
        
        print(f"✅ BCI session started")
        print(f"   Session ID: {session_id}")
        
        # Get session details
        session = device_registry.get_session(session_id)
        print(f"   Status: {session.status.value}")
        print(f"   Monitoring: {'ACTIVE' if session.monitoring_active else 'INACTIVE'}")
        print(f"   Consent Verified: {'YES' if session.consent_verified else 'NO'}")
        
        return session_id
        
    except Exception as e:
        print(f"❌ Session start failed: {e}")
        return None


def example_5_monitor_session(session_id):
    """
    Example 5: Real-time session monitoring with threshold checks
    """
    print("\n=== Example 5: Session Monitoring ===\n")
    
    if not session_id:
        print("❌ No active session")
        return
    
    import time
    import random
    
    print("📊 Monitoring session metrics...")
    
    for i in range(5):
        # Simulate real-time metrics
        cognitive_load = random.uniform(0.5, 0.95)
        entropy_level = random.uniform(0.4, 0.90)
        data_transmitted = random.randint(1000, 10000)
        
        # Update session metrics
        device_registry.update_session_metrics(
            session_id,
            cognitive_load=cognitive_load,
            entropy_level=entropy_level,
            data_transmitted=data_transmitted
        )
        
        # Display metrics
        print(f"\n   Interval {i+1}:")
        print(f"   Cognitive Load: {cognitive_load:.2f} {'⚠️ HIGH' if cognitive_load > 0.85 else '✓'}")
        print(f"   Entropy Level: {entropy_level:.2f} {'⚠️ HIGH' if entropy_level > 0.85 else '✓'}")
        print(f"   Data Transmitted: {data_transmitted} bytes")
        
        time.sleep(1)
    
    # Check if session still active
    session = device_registry.get_session(session_id)
    print(f"\n   Session Status: {session.status.value}")
    print(f"   Anomalies Detected: {session.anomalies_detected}")


def example_6_end_session(session_id):
    """
    Example 6: Properly end a BCI session
    """
    print("\n=== Example 6: Session Termination ===\n")
    
    if not session_id:
        print("❌ No active session")
        return
    
    # End session
    device_registry.end_session(session_id, reason="USER_REQUESTED")
    
    # Get final session summary
    session = device_registry.get_session(session_id)
    
    print(f"✅ Session ended")
    print(f"   Duration: {session.duration():.2f} seconds")
    print(f"   Final Cognitive Load: {session.cognitive_load:.2f}")
    print(f"   Total Data Transmitted: {session.data_transmitted} bytes")
    print(f"   Anomalies: {session.anomalies_detected}")


def example_7_rights_violation_handling():
    """
    Example 7: Detect and handle rights violations
    """
    print("\n=== Example 7: Rights Violation Handling ===\n")
    
    user_id = "user_test_violation"
    device_did = "did:infra:device:test_violation"
    
    # Simulate a privacy violation
    violation_id = rights_registry.record_violation(
        right_type=RightType.PRIVACY,
        entity_id=user_id,
        violated_by="rogue_service",
        severity=ViolationSeverity.HIGH,
        description="Unauthorized access to neural data",
        evidence={
            "access_timestamp": time.time(),
            "data_accessed": "neural_patterns",
            "accessor_ip": "192.168.1.100"
        }
    )
    
    print(f"⚠️ Rights violation recorded")
    print(f"   Violation ID: {violation_id}")
    print(f"   Right Violated: {RightType.PRIVACY.value}")
    print(f"   Severity: HIGH")
    print(f"   Auto-Escalated: YES")
    
    # Query violations
    violations = rights_registry.get_violations(entity_id=user_id)
    
    print(f"\n📋 Total Violations for {user_id}: {len(violations)}")
    for v in violations:
        print(f"   - {v.right_type.value}: {v.severity.value} ({v.remediation_status})")


def example_8_device_quarantine():
    """
    Example 8: Automatic device quarantine on anomaly
    """
    print("\n=== Example 8: Device Quarantine ===\n")
    
    user_id = "user_quarantine_test"
    
    # Register a device
    device_did = register_bci_device(
        manufacturer="TestCorp",
        model="Test-BCI-v1",
        serial_number="TEST-12345",
        device_type=DeviceType.BCI_NON_INVASIVE,
        owner_id=user_id,
        certification_level="EXPERIMENTAL",
        compliance_standards=["HIPAA", "GDPR"]
    )
    
    print(f"📱 Test device registered: {device_did[:32]}...")
    
    # Simulate critical anomaly
    device_registry.quarantine_device(
        device_did=device_did,
        reason="Critical entropy threshold exceeded",
        evidence={
            "entropy_level": 0.95,
            "threshold": 0.85,
            "cognitive_manipulation_detected": True
        }
    )
    
    # Verify quarantine
    device = device_registry.get_device(device_did)
    
    print(f"🔒 Device quarantined")
    print(f"   Status: {device.status.value}")
    print(f"   Reason: Critical entropy threshold exceeded")
    print(f"   All active sessions: TERMINATED")


def example_9_audit_trail_query():
    """
    Example 9: Query comprehensive audit trail
    """
    print("\n=== Example 9: Audit Trail Query ===\n")
    
    user_id = "user_12345"
    
    # Get audit trail for user
    audit_entries = runtime.get_audit_trail(user_id)
    
    print(f"📜 Audit Trail for {user_id}")
    print(f"   Total Entries: {len(audit_entries)}")
    
    if audit_entries:
        print(f"\n   Recent Events:")
        for entry in audit_entries[-5:]:  # Last 5 events
            print(f"   - {entry.event_type}: {entry.details.get('decision', 'N/A')}")


def example_10_complete_workflow():
    """
    Example 10: Complete end-to-end workflow
    """
    print("\n" + "="*60)
    print("         INFRA: COMPLETE AUGMENTED-USER WORKFLOW")
    print("="*60)
    
    import time
    
    # Step 1: Register user with rights
    user_id = example_1_register_user_with_rights()
    time.sleep(1)
    
    # Step 2: Request and grant consent
    consent_id = example_3_request_and_grant_consent(user_id)
    time.sleep(1)
    
    # Step 3: Register BCI device
    device_did = example_2_register_bci_device(user_id)
    if not device_did:
        print("\n❌ Workflow halted: Device registration failed")
        return
    time.sleep(1)
    
    # Step 4: Start BCI session
    session_id = example_4_start_bci_session(user_id, device_did)
    if not session_id:
        print("\n❌ Workflow halted: Session start failed")
        return
    time.sleep(1)
    
    # Step 5: Monitor session
    example_5_monitor_session(session_id)
    time.sleep(1)
    
    # Step 6: End session
    example_6_end_session(session_id)
    time.sleep(1)
    
    # Step 7: Query audit trail
    example_9_audit_trail_query()
    
    print("\n" + "="*60)
    print("         ✅ WORKFLOW COMPLETED SUCCESSFULLY")
    print("="*60)
    print("\n📊 Summary:")
    print(f"   - User registered with 10 core rights")
    print(f"   - Consent obtained and verified")
    print(f"   - BCI device registered with compliance checks")
    print(f"   - Session monitored with real-time metrics")
    print(f"   - Complete audit trail maintained")
    print(f"   - All actions blockchain-anchored")
    print("\n🌟 Infra: Trust-first, compliance-driven augmentation platform")


if __name__ == "__main__":
    # Run complete workflow
    example_10_complete_workflow()
    
    # Additional examples
    print("\n\n" + "="*60)
    print("         ADDITIONAL EXAMPLES")
    print("="*60)
    
    example_7_rights_violation_handling()
    example_8_device_quarantine()
