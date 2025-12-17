% matlab/bioaug_integration_spine.m
% Offline/CI validation of Bio-Aug + MT6883 + nanoswarm invariants. [web:15][web:21]

%% 0. GLOBAL INVARIANTS

function ok = inv_NO_NET_TO_ACTUATORS(auditEvents, policy_enforced)
% Invariant: if any event has code 0x0101 (net-path-evaluated), then
% net.nointernettoactuators must be enforced. [web:15]

    if isempty(auditEvents)
        ok = true;
        return;
    end

    codes = double([auditEvents.eventcode]);              % normalize type. [web:17]
    mask  = (codes == hex2dec('0101'));
    if any(mask)
        ok = logical(policy_enforced);
    else
        ok = true;
    end
end

function ok = inv_SAFETY_ENVELOPES(bciLinks, biomechFeatures, envelopeFn)
% Enforces Digest safety envelope for each BCI link. [web:18]

    if isempty(bciLinks)
        ok = true;
        return;
    end

    ok = true;
    for k = 1:numel(bciLinks)
        fid  = bciLinks(k).featureid;
        idx  = find([biomechFeatures.featureid] == fid, 1);
        if isempty(idx)
            ok = false;
            return;
        end
        region = biomechFeatures(idx).bodyregioncode;
        maxD   = bciLinks(k).maxactuationdelta;

        if ~isscalar(maxD) || ~isfinite(maxD)          % robustness for CI. [web:20]
            ok = false; return;
        end

        if ~envelopeFn(region, maxD)
            ok = false;
            return;
        end
    end
end

%% 1. NODE CLASSIFICATION

function node = makeIntegrationNode(node_id, ops_tops, rows, cols, compliance, fw_ver, node_class)
% Typed construction for IntegrationNode records. [web:17]

    node.node_id             = uint64(node_id);
    node.ops_threshold_tops  = single(ops_tops);
    node.topology_rows       = uint16(rows);
    node.topology_cols       = uint16(cols);
    node.compliance_level    = uint8(compliance);
    node.ai_firmware_version = uint32(fw_ver);
    node.node_class          = uint8(node_class);
end

%% 2. BIO-AUG ↔ MT6883 BINDINGS

function ok = policy_bind_bioaug_to_mt6883(binding, biomechFeatures, nodeLookupFn, ...
                                           checkProfileFn, checkRightsFn)
% Enforces throughput, clinical profile, and rights constraints for bindings. [web:18]

    ok = true;

    node = nodeLookupFn(binding.node_id);
    if ~isscalar(node.ops_threshold_tops) || ~isfinite(node.ops_threshold_tops) ...
            || double(node.ops_threshold_tops) < 12.0                       % numeric guard. [web:20]
        ok = false;
        return;
    end

    idx = find([biomechFeatures.featureid] == binding.featureid, 1);
    if isempty(idx)
        ok = false;
        return;
    end
    safetytier = biomechFeatures(idx).safetytier;

    if ~strcmp(checkProfileFn(binding.clinical_profilehash, safetytier), "OK")
        ok = false;
        return;
    end
    if ~strcmp(checkRightsFn(binding.rights_profilehash, binding.useridhash), "OK")
        ok = false;
        return;
    end
end

%% 3. NANO / NEUROMORPHIC CONSTRAINTS

function ok = policy_nano_envelope_boundary(envelope, modulationSafetyFn, damageThreshold)
% Enforces neuromorphic stimulation safety bounds. [web:18]

    ok = true;

    if ~isfinite(envelope.energyintegral)                  % guard NaN/Inf. [web:20]
        ok = false; return;
    end

    if ~strcmp(modulationSafetyFn(envelope.stim_type, ...
                                  envelope.resonancefreqhz, ...
                                  envelope.amplitudemt), "OK")
        ok = false;
        return;
    end

    if double(envelope.energyintegral) > double(damageThreshold)
        ok = false;
        return;
    end
end

%% 4. CLINICAL MODE SELECTION

