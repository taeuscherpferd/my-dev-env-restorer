Set-Alias -Name o -Value open-cur-dir
Set-Alias -Name WN -Value nav-to-nimbus

function open-cur-dir {
    ii .
}

function nav-to-nimbus {
    Set-Location -Path "D:\wprojects\nimbus\Platform\ui"
}
