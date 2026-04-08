Set-Alias -Name o -Value open-cur-dir
Set-Alias -Name WN -Value nav-to-nimbus
Set-Alias -Name WD -Value  nav-to-work-projects 

function open-cur-dir {
    ii .
}

function nav-to-nimbus {
    Set-Location -Path "D:\wprojects\nimbus\ui"
}

function nav-to-work-projects {
    Set-Location -Path "D:\wprojects"
}

function npm-clean {
  rm -r .\node_modules\, .\package-lock.json;
  npm i --legacy-peer-deps;
  npm audit fix --legacy-peer-deps;
  npm start;
}

function mm {
  git pull --rebase;
}

function beep {
  [console]::beep(500,300)
}

function npmi($a) {
  npm i $a; npm i -D @types/$a;
}

function npmd($a) {
  npm i -D $a;
}

# Chocolatey profile
$ChocolateyProfile = "$env:ChocolateyInstall\helpers\chocolateyProfile.psm1"
if (Test-Path($ChocolateyProfile)) {
  Import-Module "$ChocolateyProfile"
}
