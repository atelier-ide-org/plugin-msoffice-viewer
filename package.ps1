# Builds this plugin, then makes the file Atelier installs: target/<profile>/msoffice-viewer.atelier-plugin
# (the manifest and the library of this system). In Atelier: Plugins, the + button.
param(
    [switch]$Dev
)
$ErrorActionPreference = 'Stop'
Push-Location $PSScriptRoot
try {
    if ($Dev) { cargo build } else { cargo build --release }
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    $version = (Select-String -Path Cargo.toml -Pattern '^version = "(.*)"' | Select-Object -First 1).Matches.Groups[1].Value
} finally {
    Pop-Location
}
$profileDir = if ($Dev) { 'debug' } else { 'release' }
$out = Join-Path $PSScriptRoot "target/$profileDir"
$stage = Join-Path $out 'msoffice-viewer.stage'
Remove-Item -Recurse -Force $stage -ErrorAction SilentlyContinue
$platform = "windows-$($env:PROCESSOR_ARCHITECTURE -replace 'AMD64','x86_64' -replace 'ARM64','aarch64')"
New-Item -ItemType Directory -Force (Join-Path $stage $platform) | Out-Null
$manifest = Get-Content (Join-Path $PSScriptRoot 'plugin.json') -Raw | ConvertFrom-Json
$manifest.version = $version
$manifest | ConvertTo-Json | Set-Content (Join-Path $stage 'plugin.json')
Copy-Item (Join-Path $out 'msoffice_viewer.dll') (Join-Path $stage $platform)
$file = Join-Path $out 'msoffice-viewer.atelier-plugin'
Remove-Item -Force $file -ErrorAction SilentlyContinue
Compress-Archive -Path (Join-Path $stage '*') -DestinationPath "$file.zip"
Move-Item -Force "$file.zip" $file
Remove-Item -Recurse -Force $stage
Write-Host "$file`nIn Atelier: Plugins, Install a plugin from a file (the + button)."
