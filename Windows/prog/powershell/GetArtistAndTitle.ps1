# search for iTunes COM object
# one result will be iTunes.Application.1                      iTunes Class
Get-CimInstance Win32_COMSetting|Select-Object ProgId, Caption|Where-Object Caption -ILike "*itunes*"

# instanciate the object - the .<number> at the end may reference a version
$itunes = New-Object -ComObject iTunes.Application

# play any file from your hard drive
# $itunes.PlayFile "C:\Users\<username>\some_music_file.mp>"

# change the volume by modifying the SoundVolume property of the $iTunes object where 0 is mute and 100 max volume
# $itunes.SoundVolume = 50

# get all properties/methods of the iTunes COM object - Parameter types will be displayed and wether it only returns
# a value (get;), sets one (set;) or does both (get;set;) 
#$iTunes|Get-Member

$currentArtist = $iTunes.CurrentTrack.Artist
$currentTrack = $iTunes.CurrentTrack.Name

$resp = curl.exe '-X', 'POST', 'http://192.168.1.15:22555/clean', '--data', "artist=$($currentArtist)", '--data', "title=$($currentTrack)"

New-BurntToastNotification -Text "Dennis: Song Results", "$resp"
