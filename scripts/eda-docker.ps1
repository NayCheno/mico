param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]] $Command
)

$ErrorActionPreference = "Stop"

$repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
$image = if ($env:MICO_EDA_IMAGE) { $env:MICO_EDA_IMAGE } else { "mico-eda:ubuntu24.04" }
$cargoRegistryVolume = if ($env:MICO_CARGO_REGISTRY_VOLUME) { $env:MICO_CARGO_REGISTRY_VOLUME } else { "mico-cargo-registry" }
$cargoGitVolume = if ($env:MICO_CARGO_GIT_VOLUME) { $env:MICO_CARGO_GIT_VOLUME } else { "mico-cargo-git" }

docker image inspect $image *> $null
if ($LASTEXITCODE -ne 0 -or $env:MICO_EDA_REBUILD -eq "1") {
    docker build -f (Join-Path $repoRoot "docker/eda/Dockerfile") -t $image $repoRoot
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}

if (-not $Command -or $Command.Count -eq 0) {
    $Command = @("bash")
}

$ttyArgs = @("-i")
if (-not [Console]::IsInputRedirected -and -not [Console]::IsOutputRedirected) {
    $ttyArgs = @("-it")
}

docker run --rm @ttyArgs `
    -v "${repoRoot}:/workspace" `
    -v "${cargoRegistryVolume}:/opt/rust/cargo/registry" `
    -v "${cargoGitVolume}:/opt/rust/cargo/git" `
    -w /workspace `
    $image `
    @Command
exit $LASTEXITCODE
