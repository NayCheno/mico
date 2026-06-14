param(
    [Parameter(Mandatory = $true)]
    [string] $Source,

    [string] $ReportDir = "build/reports/vivado-host",

    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]] $VivadoArgs
)

$ErrorActionPreference = "Stop"

$repoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
$sourcePath = (Resolve-Path -LiteralPath $Source).Path
$allowedVivadoRootLiteral = "D:\Application\vivado\2025.2\Vivado"
$allowedVivadoRoot = (Resolve-Path -LiteralPath $allowedVivadoRootLiteral).Path
$defaultVivado = Join-Path $allowedVivadoRoot "bin\vivado.bat"

if ($env:VIVADO_BIN) {
    $vivado = (Resolve-Path -LiteralPath $env:VIVADO_BIN).Path
} else {
    $vivado = $defaultVivado
}

if (-not (Test-Path -LiteralPath $vivado)) {
    throw "Vivado executable was not found at required path: $defaultVivado"
}

if (-not $vivado.StartsWith($allowedVivadoRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
    throw "Vivado path '$vivado' is outside the required root '$allowedVivadoRoot'."
}

if ([System.IO.Path]::IsPathRooted($ReportDir)) {
    $reportPath = $ReportDir
} else {
    $reportPath = Join-Path $repoRoot $ReportDir
}
$reportPath = New-Item -ItemType Directory -Force -Path $reportPath
$reportPath = $reportPath.FullName
$env:MICO_VIVADO_REPORT_DIR = $reportPath

$journal = Join-Path $reportPath "vivado.jou"
$log = Join-Path $reportPath "vivado.log"

Push-Location $reportPath
try {
    & $vivado -mode batch -journal $journal -log $log -source $sourcePath @VivadoArgs
    $vivadoExitCode = $LASTEXITCODE
} finally {
    Pop-Location
}
exit $vivadoExitCode
