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

function Resolve-RepoPath {
    param([string] $Path)
    if ([System.IO.Path]::IsPathRooted($Path)) {
        return $Path
    }
    return (Join-Path $repoRoot $Path)
}

function Get-Sha256 {
    param([string] $Path)
    return (Get-FileHash -Algorithm SHA256 -LiteralPath $Path).Hash.ToLowerInvariant()
}

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

        $manifestPath = Resolve-RepoPath $Manifest
        $paperPdfPath = Resolve-RepoPath "paper/main.pdf"
        if ((Test-Path -LiteralPath $manifestPath -PathType Leaf) -and (Test-Path -LiteralPath $paperPdfPath -PathType Leaf)) {
            Write-Host ""
            Write-Host "== Update release manifest paper hash =="
            $manifestJson = Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json
            $paperItem = Get-Item -LiteralPath $paperPdfPath
            $latexmkVersion = ((& latexmk -version 2>$null) | Where-Object { $_ -match "Latexmk" } | Select-Object -First 1)
            $manifestJson | Add-Member -MemberType NoteProperty -Name paper_pdf -Value ([pscustomobject]@{
                path = "paper/main.pdf"
                sha256 = Get-Sha256 $paperPdfPath
                bytes = $paperItem.Length
            }) -Force
            $manifestJson | Add-Member -MemberType NoteProperty -Name host_latex -Value ([pscustomobject]@{
                latexmk = $latexmkVersion
            }) -Force
            $manifestJson | ConvertTo-Json -Depth 12 | Set-Content -LiteralPath $manifestPath -Encoding utf8
            Write-Host "paper_pdf_sha256=$($manifestJson.paper_pdf.sha256)"
        }
    }
} finally {
    Pop-Location
}
