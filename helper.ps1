$ErrorActionPreference = 'Stop'

function Invoke-CargoTest {
    Push-Location $PSScriptRoot
    cargo test --package finished
    Pop-Location
}

$LibRS = "$PSScriptRoot\src\lib.rs"

if (!((Get-Content -First 1 $LibRS) -match '(?:pub\s+)?mod\s+(\S+)\s*;'))
{
    Write-Host "没有找到可移动的模块"
    Invoke-CargoTest
    exit
}
$ModuleName = $Matches[1]

$LibRSContent = Get-Content $LibRS
if ($LibRSContent.Count -eq 1)
{
    Clear-Content $LibRS
}
else
{
    $LibRSContent[1..($LibRSContent.Count - 1)] | Set-Content -Path $LibRS
}

Move-Item -Path "$PSScriptRoot\src\$ModuleName.rs" -Destination "$PSScriptRoot\finished\src\$ModuleName.rs"

$FinishedLibRS = "$PSScriptRoot\finished\src\lib.rs"
$Modules = Get-Content $FinishedLibRS
$Modules += "pub mod $ModuleName;"
$Modules | Sort-Object | Set-Content $FinishedLibRS

Invoke-CargoTest
