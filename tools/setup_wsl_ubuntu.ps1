# Rustyssh
# 07/11/2022, sganis
# Install WSL with ubuntu2004, ssh server and support user
# For testing purposes
# It works for new development machines and also in Appveyor
#
# Run scripts in powershell:
# set-executionpolicy remotesigned
#
# Uninstall:
# wslconfig /u Ubuntu-18.04

Write-host "Checking if WSL feature is installed..."
$installed = (Get-WindowsOptionalFeature -FeatureName Microsoft-Windows-Subsystem-Linux -Online).State -eq 'Enabled'
if ($installed) {
    Write-host "WSL feature is installed"
} else {   
    Write-error "WSL feature is not installed, installing..."
    Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux
}

# $name = "Ubuntu1804"
# $zip = "C:\cache\$name.tar.gz"
# $folder = "C:\MyWSL\$name"
# $url = "https://cloud-images.ubuntu.com/releases/bionic/release/ubuntu-18.04-server-cloudimg-amd64-wsl.rootfs.tar.gz"

# $name = "Ubuntu2004"
# $zip = "C:\cache\$name.tar.gz"
# $folder = "C:\MyWSL\$name"
# $url = "https://cloud-images.ubuntu.com/releases/focal/release/ubuntu-20.04-server-cloudimg-amd64-wsl.rootfs.tar.gz"

$name = "Ubuntu2204"
$zip = "C:\cache\$name.tar.gz"
$folder = "C:\MyWSL\$name"
$url = "https://cloud-images.ubuntu.com/wsl/jammy/current/ubuntu-jammy-wsl-amd64-wsl.rootfs.tar.gz"


New-Item -ItemType Directory -Force -Path C:\MyWSL

if (!(Test-Path $folder)) {
    Write-host "Installing Ubuntu for WSL"
    if (!(Test-Path $zip)) {
        Write-host "Downloading..."
        [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
        (New-Object Net.WebClient).DownloadFile("$url", "$zip")
    } else {
        Write-host "Downloaded already, found in cache..."
    }
    Write-host "Installing..."

    # rootfs image
    mkdir $folder
    wsl.exe --import $name $folder $zip
    # appx image
    # Expand-Archive -Path "$zip" -DestinationPath "$folder" -Force
    # . $exe install --root\
   
}
Write-host "Setting $name as default distro..."
wsl.exe -s $name

Write-host "Creating support user..."
wsl.exe -- adduser support --gecos `"First,Last,RoomNumber,WorkPhone,HomePhone`" --disabled-password
wsl.exe -- echo `'support:support`' `| chpasswd
wsl.exe -- usermod -aG sudo support
wsl.exe -- echo -e `"support\tALL=`(ALL`)\tNOPASSWD: ALL`" `> /etc/sudoers.d/support 2`>/dev/null
wsl.exe -- chmod 0755 /etc/sudoers.d/support


# Write-host "Updating..."
# wsl.exe -e apt-get update
# Write-host "Checing user..."
# wsl.exe -e whoami

Write-host "Installing ssh..."
wsl.exe -- apt-get update
wsl.exe -- apt-get remove -y -qq --purge openssh-server `> /dev/null
wsl.exe -- apt-get install -y -qq openssh-server `> /dev/null
wsl.exe -- service ssh --full-restart

# Write-Host "Export only available from Version 10.0.20363.720"
# $host
# Write-host "Exporting..."
# wsl --export Ubuntu-20.04 $tar

# tauri prerequisites
# apt install -y libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# nodejs:
# curl -fsSL https://deb.nodesource.com/setup_current.x | sudo -E bash - 
# apt-get install -y nodejs
# npm i -g yarn

# rust
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
