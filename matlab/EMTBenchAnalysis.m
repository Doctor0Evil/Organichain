% matlab/EMTBenchAnalysis.m
% Analyze EMTBenchSample vs EMTEnvelope safety envelopes

function results = EMTBenchAnalysis(benchCsv, envelopeCsv)
  bench = readtable(benchCsv);
  env   = readtable(envelopeCsv);

  merged = innerjoin(bench, env, 'Keys', 'envelopeid');

  % Temperature rise check
  merged.delta_temp = merged.measured_temp_c - merged.baseline_temp_c;
  merged.temp_ok = merged.delta_temp <= merged.limit_temp_delta_c + 0.05;

  % Risk index recomputation
  merged.thermal_risk = min(merged.energy_density_mjmm3 ./ merged.limit_energy_mjmm3, 1.0);
  merged.mech_risk    = min(merged.duty_cycle_pct ./ merged.limit_duty_pct, 1.0);
  merged.neuro_risk   = min(merged.actuation_freq_hz ./ merged.limit_freq_hz, 1.0);

  results = merged;
end
