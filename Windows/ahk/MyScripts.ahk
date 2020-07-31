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

;COPY AND PASTE FROM ONE APP TO ANOTHER! COMMAND + C
#c:: 
Send, {CTRLDOWN}c{CTRLUP}
sleep, 300
Send, {ALTDOWN}{TAB}{ALTUP}
sleep, 300
Send, {CTRLDOWN}v{CTRLUP}
sleep, 300
Send, {Enter}
sleep, 300
Send, {ALTDOWN}{TAB}{ALTUP}
return

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


;AUTO SETUP FOR WORK CTRL + ALT + W
^!w::
;The commented out code is for if I want to login with having to type credentials
;Loginname = Your Name
;Password = Your Password
Run,C:\Users\kailean.okeefe\AppData\Local\SourceTree\SourceTree.exe
Run,D:\Programs\Fortinet\FortiClient.exe
Run,C:\Program Files (x86)\Microsoft\Skype for Desktop\Skype.exe
Run,C:\Users\kailean.okeefe\AppData\Local\Microsoft\Teams

RunAs[,kailean.okeefe, TRLvrm7966]
Run,C:\Program Files (x86)\Microsoft Visual Studio\2019\Professional\Common7\IDE\devenv.exe 
RunAs

URL = https://secure.saashr.com/ta/6123079.admin?rnd=XFP&showAdmin=1&Ext=login&sft=ARJLLOVBJV
WB := ComObjCreate("InternetExplorer.Application")
WB.Visible := True
WB.Navigate(URL)
While wb.readyState != 4 || wb.document.readyState != "complete" || wb.busy ; wait for the page to load
   Sleep, 10
;wb.document.getElementById("ctl00_mainContentPlaceHolder_loginUserControl_userLoginTextBox").value := Loginname
;wb.document.getElementById("ctl00_mainContentPlaceHolder_loginUserControl_passwordTextBox").value := Password
;wb.document.getElementById("ctl00_mainContentPlaceHolder_loginUserControl_submitButton").click()
;While wb.readyState != 4 || wb.document.readyState != "complete" || wb.busy ; wait for the page to load
;   Sleep, 10
;Msgbox, Now logged in and loaded!
return
