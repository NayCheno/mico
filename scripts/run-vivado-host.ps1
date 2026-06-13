param(
    [Parameter(Mandatory = $true)]
    [string] $Source,

    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]] $VivadoArgs
)

$ErrorActionPreference = "Stop"

$sourcePath = (Resolve-Path -LiteralPath $Source).Path

if ($env:VIVADO_BIN) {
    $vivado = $env:VIVADO_BIN
} else {
    $vivadoCommand = Get-Command vivado -ErrorAction SilentlyContinue
    if (-not $vivadoCommand) {
        throw "Vivado was not found on PATH. Set VIVADO_BIN to the full vivado executable path."
    }
    $vivado = $vivadoCommand.Source
}

& $vivado -mode batch -source $sourcePath @VivadoArgs
exit $LASTEXITCODE
