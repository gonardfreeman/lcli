# install.ps1 - install lcli on Windows

# Set default install directory
$installDir = "$env:USERPROFILE\bin"
if (!(Test-Path $installDir)) { New-Item -ItemType Directory -Path $installDir }

# Detect latest release from GitHub
$repo = "gonardfreeman/lcli"
$apiUrl = "https://api.github.com/repos/$repo/releases/latest"
$response = Invoke-RestMethod -Uri $apiUrl

# Find Windows x86_64 asset
$asset = $response.assets | Where-Object { $_.name -like "*windows*" }
if ($null -eq $asset) { Write-Error "No Windows asset found"; exit 1 }

# Download
$zipPath = "$env:TEMP\$($asset.name)"
Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $zipPath

# Extract (assumes .zip)
Add-Type -AssemblyName System.IO.Compression.FileSystem
[System.IO.Compression.ZipFile]::ExtractToDirectory($zipPath, $installDir)

# Add to PATH (session only)
$env:PATH += ";$installDir"

Write-Host "lcli installed to $installDir. Add this directory to your PATH to use globally."
