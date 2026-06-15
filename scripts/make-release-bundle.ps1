param(
    [string] $Manifest = "build/release/full_check_manifest.json",
    [string] $DeterministicEvidence = "build/release/deterministic_evidence_hashes.json",
    [string] $ReleaseClaimTable = "build/release/release_claim_table.json",
    [string] $LlmEvidence = "build/release/llm_evidence_hashes.json",
    [string] $OutputDir = "build/release",
    [string] $BundleName = "mico-release-candidate",
    [switch] $AllowDirty
)

$ErrorActionPreference = "Stop"

$repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path

function Resolve-RepoPath {
    param([string] $Path)
    if ([System.IO.Path]::IsPathRooted($Path)) {
        return $Path
    }
    return (Join-Path $repoRoot $Path)
}

function Convert-ToBundlePath {
    param([string] $Path)
    $resolved = (Resolve-Path -LiteralPath $Path).Path
    $relative = $resolved.Substring($script:stageRoot.Length).TrimStart("\", "/")
    return ($relative -replace "\\", "/")
}

function Get-Sha256 {
    param([string] $Path)
    return (Get-FileHash -Algorithm SHA256 -LiteralPath $Path).Hash.ToLowerInvariant()
}

function Copy-BundleFile {
    param(
        [string] $Source,
        [string] $Destination
    )
    $sourcePath = Resolve-RepoPath $Source
    if (-not (Test-Path -LiteralPath $sourcePath -PathType Leaf)) {
        throw "Required artifact is missing: $Source"
    }

    $destinationPath = Join-Path $script:stageRoot $Destination
    $destinationDir = Split-Path -Parent $destinationPath
    New-Item -ItemType Directory -Force -Path $destinationDir | Out-Null
    Copy-Item -LiteralPath $sourcePath -Destination $destinationPath -Force
}

function Copy-BundleTree {
    param(
        [string] $SourceDir,
        [string] $DestinationDir
    )
    $sourcePath = Resolve-RepoPath $SourceDir
    if (-not (Test-Path -LiteralPath $sourcePath -PathType Container)) {
        throw "Required artifact directory is missing: $SourceDir"
    }

    Get-ChildItem -LiteralPath $sourcePath -Recurse -File | ForEach-Object {
        $relative = $_.FullName.Substring($sourcePath.Length).TrimStart("\", "/")
        Copy-BundleFile $_.FullName (Join-Path $DestinationDir $relative)
    }
}

function Assert-PathInside {
    param(
        [string] $Child,
        [string] $Parent
    )
    $childFull = [System.IO.Path]::GetFullPath($Child)
    $parentFull = [System.IO.Path]::GetFullPath($Parent).TrimEnd("\", "/") + [System.IO.Path]::DirectorySeparatorChar
    if (-not $childFull.StartsWith($parentFull, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Refusing to operate outside expected directory: $childFull"
    }
}

Push-Location $repoRoot
try {
    $status = (& git status --short) -join "`n"
    if (-not $AllowDirty -and $status.Trim().Length -gt 0) {
        throw "Working tree must be clean before creating a release bundle."
    }

    $forbiddenTracked = (& git ls-files "config/*.local.yaml" "build" "target" "rust_project/target" "logs" "reports" "paper/*.pdf" "*.log" "*.jou" "*.wdb" "*.xpr") | Where-Object { $_ }
    if ($forbiddenTracked) {
        throw "Forbidden generated or local files are tracked: $($forbiddenTracked -join ', ')"
    }

    $manifestPath = Resolve-RepoPath $Manifest
    if (-not (Test-Path -LiteralPath $manifestPath -PathType Leaf)) {
        throw "Full-check manifest is missing: $Manifest"
    }

    $paperPdfPath = Resolve-RepoPath "paper/main.pdf"
    if (-not (Test-Path -LiteralPath $paperPdfPath -PathType Leaf)) {
        throw "Paper PDF is missing; run .\scripts\full-check.ps1 -WithLatex first."
    }

    $commit = (& git rev-parse HEAD).Trim()
    $shortCommit = (& git rev-parse --short HEAD).Trim()
    $branch = (& git branch --show-current).Trim()
    $worktreeStatus = @()
    if ($status.Trim().Length -gt 0) {
        $worktreeStatus = @($status -split "`n")
    }
    $outputRoot = Resolve-RepoPath $OutputDir
    New-Item -ItemType Directory -Force -Path $outputRoot | Out-Null
    $outputRoot = (Resolve-Path -LiteralPath $outputRoot).Path

    $script:stageRoot = Join-Path $outputRoot "$BundleName-$shortCommit-staging"
    Assert-PathInside $script:stageRoot $outputRoot
    if (Test-Path -LiteralPath $script:stageRoot) {
        $resolvedStage = (Resolve-Path -LiteralPath $script:stageRoot).Path
        Assert-PathInside $resolvedStage $outputRoot
        Remove-Item -LiteralPath $resolvedStage -Recurse -Force
    }
    New-Item -ItemType Directory -Force -Path $script:stageRoot | Out-Null

    $sourceZip = Join-Path $script:stageRoot "source\mico-source-$shortCommit.zip"
    New-Item -ItemType Directory -Force -Path (Split-Path -Parent $sourceZip) | Out-Null
    & git archive --format=zip "--output=$sourceZip" HEAD
    if ($LASTEXITCODE -ne 0) {
        throw "git archive failed."
    }

    Copy-BundleTree "schemas" "schemas"
    Copy-BundleTree "prompts" "prompts"
    Copy-BundleFile "README.md" "README.md"
    Copy-BundleFile "RELEASE_CHECKLIST.md" "RELEASE_CHECKLIST.md"
    Copy-BundleFile "docs/artifact_quickstart.md" "ARTIFACT_README.md"
    Copy-BundleFile "config/llm-provider.example.yaml" "config/llm-provider.example.yaml"
    Copy-BundleFile "benchmarks/module_compose_bench_manifest.yaml" "manifests/module_compose_bench_manifest.yaml"
    Copy-BundleFile "benchmarks/module_compose_bench_heldout.yaml" "manifests/module_compose_bench_heldout.yaml"
    Copy-BundleFile "benchmarks/module_compose_bench_realism.yaml" "manifests/module_compose_bench_realism.yaml"
    $docFiles = @(
        "docs/14_reproduction_workflow.md",
        "docs/15_case_studies.md",
        "docs/16_llm_matrix_results.md",
        "docs/17_llm_prompt_redesign_pilot.md",
        "docs/18_directed_verification_hardening.md",
        "docs/19_vivado_qor_subset.md",
        "docs/qor_vivado_subset_report.md",
        "docs/20_paper_dac_ready.md",
        "docs/21_artifact_release_candidate.md",
        "docs/22_llm_full_matrix_v2.md",
        "docs/23_heldout_benchmark_hardening.md",
        "docs/24_llm_matrix_v3.md",
        "docs/25_realism_supplement.md",
        "docs/llm_final_matrix_report.md",
        "docs/artifact_quickstart.md",
        "docs/claim_boundary.md",
        "docs/current_status.md",
        "docs/dac2027_full_check_baseline_2026-06-15.md",
        "docs/dac2027_submission_plan.md"
    )
    foreach ($doc in $docFiles) {
        Copy-BundleFile $doc $doc
    }
    Copy-BundleFile $Manifest "release/full_check_manifest.json"
    Copy-BundleFile $DeterministicEvidence "release/deterministic_evidence_hashes.json"
    Copy-BundleFile $ReleaseClaimTable "release/release_claim_table.json"
    Copy-BundleFile $LlmEvidence "release/llm_evidence_hashes.json"
    Copy-BundleFile "build/bench/seed_results.json" "results/deterministic/seed_results.json"
    Copy-BundleFile "build/bench/heldout_results.json" "results/deterministic/heldout_results.json"
    Copy-BundleFile "build/bench/realism_results.json" "results/deterministic/realism_results.json"
    Copy-BundleFile "build/bench/aggregate_results.json" "results/deterministic/aggregate_results.json"
    Copy-BundleFile "build/bench/aggregate_heldout_results.json" "results/deterministic/aggregate_heldout_results.json"
    Copy-BundleFile "build/bench/aggregate_realism_results.json" "results/deterministic/aggregate_realism_results.json"
    Copy-BundleFile "build/llm/provider_validate.json" "results/llm_validate/provider_validate.json"
    Copy-BundleFile "build/llm/bench_validate.json" "results/llm_validate/bench_validate.json"
    $sanitizedLlmExecuteFiles = @(
        "build/llm/bench_execute_public_dev_v3.json",
        "build/llm/bench_execute_heldout_v3.json"
    )
    foreach ($file in $sanitizedLlmExecuteFiles) {
        if (Test-Path -LiteralPath (Resolve-RepoPath $file) -PathType Leaf) {
            Copy-BundleFile $file ("results/llm_execute/" + (Split-Path -Leaf $file))
        }
    }
    Copy-BundleFile "paper/main.pdf" "paper/main.pdf"
    if (Test-Path -LiteralPath (Resolve-RepoPath "paper/tables") -PathType Container) {
        Copy-BundleTree "paper/tables" "paper/tables"
    }
    $optionalAggregateFiles = @(
        "build/bench/aggregate_llm_v3.json",
        "build/bench/aggregate_dac2027_llm_stats.json",
        "build/bench/aggregate_m3_heldout_directed.json"
    )
    foreach ($file in $optionalAggregateFiles) {
        if (Test-Path -LiteralPath (Resolve-RepoPath $file) -PathType Leaf) {
            Copy-BundleFile $file ("results/aggregate/" + (Split-Path -Leaf $file))
        }
    }

    $vivadoFiles = @(
        "build/reports/vivado-host/vivado_qor_subset_summary.json",
        "build/reports/vivado-host/vivado_qor_subset_summary.csv",
        "build/reports/vivado-host/vivado_qor_subset_delta.csv",
        "build/reports/vivado-host/vivado_qor_subset_summary.tex",
        "build/reports/vivado-host/vivado_qor_thresholds.json",
        "build/reports/vivado-host/vivado_qor_thresholds.tex"
    )
    foreach ($file in $vivadoFiles) {
        if (Test-Path -LiteralPath (Resolve-RepoPath $file) -PathType Leaf) {
            Copy-BundleFile $file ("results/vivado/" + (Split-Path -Leaf $file))
        }
    }

    $tableFiles = @(
        "build/bench/deterministic_summary.csv",
        "build/bench/deterministic_per_level.csv",
        "build/bench/unsafe_diagnostics.csv",
        "build/bench/ablation_counterfactual.csv",
        "build/bench/qor_summary.csv",
        "build/bench/qor_summary.tex",
        "build/bench/qor_structural.csv",
        "build/bench/llm_summary.csv",
        "build/bench/llm_compact_summary.csv",
        "build/bench/llm_failure_taxonomy.csv",
        "build/bench/llm_paired_comparisons.csv",
        "build/bench/llm_cost_tokens.csv",
        "build/bench/llm_repair_turns.csv"
    )
    foreach ($file in $tableFiles) {
        if (Test-Path -LiteralPath (Resolve-RepoPath $file) -PathType Leaf) {
            Copy-BundleFile $file ("tables/" + (Split-Path -Leaf $file))
        }
    }
    if (Test-Path -LiteralPath (Resolve-RepoPath "build/bench/heldout_tables") -PathType Container) {
        Copy-BundleTree "build/bench/heldout_tables" "tables/heldout"
    }
    if (Test-Path -LiteralPath (Resolve-RepoPath "build/bench/realism_tables") -PathType Container) {
        Copy-BundleTree "build/bench/realism_tables" "tables/realism"
    }
    if (Test-Path -LiteralPath (Resolve-RepoPath "build/bench/dac2027_llm_stats") -PathType Container) {
        Copy-BundleTree "build/bench/dac2027_llm_stats" "tables/dac2027_llm_stats"
    }

    $forbiddenBundlePaths = Get-ChildItem -LiteralPath $script:stageRoot -Recurse -File | Where-Object {
        $bundlePath = Convert-ToBundlePath $_.FullName
        $bundlePath -match "(^|/)config/.*\.local\.ya?ml$" -or
        $bundlePath -match "(^|/)build/" -or
        $bundlePath -match "(^|/)target/" -or
        $bundlePath -match "\.log$" -or
        $bundlePath -match "\.jou$" -or
        $bundlePath -match "\.wdb$" -or
        $bundlePath -match "\.xpr$" -or
        ($bundlePath -match "bench_execute.*\.json$" -and
            $bundlePath -notin @(
                "results/llm_execute/bench_execute_public_dev_v3.json",
                "results/llm_execute/bench_execute_heldout_v3.json"
            ))
    }
    if ($forbiddenBundlePaths) {
        throw "Forbidden paths were staged for the bundle: $($forbiddenBundlePaths.FullName -join ', ')"
    }

    $secretPattern = "(?i)(sk-[a-z0-9_-]{20,}|api[_-]?key\s*[:=]\s*['""]?(?!present|redacted|source|env|\$|opencode_go_api_key)[a-z0-9_./+-]{16,})"
    Get-ChildItem -LiteralPath $script:stageRoot -Recurse -File | Where-Object {
        $_.Length -lt 10485760 -and $_.Extension -ne ".zip" -and $_.Extension -ne ".pdf"
    } | ForEach-Object {
        $content = Get-Content -LiteralPath $_.FullName -Raw -ErrorAction SilentlyContinue
        if ($content -match $secretPattern) {
            throw "Potential secret-like content found in bundle file: $(Convert-ToBundlePath $_.FullName)"
        }
    }

    $files = Get-ChildItem -LiteralPath $script:stageRoot -Recurse -File | Sort-Object FullName | ForEach-Object {
        [ordered]@{
            path = Convert-ToBundlePath $_.FullName
            sha256 = Get-Sha256 $_.FullName
        }
    }

    $zipLeaf = "$BundleName-$shortCommit.zip"
    $shaLeaf = "$zipLeaf.sha256"

    $artifactManifest = [ordered]@{
        schema_version = "mico.release.bundle.v0"
        generated_at_utc = (Get-Date).ToUniversalTime().ToString("o")
        source_commit_hash = $commit
        source_branch = $branch
        worktree_status_short = $worktreeStatus
        bundle = [ordered]@{
            zip_path = $zipLeaf
            sha256_sidecar_path = $shaLeaf
        }
        full_check_manifest = [ordered]@{
            path = "release/full_check_manifest.json"
            sha256 = Get-Sha256 (Join-Path $script:stageRoot "release\full_check_manifest.json")
        }
        deterministic_evidence_hashes = [ordered]@{
            path = "release/deterministic_evidence_hashes.json"
            sha256 = Get-Sha256 (Join-Path $script:stageRoot "release\deterministic_evidence_hashes.json")
        }
        release_claim_table = [ordered]@{
            path = "release/release_claim_table.json"
            sha256 = Get-Sha256 (Join-Path $script:stageRoot "release\release_claim_table.json")
        }
        llm_evidence_hashes = [ordered]@{
            path = "release/llm_evidence_hashes.json"
            sha256 = Get-Sha256 (Join-Path $script:stageRoot "release\llm_evidence_hashes.json")
        }
        paper_pdf = [ordered]@{
            path = "paper/main.pdf"
            sha256 = Get-Sha256 (Join-Path $script:stageRoot "paper\main.pdf")
        }
        included_files = $files
        excluded_by_policy = @(
            "config/*.local.yaml",
            "non-v3 execute records",
            "provider response caches",
            "logs",
            "Vivado project output",
            "target directories"
        )
    }
    $artifactManifestPath = Join-Path $script:stageRoot "artifact_manifest.json"
    $artifactManifest | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $artifactManifestPath -Encoding utf8

    $zipPath = Join-Path $outputRoot $zipLeaf
    $shaPath = Join-Path $outputRoot $shaLeaf
    if (Test-Path -LiteralPath $zipPath) {
        Remove-Item -LiteralPath $zipPath -Force
    }
    if (Test-Path -LiteralPath $shaPath) {
        Remove-Item -LiteralPath $shaPath -Force
    }
    Compress-Archive -Path (Join-Path $script:stageRoot "*") -DestinationPath $zipPath -Force
    $bundleHash = Get-Sha256 $zipPath
    "$bundleHash  $(Split-Path -Leaf $zipPath)" | Set-Content -LiteralPath $shaPath -Encoding ascii

    Write-Host "wrote $zipPath"
    Write-Host "wrote $shaPath"
    Write-Host "bundle_sha256=$bundleHash"
    Write-Host "paper_pdf_sha256=$((Get-Sha256 $paperPdfPath))"
} finally {
    Pop-Location
}
