% Deterministic IEC 62304 / ISO 14971‑aligned safety envelope checker[web:3][web:8]
% Returns logical mask of allowed stimuli for batch neuromodulation studies.

function allowMask = neuromod_safety_envelope(amp_mt, freq_hz, pw_ms, duty)
    amp_max  = 10.0;
    f_min    = 1.0;
    f_max    = 150.0;
    pw_min   = 0.5;
    pw_max   = 10.0;
    duty_max = 0.25;

    allowMask = amp_mt > 0.0 & amp_mt <= amp_max & ...
                freq_hz >= f_min & freq_hz <= f_max & ...
                pw_ms >= pw_min & pw_ms <= pw_max & ...
                duty >= 0.0 & duty <= duty_max;
end
