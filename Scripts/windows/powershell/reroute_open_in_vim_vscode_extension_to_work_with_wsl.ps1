param([string]$path)

# Convert the Windows-style path to WSL-style
$wslPath = $path 

# Call WSL with the corrected path and open it in vim
wsl $wslPath

