$ErrorActionPreference = "Stop"

$WixBin = "C:\Program Files (x86)\WiX Toolset v3.14\bin"
$Heat = Join-Path $WixBin "heat.exe"

Write-Host "Building Rust release..."
cargo build --release

Write-Host "Regenerating wix\assets.wxs..."
& $Heat dir "target\release\assets" `
    -cg AssetsComponents `
    -dr AssetsFolder `
    -srd `
    -sfrag `
    -gg `
    -template fragment `
    -var var.CargoTargetBinDir `
    -out "wix\assets.wxs"

Write-Host "Patching generated asset paths..."
$assetsWxs = "wix\assets.wxs"
$content = Get-Content $assetsWxs -Raw

# Heat generates:
#   $(var.CargoTargetBinDir)\icons\...
# but your real files are:
#   $(var.CargoTargetBinDir)\assets\icons\...
$content = $content.Replace('$(var.CargoTargetBinDir)\', '$(var.CargoTargetBinDir)\assets\')

Set-Content $assetsWxs $content -Encoding UTF8

Write-Host "Building MSI..."
cargo wix --bin-path $WixBin --nocapture

Write-Host "Done."