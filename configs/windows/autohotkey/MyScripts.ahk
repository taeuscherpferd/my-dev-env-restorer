; V1toV2: Removed #NoEnv  ; Recommended for performance and compatibility with future AutoHotkey releases.
; #Warn  ; Enable warnings to assist with detecting common errors.
SendMode("Input")  ; Recommended for new scripts due to its superior speed and reliability.
SetWorkingDir(A_ScriptDir)  ; Ensures a consistent starting directory.

; BASH SHORTCUT CTRL + ALT + T
^!t::
{ ; V1toV2: Added bracket
    global ; V1toV2: Made function global
    Run("D:\winstoreLinks\Windows Terminal (preview)")
    return
} ; V1toV2: Added Bracket before hotkey or Hotstring

; CTRL + ALT + H TOGGLES HIDDEN FILES
#HotIf WinActive("ahk_exe explorer.exe")
^!h::
{ ; V1toV2: Added bracket
    global ; V1toV2: Made function global
    ValorHidden := RegRead("HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced", "Hidden")
    if (ValorHidden = 2)
        RegWrite(1, "REG_DWORD", "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "Hidden")
    else
        RegWrite(2, "REG_DWORD", "HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
            "Hidden")
    Send("{F5}")
    return
} ; V1toV2: Added Bracket before hotkey or Hotstring
#HotIf

; CHANGE IP SHORTCUT WIN + CTRL + ALT + I
#^!i::
{ ; V1toV2: Added bracket
    global ; V1toV2: Made function global
    Run("D:\Programs\MyIpChanger\MyIpChangerS.lnk")
    return
} ; V1toV2: Added Bracket before hotkey or Hotstring

; CREATE THE AWESOME MATH PAD WITH SHIFT
>+PgUp:: Send("{(}")
{
    return
}
>+PgDn:: Send("{)}")
{
    return
}
>+Home:: Send("{,}")
{
    return
}
>+End:: Send("{Space}")
{
    return
}
>+NumpadMult:: Send("{^}")
{
    return
}
>+Backspace:: Send("{Tab}")
{
    return
}
^!F12:: Send("{NumLock}")
{
    return
}

;COPY AND PASTE FROM ONE APP TO ANOTHER! COMMAND + C (Commented out in favor for checking if songs are clean)
;#c::
;Send, {CTRLDOWN}c{CTRLUP}
;sleep, 300
;Send, {ALTDOWN}{TAB}{ALTUP}
;sleep, 300
;Send, {CTRLDOWN}v{CTRLUP}
;sleep, 300
;Send, {Enter}
;sleep, 300
;Send, {ALTDOWN}{TAB}{ALTUP}
;return

;QUIZLET CODE COMMAND + Q
#q::
{ ; V1toV2: Added bracket
    global ; V1toV2: Made function global
    Send("{CTRLDOWN}c{CTRLUP}")
    Sleep(300)
    Send("{ALTDOWN}{TAB}{ALTUP}")
    Sleep(300)
    Send("{CTRLDOWN}v{CTRLUP}")
    Sleep(300)
    Send("{TAB}{TAB}")
    Sleep(300)
    Send("{ALTDOWN}{TAB}{ALTUP}")
    return
} ; V1toV2: Added Bracket before hotkey or Hotstring

;Mouse change virtual Desktop
#LButton::
{
    global
    Send("{LWin down}{LCtrl down}{Left}{LWin up}{LCtrl up}")
    return
}

#RButton::
{
    global
    Send("{LWin down}{LCtrl down}{Right}{LWin up}{LCtrl up}")
    return
}

;COPY IMAGE AS TEXT (WIN + I)
#i::
{ ; V1toV2: Added bracket
    global ; V1toV2: Made function global
    Run(
        "C:\Users\kailean.okeefe\prog\python\oc_copy\scripts\pythonw.exe C:\Users\kailean.okeefe\prog\python\oc_copy\oc_copy.py"
    )
    return
} ; V1toV2: Added Bracket before hotkey or Hotstring

;CHECK IF CURRENT SONG IN ITUNES IS CLEAN
#c::
{ ; V1toV2: Added bracket
    global ; V1toV2: Made function global
    Run(
        "PowerShell.exe -ExecutionPolicy Bypass -WindowStyle Hidden -Command C:\Users\kailean.okeefe\prog\powershell\GetArtistAndTitle.ps1"
    )
    return
} ; V1toV2: Added Bracket before hotkey or Hotstring

;CROSS COPY PASTE TO DENNIS (WIN + CTRL + ALT + c / v)
#^!c::
{ ; V1toV2: Added bracket
    global ; V1toV2: Made function global
    Run(
        "C:\Users\kailean.okeefe\prog\python\net_copy\scripts\pythonw.exe C:\Users\kailean.okeefe\prog\python\net_copy\net_copy.py copy"
    )
    return
} ; V1toV2: Added Bracket before hotkey or Hotstring

#^!v::
{ ; V1toV2: Added bracket
    global ; V1toV2: Made function global
    Run(
        "C:\Users\kailean.okeefe\prog\python\net_copy\scripts\pythonw.exe C:\Users\kailean.okeefe\prog\python\net_copy\net_copy.py paste"
    )
    return
} ; V1toV2: Added Bracket before hotkey or Hotstring

;AUTO ENTER DEFAULT SUBTASKS (CTRL + SHIFT + ALT + S)
$^!+s::
{ ; V1toV2: Added bracket
    global ; V1toV2: Made function global
    Send("Coding")
    Sleep(200)
    Send("{Enter}")
    Sleep(500)
    Send("Merge")
    Sleep(200)
    Send("{Enter}")
    Sleep(500)
    Send("Test")
    Sleep(200)
    Send("{Enter}")
    return
} ; V1toV2: Added Bracket before hotkey or Hotstring

;AUTO SETUP FOR WORK WIN + CTRL + ALT + W
#^!w::
{ ; V1toV2: Added bracket
    global ; V1toV2: Made function global
    Run(
        "C:\Users\kailean.okeefe\AppData\Local\Microsoft\Teams\Update.exe --processStart `"Teams.exe`" --process-start-args `"--profile=AAD -disable-features=HardwareMediaKeyHandling`""
    )
    Run("C:\Users\kailean.okeefe\AppData\Local\Programs\Microsoft VS Code\Code.exe")
    Run("C:\Program Files\iTunes\iTunes.exe")
    Run("D:\winstoreLinks\Windows Terminal (preview)")
    Run(
        "C:\Program Files\Mozilla Firefox\firefox.exe -url `"https://dev.azure.com/hexagonsf/platform/_git/Nimbus?path=`%2F&version=GBmaster&_a=contents`" -url `"https://hexagonmi.atlassian.net/jira/software/c/projects/GEN/boards/1605?quickFilter=22255`" - url `"https://metrologyreporting.dev.hexagonsfx.com/home?daterange=all`";"
    )
    return
} ; V1toV2: Added bracket in the end

;OVERRIDE THE STUPID TEAMS SHORTCUT
#HotIf WinActive("ahk_class Chrome_WidgetWin_1 ahk_exe Teams.exe",)
$^+c:: TrayTip("`"😁`"", "`"Saved your life!") ;)", 5
return
#HotIf
