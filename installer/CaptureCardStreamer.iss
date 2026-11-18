; Inno Setup script for CaptureCardStreamer (based on vicash by caaatto, MIT).
; Build with installer\build-installer.ps1, or manually:
;   cargo build --release
;   ISCC.exe installer\CaptureCardStreamer.iss
; Output: target\installer\CaptureCardStreamer-Setup-<version>.exe

#define AppName "CaptureCardStreamer"
#define AppVersion "0.1.14"
#define AppExe "CaptureCardStreamer.exe"

[Setup]
; Fixed GUID identifies the app for upgrades and uninstall. Never change it.
AppId={{85DF78ED-2AED-44D1-BA8C-94ECF0B39F9D}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppPublisher=caaatto
AppPublisherURL=https://github.com/caaatto/vicash
; Per-user install, no admin rights needed. Installs to
; %LOCALAPPDATA%\Programs\CaptureCardStreamer, which also lets the built-in
; self-updater replace the exe without elevation.
PrivilegesRequired=lowest
DefaultDirName={autopf}\{#AppName}
DisableProgramGroupPage=yes
UninstallDisplayIcon={app}\{#AppExe}
UninstallDisplayName={#AppName}
OutputDir=..\target\installer
OutputBaseFilename={#AppName}-Setup-{#AppVersion}
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0
; Close a running instance before overwriting the exe on reinstall/upgrade.
CloseApplications=yes
RestartApplications=no

[Languages]
Name: "german"; MessagesFile: "compiler:Languages\German.isl"
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"

[Files]
; The cargo binary is called vicash.exe; it is installed under the new name.
Source: "..\target\release\vicash.exe"; DestDir: "{app}"; DestName: "{#AppExe}"; Flags: ignoreversion
Source: "..\LICENSE*"; DestDir: "{app}"; Flags: ignoreversion skipifsourcedoesntexist
Source: "..\README.md"; DestDir: "{app}"; Flags: ignoreversion skipifsourcedoesntexist

[Icons]
Name: "{autoprograms}\{#AppName}"; Filename: "{app}\{#AppExe}"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExe}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#AppExe}"; Description: "{cm:LaunchProgram,{#AppName}}"; Flags: nowait postinstall skipifsilent

[UninstallDelete]
; Leftover from the self-updater's rename trick.
Type: files; Name: "{app}\vicash.exe.old"