function ok = policy_mode_gates(session, enforceClassCFn, simulationOnlyFlag, rightsRecord)
% Gating between ClinicProcedure, DailyAugmented, and ResearchSandbox. [web:18]

    ClinicProcedure = uint8(1);
    ResearchSandbox = uint8(3);

    ok = true;

    if session.mode == ClinicProcedure
        if session.digest_pipeline_enabled ~= uint8(1) ...
                || session.nausea_orchestrator_enabled ~= uint8(1)
            ok = false;
            return;
        end
        if ~strcmp(enforceClassCFn(session.node_id), "PASS")
            ok = false;
            return;
        end
    end

    if session.mode == ResearchSandbox
        if ~logical(simulationOnlyFlag)
            ok = false;
            return;
        end
        if ~isfield(rightsRecord, 'RightSelfOwnership') || ~rightsRecord.RightSelfOwnership
            ok = false;
            return;
        end
    end
end

%% 5. SQL-SCHEMA-COMPATIBLE CHECKS

function ok = validate_integration_node_sql(node)
% Mirrors integration_node table constraints with double(...) guards. [web:18]

    ok = true;

    if double(node.ops_threshold_tops) < 0
        ok = false; return;
    end
    if double(node.topology_rows) <= 0 || double(node.topology_cols) <= 0
        ok = false; return;
    end
    if double(node.compliance_level) < 0 || double(node.compliance_level) > 2
        ok = false; return;
    end
    if double(node.node_class) < 1 || double(node.node_class) > 5
        ok = false; return;
    end
end

function ok = validate_nano_bci_envelope_sql(env)
% Mirrors nano_bci_envelope table constraints with explicit numeric casting. [web:18]

    ok = true;

    if double(env.stim_type) < 1 || double(env.stim_type) > 5
        ok = false; return;
    end
    if double(env.resonancefreqhz) < 1 || double(env.resonancefreqhz) > 5000
        ok = false; return;
    end
    if double(env.amplitudemt) > 10
        ok = false; return;
    end
    if double(env.dutycycle) < 0 || double(env.dutycycle) > 1
        ok = false; return;
    end
end

%% 6. TRACEABILITY MATRIX ANCHOR

function anchor = matrix_hex_anchor()
% Returns MAT-compatible traceability anchor as uint8 vector. [web:22]

    hexStr = '42494F415547434C494E4943414C2D434C415353432D525553542D5741534D2D414C4E2D4353454D534146455F5441494C4F5245445F54524143454142494C4954595F4D415452495845532D5A45524F54525553545F5A45524F5452555354';
    anchor = uint8(sscanf(hexStr, '%2x').');
end

%% 7. CI HARNESS

function report = run_bioaug_integration_checks(ctx)
% Single entry point for CI/offline validation across all invariants. [web:21]

    report.inv.NO_NET_TO_ACTUATORS   = inv_NO_NET_TO_ACTUATORS(ctx.auditEvents, ctx.policy_enforced);
    report.inv.SAFETY_ENVELOPES      = inv_SAFETY_ENVELOPES(ctx.bciLinks, ctx.biomechFeatures, ctx.envelopeFn);

    report.policy.bind_bioaug_mt6883 = policy_bind_bioaug_to_mt6883( ...
                                          ctx.binding, ctx.biomechFeatures, ...
                                          ctx.nodeLookupFn, ctx.checkProfileFn, ctx.checkRightsFn);

    report.policy.nano_envelope      = policy_nano_envelope_boundary( ...
                                          ctx.envelope, ctx.modulationSafetyFn, ctx.damageThreshold);

    report.policy.mode_gates         = policy_mode_gates( ...
                                          ctx.session, ctx.enforceClassCFn, ...
                                          ctx.simulationOnlyFlag, ctx.rightsRecord);

    report.sql.node                  = validate_integration_node_sql(ctx.node);
    report.sql.nano_env              = validate_nano_bci_envelope_sql(ctx.envelope);

    report.trace.anchor              = matrix_hex_anchor();
end
