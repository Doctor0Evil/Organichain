package bioaug

import java.sql.Connection
import java.sql.DriverManager
import java.util.UUID

data class NanoNode(
    val nodeId: UUID,
    val subjectId: UUID,
    val opsThresholdTops: Double
)

class NanoSwarmPolicyClient(
    jdbcUrl: String,
    user: String,
    password: String
) {
    private val conn: Connection =
        DriverManager.getConnection(jdbcUrl, user, password)

    fun getNode(nodeId: UUID): NanoNode {
        val sql = """
            SELECT node_id, subject_id, ops_threshold_tops
            FROM nano_node
            WHERE node_id = ?
        """.trimIndent()
        conn.prepareStatement(sql).use { ps ->
            ps.setObject(1, nodeId)
            ps.executeQuery().use { rs ->
                if (!rs.next()) error("Nano node not found: $nodeId")
                return NanoNode(
                    nodeId = rs.getObject("node_id", UUID::class.java),
                    subjectId = rs.getObject("subject_id", UUID::class.java),
                    opsThresholdTops = rs.getDouble("ops_threshold_tops")
                )
            }
        }
    }

    fun recordSafeOff(featureId: Long, subjectId: UUID) {
        val sql = """
            INSERT INTO security_snapshot (
                feature_id, subject_id, timestamp_ms,
                security_state, threat_flag,
                pqc_enabled, hw_root_of_trust, secure_boot, isolation_enforced,
                anomaly_score_01, attack_surface_score, last_update_delta_ms
            ) VALUES (
                ?, ?, EXTRACT(EPOCH FROM NOW())::BIGINT * 1000,
                'LOCKED_DOWN', 'SUSPECTED_MALWARE',
                TRUE, TRUE, TRUE, TRUE,
                0.90, 0.80, 0
            )
        """.trimIndent()
        conn.prepareStatement(sql).use { ps ->
            ps.setLong(1, featureId)
            ps.setObject(2, subjectId)
            ps.executeUpdate()
        }
    }
}
