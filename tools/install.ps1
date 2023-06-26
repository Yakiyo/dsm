#!/usr/bin/env pwsh

# This file is duct taped from https://github.com/denoland/deno_install/blob/master/install.ps1

$ErrorActionPreference = 'Stop'

if ($v) {
    $Version = "v${v}"
}

if ($Args.Length -eq 1) {
  $Version = $Args.Get(0)
}

$InstallDir = $env:DSM_INSTALL

if (!$InstallDir) {
    $InstallDir = "${Home}\.dsm"
}

$Target = 'x86_64-pc-windows-msvc.exe'

$DownloadUrl = if (!$Version) {
  "https://github.com/Yakiyo/dsm/releases/latest/download/dsm-${Target}"
} else {
  "https://github.com/Yakiyo/dsm/releases/download/${Version}/dsm-${Target}"
}

if (!(Test-Path $BinDir)) {
  New-Item $InstallDir -ItemType Directory | Out-Null
}

$Response = Invoke-WebRequest -Uri $DownloadUrl
$Stream = [System.IO.StreamWriter]::new("${InstallDir}\dsm.exe", $false, $Response.Encoding)
try {
    $Stream.Write($Response.Content)
} finally {
    $Stream.Dispose()
}

$User = [System.EnvironmentVariableTarget]::User
$Path = [System.Environment]::GetEnvironmentVariable('Path', $User)
if (!(";${Path};".ToLower() -like "*;${InstallDir};*".ToLower())) {
  [System.Environment]::SetEnvironmentVariable('Path', "${Path};${InstallDir}", $User)
  $Env:Path += ";${InstallDir}"
}


