# Build the NextGuide NSIS installer from dist\windows-bundle.
# Prerequisites: assemble-bundle.sh already run in MSYS2; NSIS (makensis) on PATH.
param(
    [string]$BundleDir = "",
    [string]$Version = ""
)

$ErrorActionPreference = "Stop"
$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
Set-Location $RepoRoot

if (-not $BundleDir) {
    $BundleDir = Join-Path $RepoRoot "dist\windows-bundle"
}
if (-not $Version) {
    $toml = Get-Content (Join-Path $RepoRoot "crates\nextguide\Cargo.toml")
    $Version = ($toml | Select-String -Pattern '^version = "(.+)"' | Select-Object -First 1).Matches.Groups[1].Value
}

if (-not (Test-Path (Join-Path $BundleDir "nextguide.exe"))) {
    throw "Missing $BundleDir\nextguide.exe — run packaging/windows/assemble-bundle.sh in MSYS2 UCRT64 first."
}
if (-not (Test-Path (Join-Path $BundleDir "nextguide-cli.exe"))) {
    throw "Missing $BundleDir\nextguide-cli.exe — assemble-bundle.sh must include nextguide-cli."
}

$makensis = Get-Command makensis -ErrorAction SilentlyContinue
if (-not $makensis) {
    $candidates = @(
        "${env:ProgramFiles(x86)}\NSIS\makensis.exe",
        "${env:ProgramFiles}\NSIS\makensis.exe"
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) { $makensis = Get-Command $c; break }
    }
}
if (-not $makensis) {
    throw "makensis not found. Install NSIS from https://nsis.sourceforge.io/ and re-run."
}

New-Item -ItemType Directory -Force -Path (Join-Path $RepoRoot "dist") | Out-Null

$nsi = Join-Path $RepoRoot "packaging\windows\nextguide.nsi"
$bundleAbs = (Resolve-Path $BundleDir).Path
$setup = Join-Path $RepoRoot "dist\NextGuide-$Version-windows-x64-setup.exe"
& $makensis.Source `
  "/DPRODUCT_VERSION=$Version" `
  "/DBUNDLE_DIR=$bundleAbs" `
  "/DOUT_FILE=$setup" `
  $nsi
if ($LASTEXITCODE -ne 0) { throw "makensis failed with exit $LASTEXITCODE" }
if (-not (Test-Path $setup)) {
    throw "Expected installer missing: $setup"
}

if ($env:SIGNTOOL_ARGS) {
    Write-Host "==> Authenticode signing (SIGNTOOL_ARGS set)"
    $signtool = Get-Command signtool -ErrorAction SilentlyContinue
    if (-not $signtool) {
        throw "signtool not on PATH but SIGNTOOL_ARGS is set"
    }
    # SIGNTOOL_ARGS is appended after the file path args convention: signtool sign <args> file
    Invoke-Expression "signtool sign $env:SIGNTOOL_ARGS `"$setup`""
}

Write-Host "Built $setup"
Write-Host "Includes nextguide.exe + bundled GTK + nextguide-cli.exe"
Write-Host "Signing optional — leave SIGNTOOL_ARGS unset for unsigned early releases."
