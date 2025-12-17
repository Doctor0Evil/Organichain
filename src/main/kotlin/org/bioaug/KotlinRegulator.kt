// path: src/main/kotlin/org/bioaug/KotlinRegulator.kt

package org.bioaug

import kotlinx.serialization.Serializable
import java.math.BigInteger
import java.security.MessageDigest
import java.time.Instant

@Serializable
data class RiskSampleDto(
    val risk_score: Double,
    val ed_percent: Double,
    val sf_psych: Double,
    val h_mod: Long
)

@Serializable
data class RegulatorInputDto(
    val user_id: String,
    val bio_key_hex: String,
    val depth: Double,
    val energy_scalar: Double,
    val t: Long,
    val auet: String,
    val csp: String
)

object KotlinRegulator {

    private const val W_DEPTH = 0.18
    private const val W_ENERGY = 0.27
    private const val W_AUET = 0.22
    private const val W_CSP = 0.33
    private const val K_PSYCH = 1.35
    private const val AUET_FLOOR = 1.0e12
    private const val CSP_FLOOR = 1.0e12

    private fun normalizeNonZero(v: Double, max: Double): Double {
        return if (max <= 0.0) 0.0 else (v / max).coerceIn(0.0, 1.0)
    }

    private fun hexToBytes(hex: String): ByteArray {
        require(hex.length % 2 == 0) { "bio_key_hex must have even length" }
        val out = ByteArray(hex.length / 2)
        var i = 0
        while (i < hex.length) {
            val byte = hex.substring(i, i + 2).toInt(16).toByte()
            out[i / 2] = byte
            i += 2
        }
        return out
    }

    private fun computeHashMod(userId: String, bioKey: ByteArray, t: Long): Long {
        val md = MessageDigest.getInstance("SHA3-512")
        md.update(userId.toByteArray(Charsets.UTF_8))
        md.update(bioKey)
        md.update(t.toString().toByteArray(Charsets.UTF_8))
        val digest = md.digest()
        var acc = 0L
        for (i in 0 until 8) {
            acc = acc or ((digest[i].toLong() and 0xFFL) shl (8 * i))
        }
        return acc
    }

    fun evaluate(input: RegulatorInputDto): RiskSampleDto {
        val bioKey = hexToBytes(input.bio_key_hex)

        val depthN = normalizeNonZero(input.depth, 10.0)
        val energyN = normalizeNonZero(input.energy_scalar, 100.0)

        val auetVal = BigInteger(input.auet)
        val cspVal = BigInteger(input.csp)

        val auetN = if (auetVal == BigInteger.ZERO) {
            1.0
        } else {
            val v = (AUET_FLOOR / auetVal.toDouble()).coerceAtMost(5.0)
            (v / 5.0).coerceIn(0.0, 1.0)
        }

        val cspN = if (cspVal == BigInteger.ZERO) {
            1.0
        } else {
            val v = (CSP_FLOOR / cspVal.toDouble()).coerceAtMost(5.0)
            (v / 5.0).coerceIn(0.0, 1.0)
        }

        val raw =
            W_DEPTH * depthN +
            W_ENERGY * energyN +
            W_AUET * auetN +
            W_CSP * cspN

        val risk = raw.coerceIn(0.0, 1.0)
        val ed = (risk * 100.0).coerceIn(0.0, 100.0)
        val sfPsych = K_PSYCH * (0.6 * depthN + 0.4 * cspN)
        val hMod = computeHashMod(input.user_id, bioKey, input.t)

        return RiskSampleDto(
            risk_score = risk,
            ed_percent = ed,
            sf_psych = sfPsych,
            h_mod = hMod
        )
    }
}

fun main() {
    val now = Instant.now().epochSecond
    val input = RegulatorInputDto(
        user_id = "auguser-001",
        bio_key_hex = "A1B2C3D4E5F60718293A4B5C6D7E8F90112233445566778899AABBCCDDEEFF00",
        depth = 4.2,
        energy_scalar = 37.5,
        t = now,
        auet = "1500000000000",
        csp = "2300000000000"
    )
    val sample = KotlinRegulator.evaluate(input)
    println("RiskSampleDto: $sample")
}
