#NoEnv  ; Recommended for performance and compatibility with future AutoHotkey releases.
; #Warn  ; Enable warnings to assist with detecting common errors.
SendMode Input  ; Recommended for new scripts due to its superior speed and reliability.
SetWorkingDir %A_ScriptDir%  ; Ensures a consistent starting directory.

; BASH SHORTCUT CTRL + ALT + T
^!t::
Run,D:\winstoreLinks\Windows Terminal (preview)
Return

; CTRL + ALT + H TOGGLES HIDDEN FILES 
^!h:: 
RegRead, ValorHidden, HKEY_CURRENT_USER, Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced, Hidden
if ValorHidden = 2 
	RegWrite, REG_DWORD, HKEY_CURRENT_USER, Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced, Hidden, 1
Else
	RegWrite, REG_DWORD, HKEY_CURRENT_USER, Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced, Hidden, 2
send {F5}
Return

; CHANGE IP SHORTCUT CTRL + ALT + I
^!i::
Run,D:\Programs\MyIpChanger\MyIpChangerS.lnk
Return

; CREATE THE AWESOME MATH PAD WITH SHIFT
>+PgUp:: send, {(}
Return
>+PgDn:: send, {)}
Return
>+Home:: send, {,}
Return
>+End:: send, {Space}
Return
>+NumpadMult:: send, {^}
Return
>+Backspace:: send, {Tab}
Return
^!F12:: send, {NumLock}
Return

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
Send, {CTRLDOWN}c{CTRLUP}
sleep, 300
Send, {ALTDOWN}{TAB}{ALTUP}
sleep, 300
Send, {CTRLDOWN}v{CTRLUP}
sleep, 300
Send, {TAB}{TAB}
sleep, 300
Send, {ALTDOWN}{TAB}{ALTUP}
return

;Mouse change virtual Desktop
LWin & LButton::
Send, {LWin down}{LCtrl down}{Left}{LWin up}{LCtrl up}
return

LWin & RButton::
Send, {LWin down}{LCtrl down}{Right}{LWin up}{LCtrl up}
return

; KEEP WINDOW ON TOP OF OTHER WINDOWS (LOST IN THE CALAMITY)

; CHECK IF CURRENT SONG IN ITUNES IS CLEAN 
#c::
Run, C:\Users\kailean.okeefe\prog\powershell\GetArtistAndTitle.ps1
Return

; CROSS COPY PASTE TO DENNIS (CTRL + ALT + c / v)
^!c::
Run, C:\Users\kailean.okeefe\prog\python\net_copy\scripts\pythonw.exe C:\Users\kailean.okeefe\prog\python\net_copy\net_copy.py copy
Return
^!v::
Run, C:\Users\kailean.okeefe\prog\python\net_copy\scripts\pythonw.exe C:\Users\kailean.okeefe\prog\python\net_copy\net_copy.py paste
Return

; OVERRIDE THE STUPID TEAMS SHORTCUT!!!
#IfWinActive, ahk_class Chrome_WidgetWin_1 ahk_exe Teams.exe
$^+c::TrayTip "😁", "Saved your life! ;)", 5
Return

; COPY IMAGE AS TEXT (WIN + I)
#i::
Run,C:\Users\kailean.okeefe\prog\python\oc_copy\scripts\pythonw.exe C:\Users\kailean.okeefe\prog\python\oc_copy\oc_copy.py
Return 

;AUTO SETUP FOR WORK CTRL + ALT + W
^!w::
Run,C:\Users\kailean.okeefe\AppData\Local\Microsoft\Teams\Update.exe --processStart "Teams.exe" --process-start-args "--profile=AAD -disable-features=HardwareMediaKeyHandling" (0.03)
Run,C:\Users\kailean.okeefe\AppData\Local\Programs\Microsoft VS Code\Code.exe (0.03)
Run,C:\Program Files\iTunes\iTunes.exe
Run,D:\winstoreLinks\Windows Terminal (preview) (0.08)
Run,C:\Program Files\Mozilla Firefox\firefox.exe -url "https://dev.azure.com/hexagonsf/platform/_git/Nimbus?path=%2F&version=GBmaster&_a=contents" -url "https://hexagonmi.atlassian.net/jira/so
Return (8.03)
