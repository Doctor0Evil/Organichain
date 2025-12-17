-- lua/emt_stimulus_bt.lua
-- Behavior tree for safe stimulus patterns on phantom tissue

local bt = {}

bt.state = {
  session_ms = 0,
  max_session_ms = 600000, -- 10 min
  last_pattern_ms = 0
}

function bt.update(now_ms, current_risk)
  bt.state.session_ms = now_ms

  if now_ms >= bt.state.max_session_ms then
    return { action = "STOP", reason = "session_timeout" }
  end

  if current_risk >= 0.9 then
    return { action = "STOP", reason = "high_risk" }
  elseif current_risk >= 0.7 then
    return { action = "DERATE", level = 0.5 }
  elseif current_risk >= 0.5 then
    return { action = "DERATE", level = 0.8 }
  else
    return { action = "NORMAL", level = 1.0 }
  end
end

return bt
