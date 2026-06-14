param(
    [string] $LlmConfig = "config/llm-provider.local.yaml",
    [string] $Profiles = "smoke,low_cost_crosscheck",
    [string] $Baselines = "direct_verilog,sv_interface,mico_source,mico_json_ast,mico_json_ast_repair",
    [string] $ProviderProfile = "smoke",
    [string] $Manifest = "build/release/full_check_manifest.json",
    [switch] $WithLatex
)

$ErrorActionPreference = "Stop"

$repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
$edaDocker = Join-Path $PSScriptRoot "eda-docker.ps1"

Push-Location $repoRoot
try {
    Write-Host "== Host Docker =="
    $dockerVersion = (& docker --version) -join " "
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
    $dockerVersion = $dockerVersion.Trim()
    Write-Host $dockerVersion

    & $edaDocker `
        env "MICO_HOST_DOCKER_VERSION=$dockerVersion" `
        bash scripts/full-check.sh `
        --llm-config $LlmConfig `
        --profiles $Profiles `
        --baselines $Baselines `
        --provider-profile $ProviderProfile `
        --manifest $Manifest
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }

    if ($WithLatex) {
        Write-Host ""
        Write-Host "== Host LaTeX paper build =="
        & latexmk -cd -pdf -interaction=nonstopmode -halt-on-error paper/main.tex
        if ($LASTEXITCODE -ne 0) {
            exit $LASTEXITCODE
        }
    }
} finally {
    Pop-Location
}
